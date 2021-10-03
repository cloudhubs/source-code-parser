use crate::ast::*;
use crate::ComponentInfo;
use crate::AST;

use crate::go::function_body::stmt::*;

use super::parse_block;
use crate::go::function_body::expr::parse_expr;

pub(crate) fn parse_child_nodes(ast: &AST, component: &ComponentInfo) -> Vec<Node> {
    ast.children
        .iter()
        .map(|member| parse_node(member, component))
        .flat_map(|some| some)
        .collect()
}

pub(crate) fn parse_node(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    match &*ast.r#type {
        "var_declaration" => Some(Node::Stmt(parse_decl(ast, component).into())),
        "short_var_declaration" => Some(Node::Stmt(parse_short_decl(ast, component).into())),
        "if_statement" => parse_if(ast, component),
        "block" => Some(parse_block(ast, component).into()),
        
        "for_statement" => parse_for(ast, component),

        _ => {
            let expr: Stmt = parse_expr(ast, component)?.into();
            Some(expr.into())
        },
        
    }
}


