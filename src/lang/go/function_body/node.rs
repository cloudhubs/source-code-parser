use crate::ast::*;
use crate::ComponentInfo;
use crate::AST;

use crate::go::function_body::stmt::parse_decl;

use super::{
    parse_block,
    //stmt::{parse_do_while, parse_if, parse_labeled, parse_return, parse_throw, parse_while},
};
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
        "var_declaration" => {
            let decl = Some(Node::Stmt(parse_decl(ast, component).into()));

            decl
        }
        _ =>  {
            let expr: Stmt = parse_expr(ast, component)?.into();
            Some(expr.into())
        }
    }
}