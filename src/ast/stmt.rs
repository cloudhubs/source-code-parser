use super::*;
use derive_more::From;
use derive_new::new;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone, From)]
#[serde(untagged)]
pub enum Stmt {
    DeclStmt(DeclStmt),
    ExprStmt(ExprStmt),
    IfStmt(IfStmt),
    ForStmt(ForStmt),
    ForRangeStmt(ForRangeStmt),
    WhileStmt(WhileStmt),
    DoWhileStmt(DoWhileStmt),
    ReturnStmt(ReturnStmt),
    SwitchStmt(SwitchStmt),
    CaseStmt(CaseStmt),
    ImportStmt(ImportStmt),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ThrowStmt(ThrowStmt),
    TryCatchStmt(TryCatchStmt),
    CatchStmt(CatchStmt),
    WithResourceStmt(WithResourceStmt),
}

/// For variable declaration statements, we can represent various situations for
/// initialization.
///
/// In the example of Go variables may be delcared with a statement
/// like `someVar, ok := os.Config("/path/to/file")` where we would represent this
/// with two variables in the `variables` field, and one `CallExpr` in the `expressions`
/// field.
///
/// For other declarations like `x, y := foo(), bar()`, we represent this just by
/// having two variables and two call expressions in the respective `Vec` fields.
#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct DeclStmt {
    /// The declared variable(s).
    pub variables: Vec<VarDecl>,
    /// The expression(s) being assigned to the declared variables.
    pub expressions: Vec<Expr>,
    #[new(value = r#""decl_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct VarDecl {
    /// The type of the declared variable.
    pub var_type: Option<String>,
    /// The variable(s).
    pub ident: Ident,
    #[new(default)]
    pub is_static: Option<bool>,
    #[new(default)]
    pub is_final: Option<bool>,
    #[new(value = r#"vec![]"#)]
    pub annotation: Vec<crate::AnnotationComponent>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, From, new)]
pub struct ExprStmt {
    pub expr: Expr,
    #[new(value = r#""expr_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct IfStmt {
    pub cond: Expr,
    pub body: Block,
    pub else_body: Option<Block>,
    #[new(value = r#""if_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ForStmt {
    // ExprStmt(BinExpr) or DeclStmt
    pub init: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub post: Option<Expr>,
    pub body: Block,
    #[new(value = r#""for_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ForRangeStmt {
    // ExprStmt or DecLStmt
    pub init: Box<Stmt>,
    pub iterator: Option<Expr>,
    pub body: Block,
    #[new(value = r#""for_range_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
    #[new(value = r#""while_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct DoWhileStmt {
    pub condition: Expr,
    pub body: Block,
    #[new(value = r#""do_while_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ReturnStmt {
    pub expr: Option<Expr>,
    #[new(value = r#""return_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct SwitchStmt {
    pub condition: Expr,
    pub cases: Vec<CaseStmt>,
    #[new(value = r#""switch_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct CaseStmt {
    cond: Option<Expr>,
    body: Block,
    #[new(value = r#""case_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ImportStmt {
    // Whether the import is a specific type or a package/module etc.
    pub container: bool,
    // Whether the import lets functions be referenced by name directly
    pub use_direct: bool,
    pub value: String,
    #[new(value = r#""import_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct BreakStmt {
    #[new(value = r#""break_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ContinueStmt {
    #[new(value = r#""continue_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct ThrowStmt {
    expr: Option<Expr>,
    #[new(value = r#""throw_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct TryCatchStmt {
    pub try_body: Block,
    pub catch_bodies: Vec<CatchStmt>,
    #[new(value = "None")]
    pub finally_body: Option<Block>,
    #[new(value = r#""try_catch_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct CatchStmt {
    pub exc: DeclStmt,
    pub body: Block,
    #[new(value = r#""catch_stmt""#)]
    r#type: &'static str,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone, new)]
pub struct WithResourceStmt {
    pub resources: DeclStmt,
    pub body: Block,
    #[new(value = r#""with_resources_stmt""#)]
    r#type: &'static str,
}
