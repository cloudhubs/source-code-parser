use super::*;
use derive_more::From;
use derive_new::new;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone, From)]
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
    ImportStmt(ImportStmt),
    BreakStmt(BreakStmt),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct AssignStmt {
    // lhs could be something like *var
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct DeclStmt {
    pub lhs: Ident,
    pub rhs: Vec<Expr>, // Vec since FieldComponent could declare multiple variables
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, From, new)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct IfStmt {
    pub cond: Expr,
    pub body: Block,
    pub else_body: Option<Block>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ForStmt {
    pub init: Expr, // Expr, BinExpr that is = ? or a new DeclExpr?
    pub condition: Expr,
    pub body: Block,
    pub post: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct DoWhileStmt {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ReturnStmt {
    pub expr: Option<Expr>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct SwitchStmt {
    pub condition: Expr,
    pub cases: Vec<(Option<Expr>, Block)>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct IncDecStmt {
    pub is_pre: bool,
    pub is_inc: bool,
    pub expr: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ImportStmt {
    // Whether the import is a specific type or a package/module etc.
    pub container: bool,
    // Whether the import lets functions be referenced by name directly
    pub use_direct: bool,
    pub value: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct BreakStmt;
