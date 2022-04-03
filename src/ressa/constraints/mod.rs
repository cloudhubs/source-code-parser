use std::{collections::HashMap, iter::zip};

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
    scope_record: Vec<(SimpleIdent, usize)>,
    scope_list: Vec<usize>,
}

impl ConstraintStack {
    pub fn check(&self, constraint: &StructuralConstraint) -> bool {
        true
    }

    pub fn new_scope(&mut self) {
        self.scope_list.push(self.scope_record.len());
    }

    pub fn dirty_scope(&mut self) {
        // Verify in-range (default 0: if no scopes made, everything in-scope)
        let start = *self.scope_list.last().unwrap_or(&0);
        if start > self.scope_record.len() {
            warn!("Start {} out of range for scopes, ignoring", start);
            self.scope_list.pop();
            return;
        }

        // Skip to the scope's start
        let mut iter = self.scope_record.iter();
        let mut i = 0;
        while i < start {
            iter.next();
            i += 1;
        }

        // Dirty the constraints in the scope
        for (ident, ndx) in iter {
            if let Some(stack) = self.constraints.get_mut(ident) {
                if let Some(constraint) = stack.get_mut(*ndx) {
                    constraint.truth_value = TernaryBool::Unknown;
                } else {
                    warn!("Unknown constraint number {}, skipping", ndx);
                }
            } else {
                warn!("Unknown ident {:#?}, skipping", ident);
            }
        }

        // Clear out the old scope
        self.scope_list.pop();
        self.scope_record.truncate(start);
    }

    pub fn dirty_var(&mut self, ident: Ident) -> Option<()> {
        let constraints = self.constraints.get_mut(&SimpleIdent::new(&ident))?;
        for constraint in constraints.iter_mut() {
            constraint.truth_value = TernaryBool::Unknown;
        }
        Some(())
    }

    pub fn clear(&mut self) {
        self.constraints.clear();
        self.scope_list.clear();
        self.scope_record.clear();
    }

    pub fn push_constraint(&mut self, node: &Node) -> Option<()> {
        let constraint = self.do_push_constraint(node, TernaryBool::True)?;
        for ident in constraint.find_affected_idents() {
            self.record(SimpleIdent(ident.to_string()), constraint.clone());
        }
        Some(())
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
                    let mut vars = decl
                        .variables
                        .iter()
                        .map(|decl| Constraint::new(is_true, new_ident(decl.ident.name.clone())))
                        .collect::<Vec<_>>();

                    // Determine which are uninitialized and don't matter
                    for (i, var) in decl.expressions.iter().enumerate() {
                        if var.is_none() {
                            vars.remove(i);
                        }
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
    fn handle_expr(&self, expr: &Expr, is_true: TernaryBool) -> Option<Constraint> {
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
                        RelationalConstraint::equal(ident, constraints).into(),
                    ))
                } else if lhs_len == rhs_len {
                    // Set of assignments
                    let mut result = vec![];
                    for (lhs, rhs) in zip(assign.lhs.iter(), assign.rhs.iter()) {
                        result.push(Constraint::new(
                            is_true,
                            RelationalConstraint::equal(
                                self.handle_expr(lhs, is_true)?,
                                self.handle_expr(rhs, is_true)?,
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
                if let Some(relation) = ConstraintLogic::try_convert(&bin.op) {
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
                } else if let Some(struct_op) = ConstraintComposition::try_convert(&bin.op) {
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

            Expr::Ident(ident) => Some(Constraint::new(is_true, new_ident(ident.name.clone()))),
            Expr::Literal(lit) => Some(Constraint::new(is_true, new_literal(lit.value.clone()))),
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
                for _arg in call.args.iter() {
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

    fn flatten_vec(&self, data: &[Expr], is_true: TernaryBool) -> Option<Vec<Constraint>> {
        // Compute constraints
        let result: Vec<Option<Constraint>> = data
            .iter()
            .map(|expr| self.handle_expr(expr, is_true))
            .collect();

        // Verify all mapped correctly; otherwise yield None
        if !result.iter().any(|x| x.is_none()) {
            Some(result.into_iter().flatten().collect())
        } else {
            None
        }
    }

    fn flatten(&self, data: &[Expr], is_true: TernaryBool) -> Option<Constraint> {
        let result = self.flatten_vec(data, is_true)?;
        let constraints = StructuralConstraint::and(&result.into_iter().collect::<Vec<_>>()).into();
        Some(Constraint::new(is_true, constraints))
    }

    fn record(&mut self, ident: SimpleIdent, constraint: Constraint) -> Option<()> {
        let constraints = self.constraints.get_mut(&ident)?;
        self.scope_record.push((ident, constraints.len()));
        constraints.push(constraint);
        Some(())
    }
}
