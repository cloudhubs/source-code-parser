use crate::java::method_body::log_unknown_tag;
use crate::java::method_body::node::parse_child_nodes;
use crate::java::method_body::parse_block;
use crate::java::util::parameter::parse_method_parameters;
use crate::java::util::vartype::parse_type_args;

use crate::ast::*;
use crate::ComponentInfo;
use crate::AST;

pub(crate) fn parse_expr(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    match &*ast.r#type {
        // Variables an initialization
        "variable_declarator" | "assignment_expression" => parse_assignment(ast, component),
        "identifier" => {
            let ident: Expr = Ident::new(ast.value.clone()).into();
            Some(ident.into())
        }
        "field_access" => parse_field_access(ast, component),
        "decimal_integer_literal"
        | "decimal_floating_point_literal"
        | "string_literal"
        | "false"
        | "true" => Some(Expr::Literal(ast.value.clone().into())),
        "object_creation_expression" => Some(parse_object_creation(ast, component)),
        "array_creation_expression" => {
            let ident: Expr = Ident::new("AN_ARRAY_HANDLE".into()).into();
            Some(ident.into())
        }

        // Language statements
        "method_invocation" => parse_method(ast, component).into(),
        "lambda_expression" => parse_lambda(ast, component),

        // Base case
        unknown => {
            log_unknown_tag(unknown, "expressions");
            None
        }
    }
}

fn parse_method(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let lhs: Expr = match parse_expr(&ast.children[0], component) {
        Some(opt) => opt,
        None => Literal::new("this".to_string()).into(),
    };

    let mut name: Option<Expr> = None;
    let mut generic: String = String::new();
    let mut args: Vec<Expr> = vec![];

    for comp in ast.children.iter() {
        match &*comp.r#type {
            "type_arguments" => generic = parse_type_args(ast),
            "argument_list" => {
                args.append(
                    &mut comp
                        .children
                        .iter()
                        .flat_map(|e| parse_expr(e, component))
                        .collect::<Vec<Expr>>(),
                );
            }
            "identifier" => {
                let mut result = generic.clone();
                result.push_str(&*comp.value.clone());
                name = Some(Literal::new(result).into());
            }
            unknown => log_unknown_tag(unknown, "method_invoke"),
        }
    }

    // TODO add in generic
    Some(
        Expr::CallExpr(CallExpr::new(
            Box::new(
                DotExpr::new(
                    Box::new(lhs),
                    Box::new(name.expect("method with no name requested!")).into(),
                )
                .into(),
            ),
            args,
        ))
        .into(),
    )
}

fn parse_field_access(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let lhs = parse_expr(&ast.children[0], component)?;
    let rhs = parse_expr(&ast.children[2], component)?;
    Some(Expr::DotExpr(DotExpr::new(Box::new(lhs), Box::new(rhs))))
}

/// Parse an assignment expression. May contain a variable declaration
pub(crate) fn parse_assignment(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    // Define attributes
    let mut name = "";
    let mut rhs = None;

    // Find values
    for node in ast.children.iter() {
        match &*node.r#type {
            "identifier" => name = &*node.value,
            "=" => {}
            unknown => {
                rhs = if let Some(parsed_rhs) = parse_expr(node, component) {
                    Some(parsed_rhs)
                } else {
                    log_unknown_tag(unknown, "parse_assignment");
                    None
                }
            }
        }
    }

    // Assemble
    let lhs = Ident::new(name.into());
    if let Some(rhs) = rhs {
        let bin: Expr = BinaryExpr::new(Box::new(lhs.into()), "=".into(), Box::new(rhs)).into();
        Some(bin.into())
    } else {
        Some(lhs.into())
    }
}

/// Parse instantiation using 'new'
fn parse_object_creation(ast: &AST, component: &ComponentInfo) -> Expr {
    let mut name = String::new();
    let mut arg_list = vec![];
    for child in ast.children.iter() {
        match &*child.r#type {
            "type_identifier" => name = child.value.clone(),
            "argument_list" => {
                arg_list = parse_child_nodes(child, component)
                    .into_iter()
                    .map(|node| match node {
                        Node::Expr(expr) => Some(expr),
                        _ => None,
                    })
                    .flat_map(|expr| expr)
                    .collect()
            }
            unknown => {
                log_unknown_tag(unknown, "parse_object_creation");
            }
        }
    }

    // Create ident
    let ident: Expr = CallExpr::new(Box::new(Ident::new(name).into()), arg_list).into();
    ident.into()
}

pub(crate) fn parse_lambda(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let mut params = vec![];
    let mut body = None;

    for child in ast.children.iter() {
        match &*child.r#type {
            "identifier" => params.push(new_simple_param(child)),
            "inferred_parameters" => {
                params = vec![
                    params,
                    child
                        .children
                        .iter()
                        .filter(|p| p.r#type == "identifier")
                        .map(|p| new_simple_param(p))
                        .collect(),
                ]
                .concat();
            }
            "formal_parameters" => {
                params = vec![
                    params,
                    parse_method_parameters(child, component)
                        .into_iter()
                        .map(|p| {
                            let mut decl =
                                VarDecl::new(Some(p.r#type), Ident::new(p.parameter_name));
                            if let Some(annotation) = p.annotation {
                                decl.annotation = annotation;
                            } else {
                                decl.annotation = vec![];
                            }
                            decl
                        })
                        .map(|p| DeclStmt::new(vec![p], vec![]))
                        .collect(),
                ]
                .concat();
            }
            "->" => { /* Ignore the boilerplate */ }
            _ => body = Some(parse_block(child, component)),
        }
    }

    Some(LambdaExpr::new(params, body?).into())
}

fn new_simple_param(ast: &AST) -> DeclStmt {
    DeclStmt::new(
        vec![VarDecl::new(None, Ident::new(ast.value.clone()))],
        vec![],
    )
}
