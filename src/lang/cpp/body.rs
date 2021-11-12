use super::*;
use crate::ast::*;
use crate::parse::AST;

/// Takes in an AST with type field "compound_statement" and converts it to a Block
pub fn func_body(body: &AST) -> Block {
    let nodes = block_nodes(body);
    Block::new(nodes, Language::Cpp)
}

pub(crate) fn block_nodes(compound_statement: &AST) -> Vec<Node> {
    block_nodes_iter(&compound_statement.children)
}

pub(crate) fn block_nodes_iter(children: &[AST]) -> Vec<Node> {
    children.iter().map(body_node).flatten().collect()
}
