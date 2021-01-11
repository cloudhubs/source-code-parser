use serde::Serialize;
use super::*;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(untagged)]
pub enum Stmt {
    AssignStmt(AssignStmt),
    DeclStmt(DeclStmt),
    ExprStmt(ExprStmt),
    IfStmt(IfStmt),
    ForStmt(ForStmt),
    WhileStmt(WhileStmt),
    DoWhileStmt(DoWhileStmt),
    ReturnStmt(ReturnStmt),
    SwitchStmt(SwitchStmt),
    IncDecStmt(IncDecStmt),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct AssignStmt {
    pub lhs: String,
    pub rhs: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct DeclStmt {
    pub lhs: crate::FieldComponent,
    pub rhs: Vec<Expr>, // Vec since FieldComponent could declare multiple variables
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct IfStmt {
    pub body: Block,
    pub else_body: Block,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ForStmt {
    pub init: Expr, // Expr, BinExpr that is = ? or a new DeclExpr?
    pub condition: Expr,
    pub body: Block,
    pub post: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct DoWhileStmt {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ReturnStmt {
    pub expr: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct SwitchStmt {
    pub condition: Expr,
    pub cases: Vec<(Expr, Block)>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct IncDecStmt {
    pub is_pre: bool,
    pub is_inc: bool,
    pub expr: Expr,
}
