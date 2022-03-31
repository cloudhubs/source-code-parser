use derive_new::new;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, new)]
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
impl From<bool> for TernaryBool {
    fn from(boolean: bool) -> Self {
        if boolean {
            TernaryBool::True
        } else {
            TernaryBool::False
        }
    }
}

impl Default for TernaryBool {
    fn default() -> Self {
        TernaryBool::True
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ConstraintTree {
    VariableConstraint(String),
    LiteralConstraint(String),
    LogicalConstraint(LogicalConstraint),
    CompositionConstraint(CompositionConstraint),
}
pub fn new_ident(ident: String) -> ConstraintTree {
    ConstraintTree::VariableConstraint(ident)
}

pub fn new_literal(literal: String) -> ConstraintTree {
    ConstraintTree::LiteralConstraint(literal)
}
impl From<LogicalConstraint> for ConstraintTree {
    fn from(c: LogicalConstraint) -> Self {
        ConstraintTree::LogicalConstraint(c)
    }
}
impl From<CompositionConstraint> for ConstraintTree {
    fn from(c: CompositionConstraint) -> Self {
        ConstraintTree::CompositionConstraint(c)
    }
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
