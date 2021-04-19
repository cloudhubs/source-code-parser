use crate::ast::*;
use crate::java::method_body::expr::parse_expr;
use crate::java::method_body::stmt::{parse_decl, parse_for, try_catch};
use crate::ComponentInfo;
use crate::AST;

pub(crate) fn parse_child_nodes(ast: &AST, component: &ComponentInfo) -> Vec<Node> {
    ast.children
        .iter()
        .map(|member| parse_node(member, component))
        .flat_map(|some| some)
        .collect()
}

pub(crate) fn parse_node(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    match &*ast.r#type {
        // Variables an initialization
        "local_variable_declaration" | "field_declaration" => {
            Some(Node::Stmt(parse_decl(ast, component).into()))
        }
        "try_catch" | "try_with_resources_statement" => try_catch(ast, component),
        "expression_statement" => parse_expr_stmt(ast, component),
        "for_statement" => parse_for(ast, component),
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
