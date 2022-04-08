use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    iter::zip,
};

pub mod coerce;
pub use coerce::*;

pub mod util;
use tracing::log::debug;
pub use util::*;

pub mod types;
pub use types::*;

pub mod compute_idents;
pub use compute_idents::*;

pub mod check;
pub use check::*;

pub mod stringify;
pub use stringify::*;

use crate::ast::{Expr, Ident, Node, Op, Stmt};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SimpleIdent(String);
impl SimpleIdent {
    fn new(ident: &Ident) -> SimpleIdent {
        SimpleIdent(ident.name.clone())
    }
}

/// Mapping for known variables to known constraints
#[derive(Default, Debug, Clone)]
pub struct ConstraintStack {
    pub constraints: HashMap<SimpleIdent, Vec<Constraint>>,
    scope_record: Vec<(SimpleIdent, usize)>,
    scope_list: Vec<usize>,
    seen_exprs: HashSet<u64>,
    locked: bool,
}

impl ConstraintStack {
    pub fn check(&self, to_match: &Constraint) -> bool {
        true
        // let idents = to_match.find_idents();
        // let idents = idents
        //     .iter()
        //     .map(|ident| self.constraints.get(&SimpleIdent(ident.to_string())));

        // // If not all variables accounted for, can't match
        // if idents.clone().any(|x| x.is_none()) {
        //     return false;
        // }

        // // Verify if constraint met (iter/collect twice--first to dedup, then to get a slice)
        // let constraints_to_check = idents.flatten().flatten().collect::<HashSet<_>>();
        // check(
        //     to_match,
        //     constraints_to_check
        //         .into_iter()
        //         .collect::<Vec<_>>()
        //         .as_slice(),
        // )
    }

    pub fn new_scope(&mut self) {
        debug!("Scope created");
        self.scope_list.push(self.scope_record.len());
    }

    pub fn dirty_scope(&mut self) {
        debug!("Scope dirtied");

        // Verify in-range
        let start = self.get_scope();
        if start > self.scope_record.len() {
            debug!("Start {} out of range for scopes, ignoring", start);
            return;
        }

        // Dirty the constraints in the current scope
        for (ident, ndx) in self.scope_record[start..].iter() {
            if let Some(stack) = self.constraints.get_mut(ident) {
                if let Some(constraint) = stack.get_mut(*ndx) {
                    constraint.guaranteed = false;
                } else {
                    debug!("Unknown constraint number {}, skipping", ndx);
                }
            } else {
                debug!("Unknown ident {:#?}, skipping", ident);
            }
        }

        // Delete the current scope
        self.scope_record.truncate(start);
    }

    pub fn dirty_var(&mut self, ident: &Ident) -> Option<()> {
        debug!("Var {} dirtied", ident.name);
        let constraints = self.constraints.get_mut(&SimpleIdent::new(ident))?;
        for constraint in constraints.iter_mut() {
            constraint.guaranteed = false;
        }
        Some(())
    }

    pub fn clear(&mut self) {
        debug!("Constraints cleared");
        self.constraints.clear();
        self.scope_list.clear();
        self.scope_record.clear();
        self.seen_exprs.clear();
    }

    pub fn push_constraint(&mut self, node: &Node, assert_constraint: bool) -> Option<()> {
        // Verify unique
        if !self.register_seen(node) {
            debug!("Already seen {:?}, ignoring", node);
            return None;
        }

        // Create and verify constraint
        debug!("Pushing constraint");
        let constraint = self.do_push_constraint(node);
        if constraint.is_none() {
            debug!("Error encountered parsing: {:#?}", node);
        }
        let constraint = constraint?;
        if !assert_constraint && !constraint.valid_constraint() {
            println!("Invalid: {}", constraint);
        }
        let list = constraint.find_idents();
        if list.is_empty() {
            debug!("No idents, skipping");
            return None;
        }

        // Store constraint
        debug!("Generated, copying {}x", list.len());
        for ident in list {
            self.record(SimpleIdent(ident.to_string()), constraint.clone());
        }
        debug!("Constraint: {}", constraint);
        debug!(
            "Context contains {} idents and {} constraints",
            self.constraints.len(),
            self.constraints
                .values()
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        );
        debug!("Completed pushing constraint");
        Some(())
    }

    fn do_push_constraint(&mut self, node: &Node) -> Option<ConstraintTree> {
        match node {
            Node::Stmt(stmt) => match stmt {
                Stmt::DeclStmt(decl) => {
                    // Get all variable names as constraints
                    let mut vars = decl
                        .variables
                        .iter()
                        .map(|decl| new_ident(decl.ident.name.clone()))
                        .collect::<Vec<_>>();

                    // Determine which are uninitialized and don't matter
                    let mut remove = vec![];
                    for (i, var) in decl.expressions.iter().enumerate() {
                        if var.is_none() {
                            remove.push(i);
                        }
                    }

                    // Index all nodes to ignore (so don't record later)
                    self.locked = true;
                    for var in remove.iter() {
                        vars.remove(*var);
                        let x: Expr = decl.variables[*var].ident.clone().into();
                        self.do_push_constraint(&x.into());
                    }
                    self.locked = false;

                    // Convert all initialized values to constraints
                    let vals = self.flatten_vec(
                        &decl
                            .expressions
                            .iter()
                            .flat_map(|d| d.clone())
                            .collect::<Vec<_>>(),
                    )?;

                    // Zip variables and initial values into constraints
                    let result = zip(vars.into_iter(), vals.into_iter())
                        .map(|(var_name, var_val)| {
                            RelationalConstraint::equal(var_name, var_val).into()
                        })
                        .collect::<Vec<_>>();

                    // Create final constraint value
                    Some(StructuralConstraint::and(&result).into())
                }
                stmt => {
                    debug!("Unexpected constraint {:#?}", stmt);
                    None
                }
            },
            Node::Expr(expr) => self.handle_expr(expr),
            other => {
                debug!("Unexpected constraint {:#?}", other);
                None
            }
        }
    }

    /// Yes, this returns Option just so I can use `?`. I'm lazy.
    fn handle_expr(&mut self, expr: &Expr) -> Option<ConstraintTree> {
        self.register_seen(expr);
        match expr {
            Expr::AssignExpr(assign) => {
                let lhs_len = assign.lhs.len();
                let rhs_len = assign.rhs.len();

                // Verify type of assign expr in question
                if lhs_len > 1 && rhs_len == 1 {
                    let rhs_constraint = self.flatten(&assign.rhs)?;
                    let full_constraints = self
                        .flatten_vec(&assign.lhs)?
                        .into_iter()
                        .map(|ident| {
                            RelationalConstraint::equal(ident, rhs_constraint.clone()).into()
                        })
                        .collect::<Vec<_>>();

                    Some(StructuralConstraint::and(&*full_constraints).into())
                } else if lhs_len == rhs_len {
                    // Set of assignments
                    let mut result = vec![];
                    for (lhs, rhs) in zip(assign.lhs.iter(), assign.rhs.iter()) {
                        result.push(
                            RelationalConstraint::equal(
                                self.handle_expr(lhs)?,
                                self.handle_expr(rhs)?,
                            )
                            .into(),
                        );
                    }
                    Some(StructuralConstraint::and(&result).into())
                } else {
                    None
                }
            }
            Expr::BinaryExpr(bin) => {
                if let Some(relation) = ConstraintLogic::try_convert(&bin.op) {
                    // This binary expression a logical constraint
                    Some(
                        RelationalConstraint::new(
                            relation,
                            self.handle_expr(&bin.lhs)?,
                            self.handle_expr(&bin.rhs)?,
                        )
                        .into(),
                    )
                } else if let Some(struct_op) = ConstraintComposition::try_convert(&bin.op) {
                    // This binary expression a structural constraint
                    Some(
                        StructuralConstraint::new(
                            struct_op,
                            &[self.handle_expr(&bin.lhs)?, self.handle_expr(&bin.rhs)?],
                        )
                        .into(),
                    )
                } else {
                    // Verify nothing important happening before exit
                    match bin.op {
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
                        | Op::UnsignedShiftRightEqual => {
                            self.dirty_related(&bin.lhs);
                        }
                        _ => { /* No-op */ }
                    }
                    None
                }
            }

            Expr::Ident(ident) => Some(new_ident(ident.name.clone())),
            Expr::Literal(lit) => Some(new_literal(lit.value.clone())),
            Expr::UnaryExpr(unary) => match unary.op {
                // crate::ast::Op::Plus => {} // TODO This is potentially important in numeric comparisons
                // crate::ast::Op::Minus => {} // TODO This is important in numeric comparisons
                crate::ast::Op::PlusPlus | crate::ast::Op::MinusMinus => {
                    // TODO: modify as follows:
                    // PlusPlus: for x, invalidate all `x < #` constraints, change any `x == #` to `x > #`, and retain `x > #`
                    // MinusMinus: for x, invalidate all `x > #` constraints, change any `x == #` to `x < #`, and retain `x < #`
                    self.dirty_related(&unary.expr);
                    None
                }
                // crate::ast::Op::MinusMinus => {}
                crate::ast::Op::Tilde | crate::ast::Op::ExclamationPoint => {
                    Some(StructuralConstraint::not(&[self.handle_expr(&unary.expr)?]).into())
                }
                _ => self.handle_expr(&unary.expr),
            },

            // TODO Make dirty context
            Expr::IncDecExpr(_) => None,
            Expr::CallExpr(call) => {
                // Convert lhs + dirty context for its idents
                let lhs = self.handle_expr(&call.name)?;
                self.dirty_related(&call.name);

                // Convert args + dirty context for its idents
                let args: Vec<Option<_>> =
                    call.args.iter().map(|arg| self.handle_expr(arg)).collect();

                // Verify all worked
                if args.iter().any(|arg| arg.is_none()) {
                    return None;
                }
                let args = args.into_iter().flatten().collect();

                // Return call  structure
                Some(MethodConstraint::new(lhs.into(), args).into())
            }
            Expr::DotExpr(dot) => Some(
                StructuralConstraint::dot(&[
                    self.handle_expr(&dot.selected)?,
                    self.handle_expr(&dot.expr)?,
                ])
                .into(),
            ),
            Expr::ParenExpr(paren) => self.handle_expr(&paren.expr),
            Expr::IndexExpr(index) => Some(
                StructuralConstraint::dot(&[
                    self.handle_expr(&index.expr)?,
                    self.handle_expr(&index.index_expr)?,
                ])
                .into(),
            ),

            // Unused
            // Expr::LogExpr(_)
            // | Expr::LambdaExpr(_)
            // | Expr::SwitchExpr(_)
            // | Expr::InitListExpr(_)
            // | Expr::EndpointCallExpr(_)
            // | Expr::CaseExpr(case) => unhandled
            unhandled => {
                debug!("Unhandled expression {:#?}", unhandled);
                None
            }
        }
    }

    fn flatten_vec(&mut self, data: &[Expr]) -> Option<Vec<ConstraintTree>> {
        // Compute constraints
        let result: Vec<Option<ConstraintTree>> =
            data.iter().map(|expr| self.handle_expr(expr)).collect();

        // Verify all mapped correctly; otherwise yield None
        if !result.iter().any(|x| x.is_none()) {
            Some(result.into_iter().flatten().collect())
        } else {
            None
        }
    }

    fn flatten(&mut self, data: &[Expr]) -> Option<ConstraintTree> {
        let result = self.flatten_vec(data)?;
        let constraints = StructuralConstraint::and(&result.into_iter().collect::<Vec<_>>());
        Some(constraints.into())
    }

    fn record(&mut self, ident: SimpleIdent, constraint: ConstraintTree) {
        if self.locked {
            return;
        }

        // If ident doesn't exist, add it
        if !self.constraints.contains_key(&ident) {
            self.constraints.insert(ident.clone(), vec![]);
        }

        // Record found constraint
        let constraints = self.constraints.get_mut(&ident).unwrap();
        self.scope_record.push((ident.clone(), constraints.len()));
        constraints.push(Constraint::create_constraint(constraint));
    }

    fn dirty_related(&mut self, expr: &Expr) {
        for ident in get_idents(expr).iter() {
            self.dirty_var(ident);
        }
    }

    fn get_scope(&mut self) -> usize {
        self.scope_list.pop().unwrap_or(0)
    }

    fn register_seen<T>(&mut self, hashable: T) -> bool
    where
        T: Hash,
    {
        let hasher = &mut DefaultHasher::new();
        hashable.hash(hasher);
        let hash = hasher.finish();
        !self.seen_exprs.insert(hash)
    }
}
