use crate::ast::Expr;
use crate::java::method_body::expr::parse_assignment;
use crate::Language::Java;
use crate::{ast::Block, ComponentInfo, AST};

use self::node::parse_child_nodes;

mod expr;
mod node;
mod stmt;

/// Parse the body of a method, static block, constructor, etc.
pub(crate) fn parse_block(ast: &AST, component: &ComponentInfo) -> Block {
    Block::new(parse_child_nodes(ast, component), Java)
}

/// Logs an unknown tag was encountered. You better not think too much about that.
/// It does not, however, log well-known "filler tags", to keep from cluttering output.
pub(crate) fn log_unknown_tag(tag: &str, parent: &str) {
    if !is_common_junk_tag(tag) {
        tracing::warn!("Unknown tag {} encountered while parsing {}!", tag, parent);
    }
}

/// Catch all for standard-issue junk tags from treesitter, to allow easy blanket-silencing of
/// false alarms, to focus on the tags that are actually important
pub(crate) fn is_common_junk_tag(tag: &str) -> bool {
    // The compiler no longer needs to be smart enough to combine match arms :)
    matches!(
        tag,
        // Reserve words
        "new" | "volatile" | "final" | "static" |
        // Control flow
        "continue" | "break" | "if" | "else" | "for" | "while" | "do" | "switch" | "try" | "catch" | "finally" |
        // Type definitions
        "class" | "interface" | "enum" | 
        // Misc. Syntax
        "(" | ")" | "{" | "}" | "->" | ";" | "," | "." | "..."
    )
}

pub(crate) fn parse_assignment_pub(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
    parse_assignment(ast, component)
}
