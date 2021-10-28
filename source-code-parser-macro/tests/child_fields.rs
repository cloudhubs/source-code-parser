use source_code_parser_macro::ChildFields;

pub struct ExprStmt;
pub struct IndexExpr;
pub struct Ident;
pub struct Stmt;
pub enum Expr {
    ExprStmt(ExprStmt),
    IndexExpr(IndexExpr),
}

#[derive(ChildFields)]
pub struct Node {
    regular: Stmt,
    boxed: Box<Ident>,
    vector: Vec<ExprStmt>,
    opt: Option<Ident>,
    vec_opt: Vec<Option<Stmt>>,
    opt_vec: Option<Vec<ExprStmt>>,
    opt_vec_opt: Option<Vec<Option<Stmt>>>,
    nested: Expr,
}

fn main() {}
