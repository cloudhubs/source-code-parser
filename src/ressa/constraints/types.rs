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
    VariableConstraint(String),
    LogicalConstraint(LogicalConstraint),
}

// TODO expand with needed remaining
#[derive(Debug, Clone)]
pub enum ConstraintComposition {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone)]
pub enum ConstraintLogic {
    NotEqual,
    Equal,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
}

#[derive(Debug, Clone, new)]
pub struct CompositionConstraint {
    pub r#type: ConstraintComposition,
    pub children: Vec<Constraint>,
}

#[derive(Debug, Clone, new)]
pub struct LogicalConstraint {
    pub r#type: ConstraintLogic,
    pub lhs: Box<Constraint>,
    pub rhs: Box<Constraint>,
}
