use std::collections::HashMap;

pub mod coerce;
pub use coerce::*;

pub mod types;
use tracing::log::warn;
pub use types::*;

use crate::{
    ast::{Expr, Ident, Node, Stmt},
    Language,
};

use super::Indexable;
struct SimpleIdent(String);
impl SimpleIdent {
    fn new(ident: &Ident) -> SimpleIdent {
        SimpleIdent(ident.name.clone())
    }
}

/// Mapping for known variables to known constraints
pub struct ConstraintStack(HashMap<SimpleIdent, Vec<Constraint>>);

impl ConstraintStack {
    fn push_constraint(&mut self, node: &Node) {
        // self.do_push_constraint(node, true);
    }

    // fn do_push_constraint(&mut self, node: &Node, is_true: bool) -> Option<ConstraintTree> {
    //     match node {
    //         // Node::Block(block) => Some(
    //         //     block
    //         //         .nodes
    //         //         .iter()
    //         //         .flat_map(|node| self.do_push_constraint(node, is_true))
    //         //         .flatten()
    //         //         .collect(),
    //         // ),
    //         Node::Stmt(stmt) => match stmt {
    //             Stmt::DeclStmt(decl) => {
    //                 // TODO impl
    //             }
    //             stmt => {
    //                 warn!("Unexpected constraint {:#?}", stmt);
    //                 None
    //             }
    //         },
    //         Node::Expr(expr) => self.handle_expr(expr, is_true),
    //         other => {
    //             warn!("Unexpected constraint {:#?}", other);
    //             None
    //         }
    //     }
    // }

    // /// Yes, this returns Option just so I can use `?`. I'm lazy.
    // fn handle_expr(&mut self, expr: &Expr, is_true: bool) -> Option<ConstraintTree> {
    //     match expr {
    //         Expr::AssignExpr(assign) => {
    //             let lhs_len = assign.lhs.len();
    //             let rhs_len = assign.rhs.len();

    //             // Verify type of assign expr in question
    //             if lhs_len == 1 && rhs_len > 1 {
    //                 let ident = self.handle_expr(&assign.lhs[0], is_true)?;
    //                 let constraints = self.flatten(&assign.rhs, is_true);
    //             } else if lhs_len == rhs_len {
    //                 // Set of assignments
    //                 let mut i = 0;
    //                 while i < lhs_len {
    //                     let x = &assign.lhs[i];
    //                     //
    //                     i += 1;
    //                 }
    //                 None
    //             } else {
    //                 None
    //             }
    //         }
    //         Expr::BinaryExpr(_) => todo!(),
    //         Expr::EndpointCallExpr(_) => todo!(),
    //         Expr::IndexExpr(_) => todo!(),
    //         Expr::ParenExpr(_) => todo!(),
    //         Expr::DotExpr(_) => todo!(),
    //         Expr::IncDecExpr(_) => todo!(),
    //         // Expr::CaseExpr(case) => todo!(),
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

    //         Expr::Ident(ident) => Some(new_ident(ident.name)),
    //         Expr::Literal(lit) => Some(new_literal(lit.value)),

    //         Expr::CallExpr(call) => {
    //             for arg in call.args.iter() {
    //                 // TODO dirty context
    //             }
    //             None
    //         }

    //         // Expr::LogExpr(_) => todo!(),
    //         // Expr::LambdaExpr(_) => todo!(),
    //         // Expr::SwitchExpr(_) => todo!(),
    //         unhandled => {
    //             warn!("Unhandled expression {:#?}", unhandled);
    //             None
    //         }
    //     }
    // }

    // fn get(&mut self, ident: &str) -> Option<&mut Vec<Constraint>> {
    //     self.0.get_mut(ident)
    // }

    // fn flatten(&self, data: &Vec<Expr>, is_true: bool) -> Option<ConstraintTree> {
    //     // Compute constraints
    //     let result: Vec<Option<ConstraintTree>> = data
    //         .iter()
    //         .map(|expr| self.handle_expr(expr, is_true))
    //         .collect();

    //     // Verify all mapped correctly
    //     if result.iter().find(|x| x.is_none()).is_some() {
    //         return None;
    //     }

    //     // Generate results
    //     Some(
    //         CompositionConstraint::new(
    //             ConstraintLogic::And,
    //             result.into_iter().flatten().collect(),
    //         )
    //         .into(),
    //     )
    // }
}

fn standardize() {
    //
}
