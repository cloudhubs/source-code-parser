use std::fmt::Display;

use itertools::Itertools;

use super::{
    Constraint, ConstraintComposition, ConstraintLogic, ConstraintTree, MethodConstraint,
    RelationalConstraint, StructuralConstraint, VariableConstraint,
};

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.guaranteed {
            true => "",
            false => "possibly ",
        };
        f.write_fmt(format_args!("{}{}", prefix, self.value))
    }
}

impl Display for ConstraintComposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            ConstraintComposition::And => "&&",
            ConstraintComposition::Or => "||",
            ConstraintComposition::Not => "!",
            ConstraintComposition::Plus => "+",
            ConstraintComposition::Minus => "-",
            ConstraintComposition::Multiply => "*",
            ConstraintComposition::Divide => "/",
            ConstraintComposition::ShiftLeft => "<<",
            ConstraintComposition::UnsignedShiftLeft => ">>>",
            ConstraintComposition::ShiftRight => ">>",
            ConstraintComposition::Dot => ".",
        };
        f.write_fmt(format_args!("{}", op))
    }
}

impl Display for ConstraintTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            ConstraintTree::VariableConstraint(var) => var.to_string(),
            ConstraintTree::LiteralConstraint(var) => format!("lit({})", var),
            ConstraintTree::MethodConstraint(method) => method.to_string(),
            ConstraintTree::RelationalConstraint(rel) => rel.to_string(),
            ConstraintTree::StructuralConstraint(structural) => structural.to_string(),
        };
        f.write_fmt(format_args!("{}", result))
    }
}

impl Display for VariableConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.pattern))
    }
}

impl Display for StructuralConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = self.r#type.to_string();
        f.write_fmt(format_args!(
            "({})",
            self.children.iter().map(|x| format!("{x}")).join(&op)
        ))
    }
}

impl Display for ConstraintLogic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            ConstraintLogic::NotEqual => "!=",
            ConstraintLogic::Equal => "==",
            ConstraintLogic::LessThan => "<",
            ConstraintLogic::LessThanEqualTo => "<=",
            ConstraintLogic::GreaterThan => ">",
            ConstraintLogic::GreaterThanEqualTo => ">=",
        };
        f.write_fmt(format_args!("{}", result))
    }
}

impl Display for RelationalConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} {} {})", self.lhs, self.r#type, self.rhs))
    }
}

impl Display for MethodConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self.args.iter().map(|x| format!("{x}")).join(", ");
        f.write_fmt(format_args!(
            "{}({})",
            self.called
                .as_ref()
                .map(|called| format!("{}", called))
                .unwrap_or_else(|| "<unnamed>".to_string()),
            args,
        ))
    }
}
