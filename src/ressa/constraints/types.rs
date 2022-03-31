use derive_new::new;

#[derive(Debug, Clone)]
pub struct Constraint {
    pub truth_value: TernaryBool,
    pub value: ConstraintTree,
}

#[derive(Debug, Clone)]
pub enum TernaryBool {
    True,
    False,
    Undefined,
}

#[derive(Debug, Clone)]
pub enum ConstraintTree {
    VariableConstraint(pub String),
    LogicalConstraint(pub LogicalConstraint),
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
