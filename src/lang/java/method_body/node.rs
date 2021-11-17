use crate::ast::*;
use crate::java::method_body::expr::parse_expr;
use crate::java::method_body::stmt::{parse_decl, parse_enhanced_for, parse_for, parse_try_catch};
use crate::ComponentInfo;
use crate::Language::Java;
use crate::AST;

use super::{
    parse_block,
    stmt::{parse_do_while, parse_if, parse_labeled, parse_return, parse_throw, parse_while},
};

pub(crate) fn parse_child_nodes(ast: &AST, component: &ComponentInfo) -> Vec<Node> {
    ast.children
        .iter()
        .flat_map(|member| parse_node(member, component))
        .collect()
}

pub(crate) fn parse_node(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    match &*ast.r#type {
        // Variables an initialization
        "local_variable_declaration" | "field_declaration" => {
            Some(Node::Stmt(parse_decl(ast, component).into()))
        }
        "if_statement" => parse_if(ast, component),
        "try_catch" | "try_with_resources_statement" => parse_try_catch(ast, component),
        "expression_statement" => parse_expr_stmt(ast, component),
        "for_statement" => parse_for(ast, component),
        "enhanced_for_statement" => parse_enhanced_for(ast, component),
        "while_statement" => parse_while(ast, component),
        "do_statement" => parse_do_while(ast, component),
        "continue_statement" => make_continue(ast),
        "break_statement" => make_break(ast),
        "labeled_statement" => parse_labeled(ast, component),
        "return_statement" => parse_return(ast, component),
        "throw_statement" => parse_throw(ast, component),
        "block" => Some(parse_block(ast, component).into()),
        _ => {
            let expr: Stmt = parse_expr(ast, component)?.into();
            Some(expr.into())
        }
    }
}

fn make_continue(ast: &AST) -> Option<Node> {
    let mut cont = ContinueStmt::new(Java);
    if let Some(label) = ast.find_child_by_type(&["identifier"]) {
        cont.label = Some(label.value.clone());
    }
    Some(Node::Stmt(cont.into()))
}

/// Ignore label breaks
fn make_break(ast: &AST) -> Option<Node> {
    let mut cont = BreakStmt::new(Java);
    if let Some(label) = ast.find_child_by_type(&["identifier"]) {
        cont.label = Some(label.value.clone());
    }
    Some(Node::Stmt(cont.into()))
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
