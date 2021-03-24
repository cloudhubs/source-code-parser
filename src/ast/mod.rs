use derive_more::From;
use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::Serialize;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

mod op;
pub use op::*;

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Serialize, Clone)] //, From)]
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
