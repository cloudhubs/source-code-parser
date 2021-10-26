use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::Serialize;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

mod op;
pub use op::*;

use crate::Language;

// enum_dispatch adds in our From implementations for us

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(untagged)]
pub enum Node {
    Block(Block),
    Stmt(Stmt),
    Expr(Expr),
}

impl Node {
    fn get_lang(&self) -> Language {
        match self {
            Node::Block(block) => block.language,
            Node::Stmt(stmt) => stmt.get_lang(),
            Node::Expr(expr) => expr.get_lang(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
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

macro_rules! impl_node_language {
    ( $( $ty_name:ty ),+ ) => {
        $(
            impl NodeLanguage for $ty_name {
                fn get_language(&self) -> Language {
                    self.get_lang()
                }
            }
        )*
    };
}
impl_node_language!(Node, Expr, Stmt);
