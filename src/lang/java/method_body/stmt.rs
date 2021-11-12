use crate::ast::*;
use crate::java::method_body::log_unknown_tag;
use crate::java::method_body::parse_block;
use crate::java::method_body::parse_child_nodes;
use crate::java::modifier::find_modifier;
use crate::java::modifier::parse_modifiers;
use crate::java::util::vartype::find_type;
use crate::ComponentInfo;
use crate::Language::Java;
use crate::AST;

use super::{expr::parse_expr, is_common_junk_tag, node::parse_node};

/// File holding all Java statement parsing (e.g., while/for/trycatch)

/// Parse an AST section containing a variable declaration
pub(crate) fn parse_decl(ast: &AST, component: &ComponentInfo) -> DeclStmt {
    // Extract informtion about the variable
    let r#type = find_type(ast);
    let modifier = find_modifier(ast, &*component.path, &*component.package_name);

    // Determine the value it was set to
    let rhs = parse_child_nodes(ast, component);

    let mut decl = DeclStmt::new(vec![], vec![], Java);
    for var in rhs.iter() {
        // Extract expression from the hierarchy
        let base = match var {
            Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. })) | Node::Expr(expr) => expr,
            _ => {
                tracing::warn!("Unable to interpret as variable: {:#?}", var);
                continue;
            }
        };

        // Parse variable
        match base {
            Expr::BinaryExpr(expr) => match expr.lhs.as_ref() {
                Expr::Ident(lhs) => {
                    decl.variables
                        .push(VarDecl::new(Some(r#type.clone()), lhs.clone(), Java));
                    decl.expressions.push(Some(expr.rhs.as_ref().clone()));
                }
                unknown => tracing::warn!("Expected Ident got {:#?}", unknown),
            },
            Expr::Ident(id) => {
                decl.variables
                    .push(VarDecl::new(Some(r#type.clone()), id.clone(), Java));
                decl.expressions.push(None);
            }
            unknown => {
                tracing::warn!("Expected BinaryExpr or Ident, got {:#?}", unknown);
            }
        }
    }

    for var_decl in decl.variables.iter_mut() {
        var_decl.is_final = Some(modifier.is_final);
        var_decl.is_static = Some(modifier.is_static);
        var_decl.var_type = Some(r#type.clone());
    }
    decl
}

pub(crate) fn parse_if(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut guard = None;
    let mut if_stmt = None;
    let mut else_stmt = None;

    for child in ast.children.iter() {
        match &*child.r#type {
            "parenthesized_expression" => guard = parse_expr(child, component),
            _ => {
                if let Some(stmt) = parse_node(child, component) {
                    let stmt = to_block(stmt, Java);
                    if if_stmt.is_none() {
                        if_stmt = Some(stmt);
                    } else {
                        else_stmt = Some(stmt);
                        break;
                    }
                }
            }
        }
    }
    Some(Node::Stmt(
        IfStmt::new(guard?, if_stmt?, else_stmt, Java).into(),
    ))
}

/// Parse an AST fragment with a try/catch. May be try-with-resources, or standard try/catch, with any
/// number of catch/multi-catch blocks, and/or a finally block.
pub(crate) fn parse_try_catch(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut try_body = None;
    let mut catch_clauses = vec![];
    let mut finally_clause = None;
    let mut resources = None;

    for comp in ast.children.iter() {
        match &*comp.r#type {
            "resource_specification" => {
                let rss_list: Vec<DeclStmt> = comp
                    .find_all_children_by_type(&["resource"])
                    .get_or_insert(vec![])
                    .iter()
                    .flat_map(|resource| parse_resource(resource, component))
                    .collect();
                resources = Some(rss_list.into());
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
                    catch_comp
                        // Get child catch block
                        .find_child_by_type(&["catch_type"])
                        .expect("No type on catch block!")
                        // Get type on catch block
                        .find_all_children_by_type(&["type_identifier"])
                        .expect("No type on catch block!")
                        .iter()
                        // Clone each type on the catch block and map to a VarDecl
                        .map(|child| child.value.clone())
                        .map(|t| {
                            let mut decl =
                                VarDecl::new(Some(t), Ident::new(name.clone(), Java), Java);
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
                    DeclStmt::new(caught_vars, vec![], Java),
                    catch_body,
                    Java,
                ));
            }
            "finally_clause" => finally_clause = Some(parse_block(ast, component)),
            unknown => log_unknown_tag(unknown, "try/catch"),
        }
    }

    // Build body; if with resources, wrap before returning
    let mut try_catch: Stmt = TryCatchStmt::new(
        try_body.expect("Try/Catch with no body!"),
        catch_clauses,
        finally_clause,
        Java,
    )
    .into();
    if let Some(resources) = resources {
        try_catch =
            WithResourceStmt::new(resources, Block::new(vec![try_catch.into()], Java), Java).into();
    };
    Some(try_catch.into())
}

fn parse_resource(ast: &AST, component: &ComponentInfo) -> Option<DeclStmt> {
    let mut modifier = None;
    let mut name = None;
    let mut result = None;
    let mut r#type = None;

    for child in ast.children.iter() {
        match &*child.r#type {
            "modifiers" => {
                modifier = Some(parse_modifiers(
                    child,
                    &*component.path,
                    &*component.package_name,
                ))
            }
            "type_identifier" => r#type = Some(find_type(child)),
            _ => {
                if !is_common_junk_tag(&*child.r#type) {
                    match parse_expr(child, component) {
                        Some(Expr::Ident(expr)) => name = Some(expr),
                        Some(expr) => result = Some(expr),
                        None => {}
                    }
                }
            }
        }
    }

    // Unwrap
    let mut decl = VarDecl::new(r#type, name?, Java); //DeclStmt::new(vec![], vec![result?]);

    // Inject modifier information
    if let Some(mut modifier) = modifier {
        decl.annotation.append(&mut modifier.annotations);
        decl.is_final = Some(modifier.is_final);
        decl.is_static = Some(modifier.is_static);
    }

    // Assemble into declaration
    Some(DeclStmt::new(vec![decl], vec![result], Java))
}

pub(crate) fn parse_for(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut clauses: Vec<Vec<&AST>> = vec![vec![], vec![], vec![], vec![]];
    let mut i = 0;

    // Coerce an Option<Node> to an Expr, if it can be
    let to_expr = |parts: &Vec<Node>| -> Vec<Expr> {
        parts
            .iter()
            .flat_map(|part| match part.clone() {
                Node::Expr(node) => Some(node),
                Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. })) => Some(expr),
                _ => None,
            })
            .collect()
    };

    // Find all init, guard, and postcondition blocks
    for child in ast.children.iter() {
        match &*child.r#type {
            ";" | ")" => i += 1,
            unknown => {
                if !is_common_junk_tag(unknown) {
                    clauses[i].push(child);
                }
            }
        }
    }

    // Parse loop body (should be last)
    let body;
    let raw_body = clauses.pop()?;
    if !raw_body.is_empty() {
        body = parse_child_nodes(raw_body[0], component);
    } else {
        body = vec![];
    }

    // Parse for loop parts
    let parts: Vec<Option<Vec<Node>>> = clauses
        .iter()
        .map(|c| {
            if !c.is_empty() {
                Some(
                    c.iter()
                        .map(|c| parse_node(c, component))
                        .flatten()
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect();

    // Parse initialization
    let init = parts[0].clone().map_or(vec![], |init_parts| {
        init_parts
            .into_iter()
            .flat_map(|p| match p {
                Node::Stmt(node) => Some(node),
                Node::Expr(node) => Some(Stmt::ExprStmt(ExprStmt::new(node, Java))),
                _ => panic!("Not supported: block in for loop init"),
            })
            .collect()
    });

    // Parse guard condition
    let guard = parts[1].clone().map(|guard| to_expr(&guard)[0].clone());

    // Parse postcondition
    let post = parts[2].clone().map_or(vec![], |post| to_expr(&post));

    // Assemble
    Some(
        Stmt::ForStmt(ForStmt::new(
            init,
            guard,
            post,
            Block::new(body, Java),
            Java,
        ))
        .into(),
    )
}

pub(crate) fn parse_enhanced_for(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    // Extract iterator
    let iter_type = find_type(ast);
    let iter_var = DeclStmt::new(
        vec![VarDecl::new(
            Some(iter_type),
            Ident::new(ast.children[3].value.clone(), Java),
            Java,
        )],
        vec![],
        Java,
    );
    let iter = parse_expr(&ast.children[5], component);

    // Extract body
    let body;
    if let Some(block) = ast.find_child_by_type(&["block"]) {
        body = parse_block(block, component);
    } else {
        body = Block::new(vec![], Java);
    }

    Some(Node::Stmt(
        ForRangeStmt::new(Box::new(Stmt::DeclStmt(iter_var)), iter, body, Java).into(),
    ))
}

pub(crate) fn parse_labeled(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let label = LabelStmt::new(ast.children[0].value.clone(), Java);
    let body = parse_node(&ast.children[2], component);
    Some(Block::new(vec![Stmt::LabelStmt(label).into(), body?], Java).into())
}

pub(crate) fn parse_while(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut guard = None;
    let mut stmt = Block::new(vec![], Java);

    for child in ast.children.iter() {
        match &*child.r#type {
            "parenthesized_expression" => guard = parse_expr(child, component),
            _ => {
                if let Some(body_stmt) = parse_node(child, component) {
                    stmt = to_block(body_stmt, Java);
                }
            }
        }
    }
    Some(Node::Stmt(WhileStmt::new(guard?, stmt, Java).into()))
}

pub(crate) fn parse_do_while(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut stmt = Block::new(vec![], Java);
    for child in ast.children.iter() {
        match &*child.r#type {
            "parenthesized_expression" => {
                return Some(Node::Stmt(
                    DoWhileStmt::new(parse_expr(child, component)?, stmt, Java).into(),
                ));
            }
            _ => {
                if let Some(body_stmt) = parse_node(child, component) {
                    stmt = to_block(body_stmt, Java);
                }
            }
        }
    }

    // Uh... oops?
    tracing::warn!("Failure to find all parts of a do/while loop! Cannot assemble");
    None
}

pub(crate) fn parse_return(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    Some(Node::Stmt(
        ReturnStmt::new(parse_expr(&ast.children[1], component), Java).into(),
    ))
}

pub(crate) fn parse_throw(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    Some(Node::Stmt(
        ThrowStmt::new(parse_expr(&ast.children[1], component), Java).into(),
    ))
}
