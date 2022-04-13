use super::{
    Constraint, ConstraintLogic, ConstraintTree, MethodConstraint, RelationalConstraint,
    StructuralConstraint,
};
use downcast_rs::{impl_downcast, Downcast};
use itertools::Itertools;
use z3::{
    ast::{self, Ast},
    Config, Context, SatResult, Solver, SortKind,
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
    //let ctx: &'static mut Context = Box::leak(Box::new(Context::new(&cfg)));
    //let matched = check_inner(&ctx, to_match, candidate);

    // SAFETY: ctx was allocated using Box::new and leaked with Box::leak
    // so we can guarantee the lifetime lives forever and won't be deallocated
    // by other code, so this won't be a double free
    //let _ = unsafe { Box::from_raw(ctx) };

    //matched
    true
}

fn check_inner(ctx: &'static Context, to_match: &Constraint, candidate: &[&Constraint]) -> bool {
    let to_match = match add_constraint(&ctx, &to_match.value) {
        Some(constraint) => constraint,
        _ => return true,
    };
    let bool_to_match = match to_match.downcast_ref::<ast::Bool>() {
        Some(constraint) => constraint,
        _ => return false,
    };
    let candidates: Vec<_> = candidate
        .iter()
        .flat_map(|c| add_constraint(&ctx, &c.value))
        .collect();

    let bool_candidates: Vec<_> = candidates
        .iter()
        .flat_map(|c| {
            // Needs to be Bool to be assertable
            if c.get_sort().kind() != SortKind::Bool {
                None
            } else {
                Some(c)
            }
        })
        .flat_map(|c| c.downcast_ref::<ast::Bool>())
        .collect();

    let solver = Solver::new(ctx);
    solver.assert(&bool_to_match);
    for condition in bool_candidates {
        solver.assert(&condition);
    }
    solver.check() == SatResult::Sat
}

fn add_constraint(ctx: &'static Context, constraint: &ConstraintTree) -> Option<Box<dyn AstExt>> {
    use ConstraintTree::*;
    match constraint {
        VariableConstraint(ident) => add_variable_constraint(ctx, ident),
        MethodConstraint(method) => add_method_constraint(ctx, method),
        RelationalConstraint(relation) => add_relational_constraint(ctx, relation),
        StructuralConstraint(structural) => add_structural_constraint(ctx, structural),
        LiteralConstraint(lit) => add_literal_constraint(ctx, lit),
    }
}

fn add_variable_constraint(ctx: &'static Context, variable: &str) -> Option<Box<dyn AstExt>> {
    // These are currently the same implementation
    add_literal_constraint(ctx, variable)
}

fn add_method_constraint(
    ctx: &'static Context,
    constraint: &MethodConstraint,
) -> Option<Box<dyn AstExt>> {
    match &constraint.called {
        Some(called) => {
            let z3_called = add_constraint(ctx, called)?;
            let name = z3_called.downcast_ref::<ast::String>()?.as_string()?;
            // Not sure how to know the types of these children
            //let args: Vec<_> = constraint
            //    .args
            //    .iter()
            //    .flat_map(|arg| add_constraint(ctx, solver, called))
            //    .collect();
            Some(Box::new(ast::String::new_const(ctx, name)))
        }
        _ => None,
    }
}

fn add_relational_constraint(
    ctx: &'static Context,
    constraint: &RelationalConstraint,
) -> Option<Box<dyn AstExt>> {
    let lhs = add_constraint(ctx, &constraint.lhs)?;
    let rhs = add_constraint(ctx, &constraint.rhs)?;

    match (lhs.get_sort().kind(), rhs.get_sort().kind()) {
        (SortKind::Seq, SortKind::Int) => {
            let lhs_var = lhs.downcast_ref::<ast::String>()?.as_string()?;
            let rhs_int = rhs.downcast_ref::<ast::Int>()?;
            // Check constraint type
            Some(Box::new(ast::Int::new_const(ctx, lhs_var)._eq(rhs_int)))
        }
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
    constraint: &StructuralConstraint,
) -> Option<Box<dyn AstExt>> {
    let z3_children: Vec<_> = constraint
        .children
        .iter()
        .flat_map(|child| add_constraint(ctx, child))
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
            // I'm not totally convinced I can always assume these are ints
            // but I think it makes sense now because if we're using the
            // plus/minus etc operator then the children should be this type
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
            let name: String = z3_children
                .iter()
                .flat_map(|c| c.downcast_ref::<ast::String>())
                .flat_map(ast::String::as_string)
                .intersperse(".".to_owned())
                .collect();
            Some(Box::new(ast::String::new_const(ctx, name)))
        }
        // Ignoring Shifts because Z3 ints do not support them
        _ => None,
    }
}

fn add_literal_constraint(ctx: &'static Context, literal: &str) -> Option<Box<dyn AstExt>> {
    Some(Box::new(ast::String::new_const(ctx, literal)))
}
