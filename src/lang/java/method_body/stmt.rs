use crate::ast::*;
use crate::java::method_body::expr::parse_assignment;
use crate::java::method_body::log_unknown_tag;
use crate::java::method_body::parse_block;
use crate::java::method_body::parse_child_nodes;
use crate::java::modifier::find_modifier;
use crate::java::util::vartype::find_type;
use crate::ComponentInfo;
use crate::AST;

use super::{expr::parse_expr, node::parse_node};

/// File holding all Java statement parsing (e.g., while/for/trycatch)

/// Parse an AST section containing a variable declaration
pub(crate) fn parse_decl(ast: &AST, component: &ComponentInfo) -> DeclStmt {
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

/// Parse an AST fragment with a try/catch. May be try-with-resources, or standard try/catch, with any
/// number of catch/multi-catch blocks, and/or a finally block.
pub(crate) fn try_catch(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut try_body = None;
    let mut catch_clauses = vec![];
    let mut finally_clause = None;
    // let mut resources = None;

    for comp in ast.children.iter() {
        match &*comp.r#type {
            "resource_specification" => {
                // let rss: Vec<Expr> = comp
                //     .children
                //     .iter()
                //     .filter(|resource| &*resource.r#type  == "resource")
                //     .map(|resource| parse_assignment(resource, component))
                //     .flat_map(|n| match n {
                //     })
                //     .collect();
                // resources = Some(DeclStmt::new(rss, expressions));
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

pub(crate) fn parse_for(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut clauses: Vec<Vec<&AST>> = vec![vec![], vec![], vec![], vec![]];
    let mut i = 0;

    // Find all init, guard, and postcondition blocks
    for child in ast.children.iter() {
        match &*child.r#type {
            ";" => i = i + 1,
            "," | "for" | "(" => { /* Expected junk tags */ }
            ")" => break,
            _ => clauses[i].push(child),
        }
    }

    // Parse all children
    let parts: Vec<Option<Node>> = clauses
        .iter()
        .map(|c| {
            if c.len() > 1 {
                todo!("Can't handle more than one declaration at a time yet!");
            } else if c.len() == 1 {
                parse_node(c[0], component)
            } else {
                None
            }
        })
        .collect();

    // Parse initialization
    let init = match parts[0].clone() {
        Some(Node::Stmt(node)) => Some(Box::new(node)),
        Some(Node::Expr(node)) => Some(Box::new(Stmt::ExprStmt(ExprStmt::new(node)))),
        Some(Node::Block(_)) => panic!("Not supported: block in for loop init"),
        None => None,
    };

    // Parse guard condition
    let guard = match parts[1].clone() {
        Some(Node::Expr(node)) => Some(node),
        _ => None,
    };

    // Parse postcondition
    let post = match parts[2].clone() {
        Some(Node::Expr(node)) => Some(node),
        Some(Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. }))) => Some(expr),
        _ => None,
    };

    // Parse loop body
    let body = match parts[3].clone() {
        Some(node) => Block::new(vec![node]),
        None => Block::new(vec![]),
    };

    // Assemble
    Some(Stmt::ForStmt(ForStmt::new(init, guard, post, body)).into())
}
