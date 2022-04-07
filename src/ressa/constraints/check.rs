use super::{
    Constraint, ConstraintLogic, ConstraintTree, MethodConstraint, RelationalConstraint,
    StructuralConstraint,
};
use z3::{
    ast::{Ast, Bool},
    Config, Context, Solver,
};

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
    constraint: &ConstraintTree,
) -> Option<Box<dyn Ast<'static>>> {
    use ConstraintTree::*;
    match constraint {
        VariableConstraint(ident) => Some(Box::new(Bool::new_const(ctx, ident.clone()))),
        MethodConstraint(method) => convert_method_constraint(ctx, method),
        RelationalConstraint(relation) => convert_relational_constraint(ctx, relation),
        StructuralConstraint(structural) => convert_structural_constraint(ctx, structural),
        LiteralConstraint(lit) => Some(Box::new(z3::ast::String::new_const(ctx, lit.clone()))),
        _ => None,
    }
}

fn convert_method_constraint(
    ctx: &'static Context,
    constraint: &MethodConstraint,
) -> Option<Box<dyn Ast<'static>>> {
    None
}

fn convert_relational_constraint(
    ctx: &'static Context,
    constraint: &RelationalConstraint,
) -> Option<Box<dyn Ast<'static>>> {
    use ConstraintLogic::*;

    let lhs = convert_constraint(ctx, &constraint.lhs)?;
    let rhs = convert_constraint(ctx, &constraint.rhs)?;

    let ast = match constraint.r#type {
        NotEqual => {}
        Equal => {}
        LessThan => {}
        LessThanEqualTo => {}
        GreaterThan => {}
        GreaterThanEqualTo => {}
    };
    None
}

fn convert_structural_constraint(
    ctx: &'static Context,
    constraint: &StructuralConstraint,
) -> Option<Box<dyn Ast<'static>>> {
    None
}
