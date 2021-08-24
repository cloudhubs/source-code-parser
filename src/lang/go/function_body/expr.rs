use crate::go::function_body::node::{parse_child_nodes, parse_node};
use crate::go::function_body::parse_block;
use crate::go::util::vartype::find_type;

use crate::ast::*;
use crate::ComponentInfo;
use crate::AST;

pub(crate) fn parse_expr(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    match &*ast.r#type {
        // Variables an initialization
        "var_declaration" => parse_assignment(ast, component),
        "identifier" => parse_ident(ast, component),

        unknown => None
    }
}


fn parse_ident(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    let ident: Expr = Ident::new(ast.value.clone()).into();
    Some(ident.into())
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
            //log_unknown_tag(unknown, "parse_assignment");
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

