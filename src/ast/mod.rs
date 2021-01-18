use derive_more::From;
use derive_new::new;
use serde::Serialize;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

mod op;
pub use op::*;

#[derive(Debug, Eq, PartialEq, Serialize, Clone, From)]
#[serde(untagged)]
pub enum Node {
    Block(Block),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct Block {
    pub nodes: Vec<Node>,
}
