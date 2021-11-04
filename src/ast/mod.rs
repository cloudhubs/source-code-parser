use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::Serialize;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

mod op;
pub use op::*;

use source_code_parser_macro::ChildFields;
use source_code_parser_macro::NodeLanguage;

use crate::Language;

// enum_dispatch adds in our From implementations for us

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Serialize, Clone, NodeLanguage, ChildFields)]
#[serde(untagged)]
pub enum Node {
    Block(Block),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct Block {
    pub nodes: Vec<Node>,
    #[new(value = r#""block""#)]
    r#type: &'static str,
    pub language: Language,
}

pub fn to_block(node: Node, language: Language) -> Block {
    match node {
        Node::Block(block) => block,
        Node::Stmt(stmt) => Block::new(vec![stmt.into()], language),
        Node::Expr(expr) => Block::new(
            vec![Node::Stmt(ExprStmt::new(expr, language).into())],
            language,
        ),
    }
}

pub trait NodeLanguage {
    fn get_language(&self) -> Language;
}
