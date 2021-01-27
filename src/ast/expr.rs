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
    IncDecExpr(IncDecExpr),
    InitListExpr(InitListExpr),
    LogExpr(LogExpr),
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct IncDecExpr {
    pub is_pre: bool,
    pub is_inc: bool,
    pub expr: Box<Expr>,
    #[new(value = r#""inc_dec_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct InitListExpr {
    pub exprs: Vec<Expr>,
    #[new(value = r#""init_list_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct LogExpr {
    pub level: LogLevel,
    pub args: Vec<Expr>,
    #[new(value = r#""log_expr""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum LogLevel {
    Console,
    Debug,
    Warning,
    Info,
    Error,
    Fatal,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Console
    }
}

impl From<LogLevel> for String {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => "debug".into(),
            LogLevel::Error => "error".into(),
            LogLevel::Fatal => "fatal".into(),
            LogLevel::Warning => "warning".into(),
            LogLevel::Info => "info".into(),
            _ => "console".into(),
        }
    }
}

impl From<&str> for LogLevel {
    fn from(string: &str) -> LogLevel {
        match string {
            "debug" => LogLevel::Debug,
            "error" => LogLevel::Error,
            "fatal" => LogLevel::Fatal,
            "warning" => LogLevel::Warning,
            "info" => LogLevel::Info,
            _ => LogLevel::default(),
        }
    }
}
