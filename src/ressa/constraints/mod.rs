use std::{collections::HashMap, convert::TryInto, iter::zip};

pub mod coerce;
pub use coerce::*;

pub mod types;
use tracing::log::warn;
pub use types::*;

use crate::ast::{Expr, Ident, Node, Op, Stmt};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SimpleIdent(String);
impl SimpleIdent {
    fn new(ident: &Ident) -> SimpleIdent {
        SimpleIdent(ident.name.clone())
    }
}

/// Mapping for known variables to known constraints
#[derive(Default, Debug, Clone)]
pub struct ConstraintStack {
    constraints: HashMap<SimpleIdent, Vec<Constraint>>,
}

impl ConstraintStack {
    pub fn check(&self, constraint: &StructuralConstraint) -> bool {
        true
    }

    pub fn push_constraint(&mut self, node: &Node) {
        self.do_push_constraint(node, TernaryBool::True);
    }

    fn do_push_constraint(&mut self, node: &Node, is_true: TernaryBool) -> Option<Constraint> {
        match node {
            // Node::Block(block) => Some(
            //     block
            //         .nodes
            //         .iter()
            //         .flat_map(|node| self.do_push_constraint(node, is_true))
            //         .flatten()
            //         .collect(),
            // ),
            Node::Stmt(stmt) => match stmt {
                Stmt::DeclStmt(decl) => {
                    // Get all variable names as constraints
                    let vars = decl
                        .variables
                        .iter()
                        .map(|decl| Constraint::new(is_true, new_ident(decl.ident.name)))
                        .collect::<Vec<Constraint>>();

                    // Determine which are uninitialized and don't matter
                    let mut i = 0;
                    for var in decl.expressions.iter() {
                        if var.is_none() {
                            vars.remove(i);
                        }
                        i += 1;
                    }

                    // Convert all initialized values to constraints
                    let vals = self.flatten_vec(
                        &decl
                            .expressions
                            .iter()
                            .flat_map(|d| d.clone())
                            .collect::<Vec<_>>(),
                        is_true,
                    )?;

                    // Zip variables and initial values into constraints
                    let result = zip(vars.into_iter(), vals.into_iter())
                        .map(|(var_name, var_val)| {
                            Constraint::new(
                                is_true,
                                RelationalConstraint::equal(var_name, var_val).into(),
                            )
                        })
                        .collect::<Vec<_>>();

                    // Create final constraint value
                    Some(Constraint::new(
                        is_true,
                        StructuralConstraint::and(&result).into(),
                    ))
                }
                stmt => {
                    warn!("Unexpected constraint {:#?}", stmt);
                    None
                }
            },
            Node::Expr(expr) => self.handle_expr(expr, is_true),
            other => {
                warn!("Unexpected constraint {:#?}", other);
                None
            }
        }
    }

    /// Yes, this returns Option just so I can use `?`. I'm lazy.
    fn handle_expr(&mut self, expr: &Expr, is_true: TernaryBool) -> Option<Constraint> {
        match expr {
            Expr::AssignExpr(assign) => {
                let lhs_len = assign.lhs.len();
                let rhs_len = assign.rhs.len();

                // Verify type of assign expr in question
                if lhs_len == 1 && rhs_len > 1 {
                    let ident = self.handle_expr(&assign.lhs[0], is_true)?;
                    let constraints = self.flatten(&assign.rhs, is_true)?;
                    Some(Constraint::new(
                        is_true,
                        RelationalConstraint::equal(ident.into(), constraints.into()).into(),
                    ))
                } else if lhs_len == rhs_len {
                    // Set of assignments
                    let mut result = vec![];
                    for (lhs, rhs) in zip(assign.lhs.iter(), assign.rhs.iter()) {
                        result.push(Constraint::new(
                            is_true,
                            RelationalConstraint::equal(
                                self.handle_expr(lhs, is_true)?.into(),
                                self.handle_expr(rhs, is_true)?.into(),
                            )
                            .into(),
                        ));
                    }
                    Some(Constraint::new(
                        is_true,
                        StructuralConstraint::and(&result).into(),
                    ))
                } else {
                    None
                }
            }
            Expr::BinaryExpr(bin) => {
                if let Ok(relation) = bin.op.try_into() {
                    // This binary expression a logical constraint
                    Some(Constraint::new(
                        is_true,
                        RelationalConstraint::new(
                            relation,
                            self.handle_expr(&bin.lhs, is_true)?,
                            self.handle_expr(&bin.rhs, is_true)?,
                        )
                        .into(),
                    ))
                } else if let Ok(struct_op) = bin.op.try_into() {
                    // This binary expression a structural constraint
                    Some(Constraint::new(
                        is_true,
                        StructuralConstraint::new(
                            struct_op,
                            &[
                                self.handle_expr(&bin.lhs, is_true)?,
                                self.handle_expr(&bin.rhs, is_true)?,
                            ],
                        )
                        .into(),
                    ))
                } else {
                    // Verify nothing important happening before exit
                    match bin.op {
                        // TODO make dirty context
                        // Op::PlusPlus
                        // | Op::MinusMinus
                        Op::PlusEqual
                        | Op::MinusEqual
                        | Op::StarEqual
                        | Op::SlashEqual
                        | Op::ModulusEqual
                        | Op::AndEqual
                        | Op::PipeEqual
                        | Op::CaratEqual
                        | Op::TildeEqual
                        | Op::ShiftLeftEqual
                        | Op::ShiftRightEqual
                        | Op::UnsignedShiftRightEqual => { /* Dirty context */ }
                        _ => { /* No-op */ }
                    }
                    None
                }
            }

            Expr::Ident(ident) => Some(Constraint::new(is_true, new_ident(ident.name))),
            Expr::Literal(lit) => Some(Constraint::new(is_true, new_literal(lit.value))),
            Expr::UnaryExpr(unary) => match unary.op {
                // TODO implement the following
                // crate::ast::Op::Plus => {} // TODO This is potentially important in numeric comparisons
                // crate::ast::Op::Minus => {} // TODO This is important in numeric comparisons
                // crate::ast::Op::PlusPlus => {} // TODO: for x, invalidate all `x < #` constraints, change any `x == #` to `x > #`, and retain `x > #`
                // crate::ast::Op::MinusMinus => {} // TODO: for x, invalidate all `x > #` constraints, change any `x == #` to `x < #`, and retain `x < #`
                crate::ast::Op::Tilde | crate::ast::Op::ExclamationPoint => Some(Constraint::new(
                    is_true,
                    StructuralConstraint::not(&[self.handle_expr(expr, is_true.negate())?]).into(),
                )),
                _ => self.handle_expr(&unary.expr, is_true),
            },

            // TODO Make dirty context
            Expr::IncDecExpr(_) => None,
            Expr::CallExpr(call) => {
                for arg in call.args.iter() {
                    // TODO dirty context
                }
                None
            }

            // Unused
            // Expr::LogExpr(_)
            // | Expr::LambdaExpr(_)
            // | Expr::SwitchExpr(_)
            // | Expr::InitListExpr(_)
            // | Expr::IndexExpr(_)
            // | Expr::ParenExpr(_)
            // | Expr::EndpointCallExpr(_)
            // | Expr::DotExpr(_)
            // | Expr::CaseExpr(case) => unhandled
            unhandled => {
                warn!("Unhandled expression {:#?}", unhandled);
                None
            }
        }
    }

    fn get(&mut self, ident: &str) -> Option<&mut Vec<Constraint>> {
        self.constraints.get_mut(&SimpleIdent(ident.into()))
    }

    fn flatten_vec(&self, data: &[Expr], is_true: TernaryBool) -> Option<Vec<Constraint>> {
        // Compute constraints
        let result: Vec<Option<Constraint>> = data
            .iter()
            .map(|expr| self.handle_expr(expr, is_true))
            .collect();

        // Verify all mapped correctly; otherwise yield None
        if result.iter().find(|x| x.is_none()).is_none() {
            Some(result.into_iter().flatten().collect())
        } else {
            None
        }
    }

    fn flatten(&self, data: &[Expr], is_true: TernaryBool) -> Option<Constraint> {
        // Compute constraints
        let result = self.flatten_vec(data, is_true);
        if result.is_none() {
            return None;
        }

        // Generate results
        let comp =
            StructuralConstraint::and(&result.into_iter().flatten().collect::<Vec<_>>()).into();
        Some(Constraint::new(is_true, comp))
    }
}

fn standardize() {
    //
}
