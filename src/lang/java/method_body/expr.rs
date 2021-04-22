use crate::java::method_body::node::{parse_child_nodes, parse_node};
use crate::java::method_body::parse_block;
use crate::java::util::parameter::parse_method_parameters;
use crate::java::util::vartype::parse_type_args;
use crate::java::{method_body::log_unknown_tag, util::vartype::find_type};

use crate::ast::*;
use crate::ComponentInfo;
use crate::AST;

use super::is_common_junk_tag;

pub(crate) fn parse_expr(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    match &*ast.r#type {
        // Variables an initialization
        "variable_declarator" | "assignment_expression" => parse_assignment(ast, component),
        "identifier" => parse_ident(ast, component),
        "field_access" => parse_field_access(ast, component),
        "decimal_integer_literal"
        | "hex_integer_literal"
        | "binary_integer_literal"
        | "decimal_floating_point_literal"
        | "string_literal"
        | "false"
        | "true" => Some(Expr::Literal(ast.value.clone().into())),
        "object_creation_expression" => Some(parse_object_creation(ast, component)),
        "array_creation_expression" => Some(parse_array_creation(ast, component)),
        "array_initializer" => Some(parse_array_init(ast, component)),
        "array_access" => parse_array_access(ast, component),

        // Language statements
        "method_invocation" => parse_method(ast, component).into(),
        "lambda_expression" => parse_lambda(ast, component),
        "switch_statement" => parse_switch(ast, component),
        "parenthesized_expression" => parse_expr(&ast.children[1], component),
        "ternary_expression" => parse_ternary(ast, component.into()),
        "binary_expression" => parse_binary(ast, component),
        "update_expression" => parse_inc_dec(ast, component),
        "unary_expression" => parse_unary(ast, component),

        // Base case
        unknown => {
            log_unknown_tag(unknown, "expressions");
            None
        }
    }
}

fn parse_ident(ast: &AST, _component: &ComponentInfo) -> Option<Expr> {
    let ident: Expr = Ident::new(ast.value.clone()).into();
    Some(ident.into())
}

fn parse_method(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    // Get calleee
    let lhs: Expr;
    if ast.find_child_by_type(&["."]).is_some() {
        lhs = match parse_expr(&ast.children[0], component) {
            Some(opt) => opt,
            None => Literal::new("this".to_string()).into(),
        };
    } else {
        lhs = Literal::new("this".to_string()).into();
    }

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
    let mut lhs = None;
    let mut rhs = None;

    // Find values
    for node in ast.children.iter() {
        let unknown = &*node.r#type;
        if unknown == "=" {
            continue;
        }

        let result = parse_expr(node, component);
        if result.is_some() {
            if lhs.is_none() {
                lhs = result;
            } else if rhs.is_none() {
                rhs = result;
            } else {
                eprintln!(
                    "Extra parsable tag {} encountered while parsing assignment",
                    unknown
                );
            }
        } else {
            log_unknown_tag(unknown, "parse_assignment");
        }
    }

    // Assemble
    if let Some(lhs) = lhs {
        if let Some(rhs) = rhs {
            let bin: Expr = BinaryExpr::new(Box::new(lhs.into()), "=".into(), Box::new(rhs)).into();
            Some(bin.into())
        } else {
            Some(lhs.into())
        }
    } else {
        eprintln!("Assignment with no lefthand side!");
        None
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

fn parse_array_creation(ast: &AST, component: &ComponentInfo) -> Expr {
    if let Some(init) = ast.find_child_by_type(&["array_initializer"]) {
        // We've got an initializer list, don't mess with the cuckoo-bananas Treesitter format
        parse_array_init(init, component)
    } else {
        // Index-style declaration
        let indexes: Vec<Expr> = ast
            .find_all_children_by_type(&["dimensions_expr"])
            .expect("Array without dimensions!")
            .iter()
            .flat_map(|ndx| parse_expr(&ndx.children[1], component))
            .collect();

        // Recursively compose indexing
        let mut expr = Expr::Ident(Ident::new(find_type(ast)));
        for ndx in indexes {
            expr = Expr::IndexExpr(IndexExpr::new(Box::new(expr), Box::new(ndx)));
        }
        expr.into()
    }
}

fn parse_array_init(ast: &AST, component: &ComponentInfo) -> Expr {
    let mut contents = vec![];
    for init_val in ast.children.iter() {
        if let Some(expr) = parse_expr(init_val, component) {
            contents.push(expr);
        }
    }
    InitListExpr::new(contents).into()
}

fn parse_array_access(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let ident = parse_expr(&ast.children[0], component);
    let index = parse_expr(&ast.children[2], component);
    Some(IndexExpr::new(Box::new(ident?), Box::new(index?)).into())
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
        vec![None],
    )
}

pub(crate) fn parse_switch(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let mut condition = None;
    let mut guard = None;
    let mut cases: Vec<CaseExpr> = vec![];

    let gen_cases = |cases: &mut Vec<CaseExpr>, guard: &Option<Expr>, in_case: &Vec<&AST>| {
        cases.push(CaseExpr::new(
            guard.clone(),
            Block::new(
                in_case
                    .iter()
                    .flat_map(|c| parse_node(c, component))
                    .collect(),
            ),
        ))
    };

    for child in ast.children.iter() {
        match &*child.r#type {
            "parenthesized_expression" => condition = parse_expr(&child.children[1], component),
            "switch_block" => {
                let mut warmed_up = false;
                let mut in_case: Vec<&AST> = vec![];

                for case_pt in child.children.iter() {
                    match &*case_pt.r#type {
                        "switch_label" => {
                            // If we've recorded the first case, parse it
                            if warmed_up {
                                gen_cases(&mut cases, &guard, &in_case);
                            } else {
                                warmed_up = true;
                            }

                            // Clean up
                            in_case = vec![];

                            // Extract case guard for next one
                            if case_pt.children[0].value != "default" {
                                guard = parse_expr(&case_pt.children[1], component);
                            } else {
                                guard = None;
                            }
                        }
                        _ => in_case.push(case_pt),
                    }
                }

                // Ensure we don't have a straggler case
                if in_case.len() != 0 {
                    gen_cases(&mut cases, &guard, &in_case);
                }
            }
            unknown => log_unknown_tag(unknown, "switch"),
        }
    }

    Some(SwitchExpr::new(Box::new(condition?), cases).into())
}

/// Parse a ternary operator into our rendition of this structure
fn parse_ternary(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let cond = parse_expr(&ast.children[0], component)?;
    let if_true = parse_expr(&ast.children[2], component);
    let if_false = parse_expr(&ast.children[4], component);

    let to_stmt = |stmt: Stmt| to_block(stmt.into());

    Some(
        CallExpr::new(
            Box::new(
                LambdaExpr::new(
                    vec![],
                    to_block(Node::Stmt(
                        IfStmt::new(
                            cond,
                            to_stmt(ReturnStmt::new(if_true).into()),
                            Some(to_stmt(ReturnStmt::new(if_false).into())),
                        )
                        .into(),
                    )),
                )
                .into(),
            ),
            vec![],
        )
        .into(),
    )
}

/// Handle parsing all logical guards
fn parse_binary(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let mut lhs = None;
    let mut op = None;
    let mut rhs = None;
    for child in ast.children.iter() {
        if !is_common_junk_tag(&child.r#type) {
            let res = Some(child);
            if lhs.is_none() {
                lhs = res;
            } else if op.is_none() {
                op = res;
            } else if rhs.is_none() {
                rhs = res;
                break;
            }
        }
    }

    if let Some(lhs) = lhs {
        if let Some(op) = op {
            if let Some(rhs) = rhs {
                return Some(
                    BinaryExpr::new(
                        Box::new(parse_expr(lhs, component)?),
                        op.value.as_str().into(),
                        Box::new(parse_expr(rhs, component)?),
                    )
                    .into(),
                );
            }
        }
    }
    eprintln!("Malformed binary expression detected!");
    None
}

fn parse_unary(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    Some(
        UnaryExpr::new(
            Box::new(parse_expr(&ast.children[1], component)?),
            ast.children[0].r#type.as_str().into(),
        )
        .into(),
    )
}

fn parse_inc_dec(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let name = if ast.children[0].r#type == "identifier" {
        0
    } else {
        1
    };
    let op = (name + 1) % 2;

    Some(
        IncDecExpr::new(
            op < name,
            ast.children[op].r#type == "++",
            Box::new(parse_expr(&ast.children[name], component)?),
        )
        .into(),
    )
}
