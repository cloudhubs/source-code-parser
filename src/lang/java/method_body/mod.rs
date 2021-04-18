use crate::{ast::Block, ComponentInfo, AST};

use self::node::parse_child_nodes;

mod expr;
mod node;
mod stmt;

/// Parse the body of a method, static block, constructor, etc.
pub(crate) fn parse_block(ast: &AST, component: &ComponentInfo) -> Block {
    Block::new(parse_child_nodes(ast, component))
}

/// Logs an unknown tag was encountered. You better not think too much about that.
pub(crate) fn log_unknown_tag(tag: &str, parent: &str) {
    eprintln!("Unknown tag {} encountered while parsing {}!", tag, parent);
}
