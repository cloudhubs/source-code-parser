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

fn add_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &ConstraintTree,
) -> Option<Box<dyn AstExt>> {
    use ConstraintTree::*;
    match constraint {
        VariableConstraint(ident) => add_variable_constraint(ctx, solver, ident), //Some(Box::new(Bool::new_const(ctx, ident.clone()))),
        MethodConstraint(method) => add_method_constraint(ctx, solver, method),
        RelationalConstraint(relation) => add_relational_constraint(ctx, solver, relation),
        StructuralConstraint(structural) => add_structural_constraint(ctx, solver, structural),
        LiteralConstraint(lit) => add_literal_constraint(ctx, solver, lit), //Some(Box::new(ast::String::new_const(ctx, lit.clone()))),
    }
}

fn add_variable_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &str,
) -> Option<Box<dyn AstExt>> {
    None
}

fn add_method_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &MethodConstraint,
) -> Option<Box<dyn AstExt>> {
    None
}

fn add_relational_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &RelationalConstraint,
) -> Option<Box<dyn AstExt>> {
    let lhs = add_constraint(ctx, solver, &constraint.lhs)?;
    let rhs = add_constraint(ctx, solver, &constraint.rhs)?;

    match (lhs.get_sort().kind(), rhs.get_sort().kind()) {
        (SortKind::Int, SortKind::Int) => {
            use ConstraintLogic::*;
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
            // solver.assert(&c);
            Some(Box::new(c))
        }
        (lhs_sort, rhs_sort) => {
            println!("Sort: {lhs_sort:?}, {rhs_sort:?}");
            None
        }
    }
}

fn add_structural_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &StructuralConstraint,
) -> Option<Box<dyn AstExt>> {
    let z3_children: Vec<_> = constraint
        .children
        .iter()
        .flat_map(|child| add_constraint(ctx, solver, child))
        .collect();

    use super::ConstraintComposition::*;
    match constraint.r#type {
        And | Or => {
            let bool_children: Vec<_> = z3_children
                .iter()
                .flat_map(|c| c.downcast_ref::<ast::Bool>())
                .collect();
            let op = match &constraint.r#type {
                And => ast::Bool::and,
                Or => ast::Bool::or,
                _ => unreachable!(),
            };
            Some(Box::new(op(ctx, &bool_children)))
        }
        Not => match z3_children.first() {
            Some(child) => {
                let bool_child = child.downcast_ref::<ast::Bool>()?;
                Some(Box::new(bool_child.not()))
            }
            _ => None,
        },
        Plus | Minus | Multiply => {
            // I just realized I don't think I can cast the children to ints
            // since practically thing will end up as a tree of children
            // realistically rather than a linear list of the ints..
            let int_children: Vec<_> = z3_children
                .iter()
                .flat_map(|c| c.downcast_ref::<ast::Int>())
                .collect();
            let op = match &constraint.r#type {
                Plus => ast::Int::add,
                Minus => ast::Int::sub,
                Multiply => ast::Int::mul,
                _ => unreachable!(),
            };
            Some(Box::new(op(ctx, &int_children)))
        }
        Dot => {
            let _ = 0;
            None
        }
        // Ignoring Shifts because Z3 ints do not support them
        _ => None,
    }
}

fn add_literal_constraint(
    ctx: &'static Context,
    solver: &'static Solver,
    constraint: &str,
) -> Option<Box<dyn AstExt>> {
    None
}
