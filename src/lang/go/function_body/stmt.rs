use crate::ast::{to_block, DeclStmt, Expr, ExprStmt, ForStmt, IfStmt, Node, Stmt, VarDecl};
use crate::ComponentInfo;
use crate::AST;

use super::{expr::parse_expr, is_common_junk_tag, node::parse_node, parse_block};
use crate::go::function_body::node::parse_child_nodes;
use crate::go::util::vartype::find_type;

/// Parse an AST section containing a variable declaration
pub(crate) fn parse_decl(ast: &AST, component: &ComponentInfo) -> DeclStmt {
    // Extract informtion about the variable
    let ast = match ast.find_child_by_type(&["var_spec"]) {
        Some(var) => var,
        None => ast,
    };
    let r#type = find_type(ast);

    // Determine the value it was set to
    let rhs = parse_child_nodes(ast, component);

    let mut decl = DeclStmt::new(vec![], vec![]);
    for var in rhs.iter() {
        // Extract expression from the hierarchy
        let base = match var {
            Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. })) | Node::Expr(expr) => expr,
            _ => {
                eprintln!("Unable to interpret as variable: {:#?}", var);
                continue;
            }
        };

        // Parse variable
        match base {
            Expr::BinaryExpr(expr) => match expr.lhs.as_ref() {
                Expr::Ident(lhs) => {
                    decl.variables
                        .push(VarDecl::new(Some(r#type.clone()), lhs.clone()));
                    decl.expressions.push(Some(expr.rhs.as_ref().clone()));
                }
                unknown => eprintln!("Expected Ident got {:#?}", unknown),
            },
            Expr::Ident(id) => {
                decl.variables
                    .push(VarDecl::new(Some(r#type.clone()), id.clone()));
                decl.expressions.push(None);
            }
            Expr::Literal(lit) => decl.expressions.push(Some(lit.clone().into())),
            unknown => {
                eprintln!("Expected BinaryExpr or Ident, got {:#?}", unknown);
            }
        }
    }

    for var_decl in decl.variables.iter_mut() {
        var_decl.is_final = None; //Go does not have final variables
        var_decl.is_static = None; //Go does not have static variables
        var_decl.var_type = Some(r#type.clone());
    }
    decl.into()
}

pub(crate) fn parse_short_decl(ast: &AST, component: &ComponentInfo) -> DeclStmt {
    let mut r#type = "N/A".to_string();
    let mut i = 0;
    for expr in ast.find_all_children_by_type(&["expression_list"]).get_or_insert(vec![]).iter() {
        if i == 0 {
            i+= 1;
        }
        else {
            r#type = determine_var_type(expr);
        }
    }
    



    // Determine the value it was set to
    let rhs = parse_child_nodes(ast, component);

    let mut decl = DeclStmt::new(vec![], vec![]);
    for var in rhs.iter() {
        // Extract expression from the hierarchy
        let base = match var {
            Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. })) | Node::Expr(expr) => expr,
            _ => {
                eprintln!("Unable to interpret as variable: {:#?}", var);
                continue;
            }
        };

        // Parse variable
        match base {
            Expr::BinaryExpr(expr) => match expr.lhs.as_ref() {
                Expr::Ident(lhs) => {
                    decl.variables
                        .push(VarDecl::new(Some(r#type.clone()), lhs.clone()));
                    decl.expressions.push(Some(expr.rhs.as_ref().clone()));
                }
                unknown => eprintln!("Expected Ident got {:#?}", unknown),
            },
            Expr::Ident(id) => {
                decl.variables
                    .push(VarDecl::new(Some(r#type.clone()), id.clone()));
                decl.expressions.push(None);
            }
            Expr::Literal(lit) => decl.expressions.push(Some(lit.clone().into())),
            unknown => {
                eprintln!("Expected BinaryExpr or Ident, got {:#?}", unknown);
            }
        }
    }

    for var_decl in decl.variables.iter_mut() {
        var_decl.is_final = None; //Go does not have final variables
        var_decl.is_static = None; //Go does not have static variables
        var_decl.var_type = Some(r#type.clone());
    }
    decl.into()
}

pub(crate) fn parse_if(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut guard = None;
    let mut if_stmt = None;
    let mut else_stmt = None;

    for child in ast.children.iter().filter(|node| node.r#type != "else") {
        match &*child.r#type {
            "binary_expression" => {
                guard = parse_expr(child, component)
            },
            
            _ => {
                if let Some(stmt) = parse_node(child, component) {
                    let stmt = to_block(stmt);
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
    let retNode = Some(Node::Stmt(IfStmt::new(guard?, if_stmt?, else_stmt).into()));
    retNode
}


pub(crate) fn parse_for(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    let mut for_clauses: Vec<Vec<&AST>> = vec![vec![], vec![], vec![]];
    let mut i = 0;

    //find the node containing the for clauses of the statement
    let clause_node = match ast.find_child_by_type(&["for_clause"]) {
        Some(node) => node,
        None => ast,
    };

    //Coerce an Option<Node> to an Expr, if it can be
    let to_expr = |parts: &Vec<Node>| -> Vec<Expr> {
        parts
            .into_iter()
            .flat_map(|part| match part.clone() {
                Node::Expr(node) => Some(node),
                Node::Stmt(Stmt::ExprStmt(ExprStmt { expr, .. })) => Some(expr),
                _ => None,
            })
            .collect()
    };

    //get all the clauses in the for_clauses vector
    for node in clause_node.children.iter() {
        if !is_common_junk_tag(&*node.r#type) {
            for_clauses[i].push(node);
        }
        else {
            i = i + 1;
        }
    }

    //prepare clauses for parsing
    let parts: Vec<Option<Vec<Node>>> = for_clauses
        .iter()
        .map(|c| {
            if c.len() > 0 {
                Some(
                    c.iter()
                        .map(|c| parse_node(c, component))
                        .flat_map(|c| c)
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect();

    
    
    //parse initialization
    let init = parts[0].clone().map_or(vec![], |init_parts| {
        init_parts
            .into_iter()
            .flat_map(|p| match p {
                Node::Stmt(node) => Some(node),
                Node::Expr(node) => Some(Stmt::ExprStmt(ExprStmt::new(node))),
                _ => panic!("Not supported: block in for loop init"),
            })
            .collect()
    });
    //parse guard condition
    let guard = parts[1]
        .clone()
        .map_or(None, |guard| Some(to_expr(&guard)[0].clone()));

    //parse postcondition
    let post = parts[2].clone().map_or(vec![], |post| to_expr(&post));

    
    
    //find the node containing the block of the for statement
    let block_node = match ast.find_child_by_type(&["block"]) {
        Some(node) => parse_block(node, component),
        None => parse_block(ast, component),
    };
    
    //assemble into a for statement
    let for_stmt = ForStmt::new(
        init,
        guard,
        post,
        block_node,
    );
    //return the node
    Some(Stmt::ForStmt(for_stmt).into())
}


fn determine_var_type(node: &AST) -> String {
    let mut toRet = String::from("N/A");

    match &*node.children[0].r#type {
        "int_literal" => {
            toRet = String::from("int");
        },
        _ => {
            toRet = String::from("Unknown");
        }
    }
    
    toRet
}

