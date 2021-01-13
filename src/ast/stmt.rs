use super::*;
use serde::Serialize;

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
    ImportStmt(ImportStmt),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct AssignStmt {
    pub lhs: String,
    pub rhs: Expr,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct DeclStmt {
    pub lhs: Ident,
    pub rhs: Vec<Expr>, // Vec since FieldComponent could declare multiple variables
}

impl DeclStmt {
    pub fn new(lhs: Ident, rhs: Vec<Expr>) -> DeclStmt {
        DeclStmt { lhs, rhs }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

impl ExprStmt {
    pub fn new(expr: Expr) -> ExprStmt {
        ExprStmt { expr }
    }
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ImportStmt {
    // Whether the import is a specific type or a package/module etc.
    pub container: bool,
    // Whether the import lets functions be referenced by name directly
    pub use_direct: bool,
    pub value: String,
}
