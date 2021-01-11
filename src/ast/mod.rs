use serde::Serialize;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(untagged)]
pub enum Node {
    Block(Block),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct Block {
    pub nodes: Vec<Node>,
}

