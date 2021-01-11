use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(untagged)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    CallExpr(CallExpr),
    IndexExpr(IndexExpr),
    ParenExpr(ParenExpr),
    DotExpr(DotExpr),
    RefExpr(RefExpr),
    StarExpr(StarExpr),
    Literal(String),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    // pub op: Op,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct UnaryExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct CallExpr {
    // This could either be a Literal or a DotExpr
    pub name: Box<Expr>,
    pub args: Vec<crate::FieldComponent>, // Should this be FieldComponent or a new type?
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct IndexExpr {
    pub expr: Box<Expr>,
    pub index_expr: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ParenExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct DotExpr {
    pub expr: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct RefExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct StarExpr {
    pub expr: Box<Expr>,
}