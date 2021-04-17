use crate::ast::*;
use crate::java::modifier::find_modifier;
use crate::java::vartype::find_type;
use crate::java::vartype::parse_type_args;
use crate::ComponentInfo;
use crate::AST;

use super::util::log_unknown_tag;

/// Parse the body of a method, static block, constructor, etc.
pub(crate) fn parse_block(ast: &AST, component: &ComponentInfo) -> Block {
    Block::new(parse_child_nodes(ast, component))
}

fn parse_child_nodes(ast: &AST, component: &ComponentInfo) -> Vec<Node> {
    ast.children
        .iter()
        .map(|member| parse_node(member, component))
        .flat_map(|some| some)
        .collect()
}

fn parse_node(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    match &*ast.r#type {
        // Variables an initialization
        "local_variable_declaration" | "field_declaration" => {
            Some(Node::Stmt(parse_decl(ast, component).into()))
        }
        "try_catch" | "try_with_resources_statement" => try_catch(ast, component),
        "expression_statement" => parse_expr_stmt(ast, component),
        _ => {
            let expr: Stmt = parse_expr(ast, component)?.into();
            Some(expr.into())
        }
    }
}

fn parse_expr_stmt(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut expr = None;
    for comp in ast.children.iter() {
        expr = parse_expr(comp, component);
        if expr.is_some() {
            break;
        }
    }
    Some(Node::Stmt(expr?.into()))
}

fn parse_expr(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    println!("{}::{}", ast.r#type, ast.value);
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
        "method_invocation" => method_invoke(ast, component).into(),

        // Base case
        unknown => {
            log_unknown_tag(unknown, "expressions");
            None
        }
    }
}

fn parse_field_access(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    // println!("FIELD ACCESS")
    let lhs = parse_expr(&ast.children[0], component)?;
    let rhs = parse_expr(&ast.children[2], component)?;
    Some(Expr::DotExpr(DotExpr::new(Box::new(lhs), Box::new(rhs))))
}

fn method_invoke(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let lhs: Expr = match parse_expr(&ast.children[0], component) {
        Some(opt) => opt,
        None => Literal::new("this".to_string()).into(),
    };

    let mut name = None;
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
                name = Some(Ident::new(result).into());
            }
            unknown => log_unknown_tag(unknown, "method_invoke"),
        }
    }
    println!("{:?}.{:?} ({:?})", lhs, name.clone().expect("ohno"), args);

    // TODO add in generic
    Some(Expr::DotExpr(DotExpr::new(
        Box::new(lhs),
        Box::new(
            CallExpr::new(
                Box::new(name.expect("method with no name requested!")),
                args,
            )
            .into(),
        ),
    )))
}

fn try_catch(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut try_body = None;
    let mut catch_clauses = vec![];
    let mut finally_clause = None;
    // let mut resources = None;

    for comp in ast.children.iter() {
        match &*comp.r#type {
            "resource_specification" => {
                let rss: Vec<Expr> = comp
                    .children
                    .iter()
                    .filter(|resource| match &*resource.r#type {
                        "resource" => true,
                        _ => false,
                    })
                    .map(|resource| parse_assignment(resource, component))
                    .flat_map(|n| n)
                    .collect();
                // resources = Some(DeclStmt::new(rss.iter().map(), expressions));
            }
            "block" => try_body = Some(parse_block(comp, component)),
            "catch_clause" => {
                let catch_comp = comp
                    .find_child_by_type(&["catch_formal_parameter"])
                    .expect("No catch variables declared!");

                // Modifiers
                let modifiers =
                    find_modifier(catch_comp, &*component.path, &*component.package_name);

                // Variables
                let caught_vars = {
                    let name = &catch_comp
                        .find_child_by_type(&["identifier"])
                        .expect("No name for caught variable!")
                        .value;
                    let types: Vec<String> = catch_comp
                        .find_child_by_type(&["catch_type"])
                        .expect("No type on catch block!")
                        .find_all_children_by_type(&["type_identifier"])
                        .expect("No type on catch block!")
                        .iter()
                        .map(|child| child.value.clone())
                        .collect();

                    types
                        .into_iter()
                        .map(|t| {
                            let mut decl = VarDecl::new(Some(t), Ident::new(name.clone()));
                            decl.is_final = Some(modifiers.is_final);
                            decl.is_static = Some(modifiers.is_static);
                            decl.annotation = modifiers.annotations.clone();
                            decl
                        })
                        .collect()
                };

                // Body
                let catch_body = parse_block(
                    comp.find_child_by_type(&["block"])
                        .expect("No block for the catch body!"),
                    component,
                );

                catch_clauses.push(CatchStmt::new(
                    DeclStmt::new(caught_vars, vec![]),
                    catch_body,
                ));
            }
            "finally_clause" => finally_clause = Some(parse_block(ast, component)),
            unknown => log_unknown_tag(unknown, "try/catch"),
        }
    }

    // Generated wrappers and return
    let mut try_catch =
        TryCatchStmt::new(try_body.expect("Try/Catch with no body!"), catch_clauses);
    if finally_clause.is_some() {
        try_catch.finally_body = finally_clause;
    }
    Some(Node::Stmt(try_catch.into()))
}

/// Parse an AST section containing a variable declaration
fn parse_decl(ast: &AST, component: &ComponentInfo) -> DeclStmt {
    // Extract informtion about the variable
    let r#type = find_type(ast);
    let modifier = find_modifier(ast, &*component.path, &*component.package_name);

    // Determine the value it was set to
    let rhs = parse_child_nodes(ast, component);

    let mut decl = DeclStmt::new(vec![], vec![]);
    for var in rhs.iter() {
        let base;

        // Extract expression from the hierarchy
        if let Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. })) = var {
            base = expr;
        } else if let Node::Expr(expr) = var {
            base = expr;
        } else {
            eprintln!("Unable to interpret as variable: {:#?}", var);
            continue;
        }

        // Parse variable
        match base {
            Expr::BinaryExpr(expr) => match expr.lhs.as_ref() {
                Expr::Ident(lhs) => {
                    decl.variables
                        .push(VarDecl::new(Some(r#type.clone()), lhs.clone()));
                    decl.expressions.push(expr.rhs.as_ref().clone());
                }
                unknown => eprintln!("Expected Ident got {:#?}", unknown),
            },
            Expr::Ident(id) => decl
                .variables
                .push(VarDecl::new(Some(r#type.clone()), id.clone())),
            unknown => {
                eprintln!("Expected BinaryExpr or Ident, got {:#?}", unknown);
            }
        }
    }

    // TODO: Use name
    for var_decl in decl.variables.iter_mut() {
        var_decl.is_final = Some(modifier.is_final);
        var_decl.is_static = Some(modifier.is_static);
        var_decl.var_type = Some(r#type.clone());
    }
    decl.into()
}

/// Parse an assignment expression. May contain a variable declaration
fn parse_assignment(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
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
