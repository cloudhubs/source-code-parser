use std::collections::HashMap;

pub mod coerce;
pub use coerce::*;

use crate::{
    ast::{Expr, Ident},
    Language,
};

use super::Indexable;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct SimpleIdent(String);
impl SimpleIdent {
    fn new(name: &str, lang: Language) -> SimpleIdent {
        SimpleIdent(format!("{}#{}", lang, name))
    }

    fn copy_ident(ident: &Ident) -> SimpleIdent {
        SimpleIdent::new(&ident.name, ident.language)
    }
}

/// Mapping for known variables to known constraints
pub struct ConstraintStack(HashMap<SimpleIdent, Vec<Constraint>>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Constraint {
    pub expr: Expr,
    pub value: bool,
}

impl ConstraintStack {
    fn push_constraint(&mut self, expr: &Expr) {
        self.do_push_constraint(expr, true);
    }

    /// Yes, this returns Option just so I can use `?`. I'm lazy.
    fn do_push_constraint(&mut self, expr: &Expr, is_true: bool) -> Option<Vec<SimpleIdent>> {
        match expr {
            Expr::AssignExpr(assign) => {
                let lhs_len = assign.lhs.len();
                let rhs_len = assign.rhs.len();

                // Verify type of assign expr in question
                if lhs_len == 1 && rhs_len > 1 {
                    // Tuple assign assumed
                    let lhs = &assign.lhs[0];
                    let stack = self.get("wa", lhs.get_lang())?;
                } else if lhs_len == rhs_len {
                    // Set of assignments
                    let mut i = 0;
                    while i < lhs_len {
                        let x = &assign.lhs[i];
                        //
                        i += 1;
                    }
                }
            }
            Expr::BinaryExpr(_) => todo!(),
            Expr::EndpointCallExpr(_) => todo!(),
            Expr::IndexExpr(_) => todo!(),
            Expr::ParenExpr(_) => todo!(),
            Expr::DotExpr(_) => todo!(),
            Expr::IncDecExpr(_) => todo!(),
            // Expr::CaseExpr(case) => todo!(),
            Expr::SwitchExpr(switch) => switch.cases.iter().map(f),

            Expr::UnaryExpr(unary) => match unary.op {
                // TODO implement the following
                // crate::ast::Op::Plus => {} // TODO This is potentially important in numeric comparisons
                // crate::ast::Op::Minus => {} // TODO This is important in numeric comparisons
                // crate::ast::Op::PlusPlus => {} // TODO: for x, invalidate all `x < #` constraints, change any `x == #` to `x > #`, and retain `x > #`
                // crate::ast::Op::MinusMinus => {} // TODO: for x, invalidate all `x > #` constraints, change any `x == #` to `x < #`, and retain `x < #`
                crate::ast::Op::Tilde | crate::ast::Op::ExclamationPoint => {
                    self.do_push_constraint(expr, !is_true)
                }
                crate::ast::Op::QuestionMark => todo!(),
                _ => self.do_push_constraint(&unary.expr, is_true),
            },
            Expr::InitListExpr(lst) => self.flatten(&lst.exprs, is_true),
            Expr::Ident(ident) => Some(vec![SimpleIdent::copy_ident(ident)]),
            Expr::Literal(lit) => Some(vec![]),
            Expr::LogExpr(log) => self.flatten(&log.args, is_true),
            Expr::CallExpr(call) => self.flatten(&call.args, is_true),
            Expr::LambdaExpr(lambda) => Some(vec![]),
        }
    }

    fn get(&mut self, name: &str, lang: Language) -> Option<&mut Vec<Constraint>> {
        self.0.get_mut(&SimpleIdent::new(name, lang))
    }

    fn flatten(&self, args: &Vec<Expr>, is_true: bool) -> Option<Vec<SimpleIdent>> {
        Some(
            args.iter()
                .flat_map(|expr| self.do_push_constraint(expr, is_true))
                .flatten()
                .collect(),
        )
    }
}

fn standardize() {
    //
}
