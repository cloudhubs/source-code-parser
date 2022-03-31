use derive_new::new;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Constraint {
    #[serde(default)]
    pub truth_value: TernaryBool,
    pub value: ConstraintTree,
}

#[derive(Debug, Clone, Deserialize)]
pub enum TernaryBool {
    True,
    False,
    Unknown,
}

impl Default for TernaryBool {
    fn default() -> Self {
        TernaryBool::True
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ConstraintTree {
    VariableConstraint(String),
    LogicalConstraint(LogicalConstraint),
}

// TODO expand with needed remaining
#[derive(Debug, Clone, Deserialize)]
pub enum ConstraintComposition {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ConstraintLogic {
    NotEqual,
    Equal,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
}

#[derive(Debug, Clone, new, Deserialize)]
pub struct CompositionConstraint {
    pub r#type: ConstraintComposition,
    pub children: Vec<Constraint>,
}

#[derive(Debug, Clone, new, Deserialize)]
pub struct LogicalConstraint {
    pub r#type: ConstraintLogic,
    pub lhs: Box<Constraint>,
    pub rhs: Box<Constraint>,
}
