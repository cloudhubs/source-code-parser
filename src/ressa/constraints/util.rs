use crate::ast::{Expr, Ident};

pub fn get_idents(expr: &Expr) -> Vec<&Ident> {
    use Expr::*;
    match expr {
        AssignExpr(e) => e.lhs.iter().flat_map(get_idents).collect(),
        BinaryExpr(e) => get_idents(&e.lhs)
            .into_iter()
            .chain(get_idents(&e.rhs).into_iter())
            .collect(),
        UnaryExpr(e) => get_idents(&e.expr),
        CallExpr(e) => e.args.iter().flat_map(get_idents).collect(),
        ParenExpr(e) => get_idents(&e.expr),
        DotExpr(e) => get_idents(&e.expr),
        IncDecExpr(e) => get_idents(&*e.expr),
        InitListExpr(e) => e.exprs.iter().flat_map(get_idents).collect(),
        Ident(e) => vec![e],
        // IndexExpr(e) => get_ide,
        // LambdaExpr(e) => e.language,
        // Literal(e) => vec![],
        // SwitchExpr(e) => e.language,
        _ => vec![],
    }
}
