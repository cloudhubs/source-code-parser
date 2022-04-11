use std::any::Any;

use super::{
    Constraint, ConstraintLogic, ConstraintTree, MethodConstraint, RelationalConstraint,
    StructuralConstraint,
};
use downcast_rs::{impl_downcast, Downcast};
use z3::{
    ast::{self, Ast, Bool},
    Config, Context, Solver, SortKind,
};

trait AstExt: Ast<'static> + Downcast {}
impl_downcast!(AstExt);

macro_rules! impl_dynamic_ast {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl AstExt for $struct_name {}
        )*
    };
}

impl_dynamic_ast!(
    ast::Bool<'static>,
    ast::Int<'static>,
    ast::String<'static>,
    ast::Float<'static>
);

pub fn check(to_match: &Constraint, candidate: &[&Constraint]) -> bool {
    let cfg = Config::new();
    let ctx = Box::leak(Box::new(Context::new(&cfg)));
    let solver = Solver::new(&ctx);

    // We must drop Solver early so that we can drop ctx
    drop(solver);
    // SAFETY: ctx was allocated using Box::new and leaked with Box::leak
    // so we can guarantee the lifetime lives forever and won't be deallocated
    // by other code, so this won't be a double free
    let _ = unsafe { Box::from_raw(ctx) };
    true
}

fn convert_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &ConstraintTree,
) -> Option<Box<dyn AstExt>> {
    use ConstraintTree::*;
    match constraint {
        VariableConstraint(ident) => Some(Box::new(Bool::new_const(ctx, ident.clone()))),
        MethodConstraint(method) => convert_method_constraint(ctx, solver, method),
        RelationalConstraint(relation) => convert_relational_constraint(ctx, solver, relation),
        StructuralConstraint(structural) => convert_structural_constraint(ctx, solver, structural),
        LiteralConstraint(lit) => Some(Box::new(ast::String::new_const(ctx, lit.clone()))),
        _ => None,
    }
}

fn convert_method_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &MethodConstraint,
) -> Option<Box<dyn AstExt>> {
    None
}

fn convert_relational_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &RelationalConstraint,
) -> Option<Box<dyn AstExt>> {
    use ConstraintLogic::*;

    let lhs = convert_constraint(ctx, solver, &constraint.lhs)?;
    let rhs = convert_constraint(ctx, solver, &constraint.rhs)?;

    match (lhs.get_sort().kind(), rhs.get_sort().kind()) {
        (SortKind::Int, SortKind::Int) => {
            let lhs_int = lhs.downcast_ref::<ast::Int>()?;
            let rhs_int = lhs.downcast_ref::<ast::Int>()?;
            let c = match constraint.r#type {
                NotEqual => lhs_int._eq(rhs_int).not(),
                Equal => lhs_int._eq(rhs_int),
                LessThan => lhs_int.lt(rhs_int),
                LessThanEqualTo => lhs_int.le(rhs_int),
                GreaterThan => lhs_int.gt(rhs_int),
                GreaterThanEqualTo => lhs_int.ge(rhs_int),
            };
            solver.assert(&c);
        }
        _ => {}
    };

    None
}

fn convert_structural_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &StructuralConstraint,
) -> Option<Box<dyn AstExt>> {
    None
}
