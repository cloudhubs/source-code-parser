use std::collections::HashMap;

pub mod coerce;
pub use coerce::*;

pub mod types;
pub use types::*;

use crate::{
    ast::{Expr, Ident, Node, Stmt},
    Language,
};

use super::Indexable;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct SimpleIdent(String);
impl SimpleIdent {
    fn new(ident: &Ident) -> SimpleIdent {
        SimpleIdent(format!("{}#{}", ident.language, ident.name))
    }
}

/// Mapping for known variables to known constraints
pub struct ConstraintStack(HashMap<SimpleIdent, Vec<Constraint>>);

impl ConstraintStack {
    fn push_constraint(&mut self, node: &Node) {
        // self.do_push_constraint(node, true);
    }

    // fn do_push_constraint(&mut self, node: &Node, is_true: bool) -> Option<Vec<SimpleIdent>> {
    //     None
    // }

    // /// Yes, this returns Option just so I can use `?`. I'm lazy.
    // fn handle_expr(&mut self, expr: &Expr, is_true: bool) -> Option<Vec<SimpleIdent>> {
    //     match expr {
    //         Expr::AssignExpr(assign) => {
    //             let lhs_len = assign.lhs.len();
    //             let rhs_len = assign.rhs.len();

    //             // Verify type of assign expr in question
    //             if lhs_len == 1 && rhs_len > 1 {
    //                 // Tuple assign assumed
    //                 let lhs = &assign.lhs[0];
    //                 // let stack = self.get(SimpleIdent::from_strings(""))?;
    //             } else if lhs_len == rhs_len {
    //                 // Set of assignments
    //                 let mut i = 0;
    //                 while i < lhs_len {
    //                     let x = &assign.lhs[i];
    //                     //
    //                     i += 1;
    //                 }
    //             }
    //             None
    //         }
    //         Expr::BinaryExpr(_) => todo!(),
    //         Expr::EndpointCallExpr(_) => todo!(),
    //         Expr::IndexExpr(_) => todo!(),
    //         Expr::ParenExpr(_) => todo!(),
    //         Expr::DotExpr(_) => todo!(),
    //         Expr::IncDecExpr(_) => todo!(),
    //         // Expr::CaseExpr(case) => todo!(),
    //         Expr::SwitchExpr(switch) => switch.cases.iter().map(f),

    //         Expr::UnaryExpr(unary) => match unary.op {
    //             // TODO implement the following
    //             // crate::ast::Op::Plus => {} // TODO This is potentially important in numeric comparisons
    //             // crate::ast::Op::Minus => {} // TODO This is important in numeric comparisons
    //             // crate::ast::Op::PlusPlus => {} // TODO: for x, invalidate all `x < #` constraints, change any `x == #` to `x > #`, and retain `x > #`
    //             // crate::ast::Op::MinusMinus => {} // TODO: for x, invalidate all `x > #` constraints, change any `x == #` to `x < #`, and retain `x < #`
    //             crate::ast::Op::Tilde | crate::ast::Op::ExclamationPoint => {
    //                 self.handle_expr(expr, !is_true)
    //             }
    //             crate::ast::Op::QuestionMark => todo!(),
    //             _ => self.handle_expr(&unary.expr, is_true),
    //         },
    //         Expr::InitListExpr(lst) => self.flatten(&lst.exprs, is_true),
    //         Expr::Ident(ident) => Some(vec![SimpleIdent::new(ident)]),
    //         Expr::Literal(lit) => Some(vec![]),
    //         Expr::LogExpr(log) => self.flatten(&log.args, is_true),
    //         Expr::CallExpr(call) => self.flatten(&call.args, is_true),
    //         Expr::LambdaExpr(lambda) => Some(vec![]),
    //     }
    // }

    // fn get(&mut self, ident: &SimpleIdent) -> Option<&mut Vec<Constraint>> {
    //     self.0.get_mut(ident)
    // }

    // fn flatten(&self, args: &Vec<Expr>, is_true: bool) -> Option<Vec<SimpleIdent>> {
    //     Some(
    //         args.iter()
    //             .flat_map(|expr| self.handle_expr(expr, is_true))
    //             .flatten()
    //             .collect(),
    //     )
    // }
}

fn standardize() {
    //
}
