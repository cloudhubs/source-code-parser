use std::convert::TryFrom;

use derive_new::new;
use serde::Deserialize;

use crate::ast::Op;

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
impl TernaryBool {
    pub fn negate(&self) -> TernaryBool {
        match self {
            TernaryBool::True => TernaryBool::False,
            TernaryBool::False => TernaryBool::True,
            _ => TernaryBool::Unknown,
        }
    }
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
    LogicalConstraint(RelationalConstraint),
    CompositionConstraint(StructuralConstraint),
}
pub fn new_ident(ident: String) -> ConstraintTree {
    ConstraintTree::VariableConstraint(ident)
}

pub fn new_literal(literal: String) -> ConstraintTree {
    ConstraintTree::LiteralConstraint(literal)
}
impl From<RelationalConstraint> for ConstraintTree {
    fn from(c: RelationalConstraint) -> Self {
        ConstraintTree::LogicalConstraint(c)
    }
}
impl From<StructuralConstraint> for ConstraintTree {
    fn from(c: StructuralConstraint) -> Self {
        ConstraintTree::CompositionConstraint(c)
    }
}

// Structural constraints

// TODO expand with needed remaining
#[derive(Debug, Clone, Deserialize)]
pub enum ConstraintComposition {
    And,
    Or,
    Not,
    Plus,
    Minus,
    Multiply,
    Divide,
    ShiftLeft,
    UnsignedShiftLeft,
    ShiftRight,
}
impl TryFrom<Op> for ConstraintComposition {
    type Error = ();

    fn try_from(value: Op) -> Result<Self, Self::Error> {
        match value {
            Op::AndAnd | Op::And => Ok(ConstraintComposition::And),
            Op::PipePipe | Op::Pipe => Ok(ConstraintComposition::Or),
            Op::Plus => Ok(ConstraintComposition::Plus),
            Op::Minus => Ok(ConstraintComposition::Minus),
            Op::Star => Ok(ConstraintComposition::Multiply),
            Op::Slash => Ok(ConstraintComposition::Divide),

            // Handle later
            Op::BitShiftLeft => Ok(ConstraintComposition::ShiftLeft),
            Op::BitShiftRight => Ok(ConstraintComposition::ShiftRight),
            Op::UnsignedBitShiftRight => Ok(ConstraintComposition::UnsignedShiftLeft),

            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StructuralConstraint {
    pub r#type: ConstraintComposition,
    pub children: Vec<Constraint>,
}

impl StructuralConstraint {
    pub fn new(r#type: ConstraintComposition, children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint {
            r#type,
            children: children.to_vec(),
        }
    }

    pub fn and(children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::And, children)
    }

    pub fn not(children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::Not, children)
    }
}

// Logical constraints

#[derive(Debug, Clone, Deserialize)]
pub enum ConstraintLogic {
    NotEqual,
    Equal,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
}
impl TryFrom<Op> for ConstraintLogic {
    type Error = ();

    fn try_from(value: Op) -> Result<Self, Self::Error> {
        match value {
            Op::EqualEqual => Ok(ConstraintLogic::Equal),
            Op::NotEqual => Ok(ConstraintLogic::NotEqual),
            Op::GreaterThan => Ok(ConstraintLogic::GreaterThan),
            Op::GreaterThanEqualTo => Ok(ConstraintLogic::GreaterThanEqualTo),
            Op::LessThan => Ok(ConstraintLogic::LessThan),
            Op::LessThanEqualTo => Ok(ConstraintLogic::LessThanEqualTo),
            Op::Equal => Ok(ConstraintLogic::Equal),

            // Unknown
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelationalConstraint {
    pub r#type: ConstraintLogic,
    pub lhs: Box<Constraint>,
    pub rhs: Box<Constraint>,
}

impl RelationalConstraint {
    pub fn new(r#type: ConstraintLogic, lhs: Constraint, rhs: Constraint) -> RelationalConstraint {
        RelationalConstraint {
            r#type,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn equal(lhs: Constraint, rhs: Constraint) -> RelationalConstraint {
        RelationalConstraint::new(ConstraintLogic::Equal, lhs, rhs)
    }
}
