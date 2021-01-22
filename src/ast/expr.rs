use crate::ast::op::Op;
use crate::ast::stmt::*;
use derive_more::From;
use derive_new::new;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone, From)]
#[serde(untagged)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    CallExpr(CallExpr),
    IndexExpr(IndexExpr),
    ParenExpr(ParenExpr),
    DotExpr(DotExpr),
    Ident(Ident),
    Literal(String),
}

impl Into<Stmt> for Expr {
    fn into(self) -> Stmt {
        Stmt::ExprStmt(ExprStmt::new(self))
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>,
    #[new(value = r#""binary_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct UnaryExpr {
    pub expr: Box<Expr>,
    pub op: Op,
    #[new(value = r#""unary_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct CallExpr {
    // This could either be a Literal or a DotExpr
    pub name: Box<Expr>,
    pub args: Vec<Expr>,
    #[new(value = r#""call_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct IndexExpr {
    pub expr: Box<Expr>,
    pub index_expr: Box<Expr>,
    #[new(value = r#""index_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ParenExpr {
    pub expr: Box<Expr>,
    #[new(value = r#""paren_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct DotExpr {
    pub expr: Box<Expr>,
    pub selected: Box<Expr>,
    #[new(value = r#""dot_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct Ident {
    pub name: String,
    #[new(value = r#""ident_expr""#)]
    r#type: &'static str,
}
