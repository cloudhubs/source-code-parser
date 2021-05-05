use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::Serialize;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

mod op;
pub use op::*;

// enum_dispatch adds in our From implementations for us

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(untagged)]
pub enum Node {
    Block(Block),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct Block {
    pub nodes: Vec<Node>,
    #[new(value = r#""block""#)]
    r#type: &'static str,
}

pub fn to_block(node: Node) -> Block {
    match node {
        Node::Block(block) => block,
        Node::Stmt(stmt) => Block::new(vec![stmt.into()]),
        Node::Expr(expr) => Block::new(vec![Node::Stmt(ExprStmt::new(expr).into())]),
    }
}
