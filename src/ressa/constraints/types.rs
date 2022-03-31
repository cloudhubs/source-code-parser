use derive_new::new;

#[derive(Debug, Clone)]
pub enum Constraint {
    VariableConstraint(String),
    LogicalConstraint(LogicalConstraint),
}

// TODO expand with needed remaining
#[derive(Debug, Clone)]
pub enum ConstraintType {
    And,
    Or,
    Not,
    Equal,
    LessThan,
    LessThanEqualTo,
}

#[derive(Debug, Clone, new)]
pub struct LogicalConstraint {
    pub r#type: ConstraintType,
    pub children: Vec<Constraint>,
}
