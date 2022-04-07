use crate::ast::op::Op;
use crate::ast::stmt::*;
use crate::ast::Block;
use crate::Language;
use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::Serialize;

use source_code_parser_macro::ChildFields;
use source_code_parser_macro::NodeLanguage;

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Serialize, Clone, NodeLanguage, ChildFields)]
#[serde(untagged)]
pub enum Expr {
    AssignExpr(AssignExpr),
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    CallExpr(CallExpr),
    EndpointCallExpr(EndpointCallExpr),
    IndexExpr(IndexExpr),
    ParenExpr(ParenExpr),
    DotExpr(DotExpr),
    IncDecExpr(IncDecExpr),
    InitListExpr(InitListExpr),
    LogExpr(LogExpr),
    LambdaExpr(LambdaExpr),
    Ident(Ident),
    Literal(Literal),
    SwitchExpr(SwitchExpr),
    CaseExpr(CaseExpr),
}

impl Expr {
    pub fn get_lang(&self) -> Language {
        use Expr::*;
        match self {
            AssignExpr(e) => e.language,
            BinaryExpr(e) => e.language,
            UnaryExpr(e) => e.language,
            CallExpr(e) => e.language,
            EndpointCallExpr(e) => e.language,
            IndexExpr(e) => e.language,
            ParenExpr(e) => e.language,
            DotExpr(e) => e.language,
            IncDecExpr(e) => e.language,
            InitListExpr(e) => e.language,
            LogExpr(e) => e.language,
            LambdaExpr(e) => e.language,
            Ident(e) => e.language,
            Literal(e) => e.language,
            SwitchExpr(e) => e.language,
            CaseExpr(e) => e.language,
        }
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        let lang = expr.get_lang();
        Stmt::ExprStmt(ExprStmt::new(expr, lang))
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct AssignExpr {
    pub lhs: Vec<Expr>,
    pub rhs: Vec<Expr>,
    #[new(value = r#""assign_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>,
    #[new(value = r#""binary_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct UnaryExpr {
    pub expr: Box<Expr>,
    pub op: Op,
    #[new(value = r#""unary_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct CallExpr {
    // This could either be a Literal or a DotExpr
    pub name: Box<Expr>,
    pub args: Vec<Expr>,
    #[new(value = r#""call_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct EndpointCallExpr {
    pub service_module_name: String,
    pub service_class_name: Option<String>,
    pub endpoint_method_name: String,
    pub call_expr: CallExpr,
    #[new(value = r#""endpoint_call_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct IndexExpr {
    pub expr: Box<Expr>,
    pub index_expr: Box<Expr>,
    #[new(value = r#""index_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct ParenExpr {
    pub expr: Box<Expr>,
    #[new(value = r#""paren_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct DotExpr {
    pub expr: Box<Expr>,
    pub selected: Box<Expr>,
    #[new(value = r#""dot_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct Ident {
    pub name: String,
    #[new(value = r#""ident_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct IncDecExpr {
    pub is_pre: bool,
    pub is_inc: bool,
    pub expr: Box<Expr>,
    #[new(value = r#""inc_dec_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct InitListExpr {
    pub exprs: Vec<Expr>,
    #[new(value = r#""init_list_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct LogExpr {
    pub level: LogLevel,
    pub args: Vec<Expr>,
    #[new(value = r#""log_expr""#)]
    r#type: &'static str,
    pub language: Language,
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct LambdaExpr {
    pub parameters: Vec<DeclStmt>,
    pub body: Block,
    #[new(value = r#""lambda_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct Literal {
    pub value: String,
    #[new(value = r#""literal_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct SwitchExpr {
    pub condition: Box<Expr>,
    pub cases: Vec<CaseExpr>,
    #[new(value = r#""switch_expr""#)]
    r#type: &'static str,
    pub language: Language,
}

impl From<SwitchExpr> for Stmt {
    /// Enable a SwitchExpr to be easily coerced to a Stmt. Since Switches (depending on the language)
    /// could be either a statement or an expression, giving it this special case makes sense.
    fn from(expr: SwitchExpr) -> Self {
        let expr: Expr = expr.into();
        let lang = expr.get_lang();
        Stmt::ExprStmt(ExprStmt::new(expr, lang))
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new, NodeLanguage, ChildFields)]
pub struct CaseExpr {
    pub cond: Option<Box<Expr>>,
    pub body: Box<Block>,
    #[new(value = r#""case_expr""#)]
    r#type: &'static str,
    pub language: Language,
}
