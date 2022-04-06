use derive_new::new;
use serde::Deserialize;

use crate::ast::Op;

// Top-level: Constraint

/// An actual constraint (logic/structure + whether it should be met, unmet, or possibly met)
#[derive(Debug, Clone, Deserialize, new, Hash, PartialEq, Eq)]
pub struct Constraint {
    #[serde(default)]
    pub truth_value: TernaryBool,
    pub value: ConstraintTree,
}

// Ternary logic

/// Three-way logic
#[derive(derive_more::Display, Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
pub enum TernaryBool {
    True,
    False,
    Unknown,
}
impl TernaryBool {
    /// Negation operation  (True <-> False, Unknown unaffected)
    pub fn negate(&self) -> TernaryBool {
        match self {
            TernaryBool::True => TernaryBool::False,
            TernaryBool::False => TernaryBool::True,
            _ => TernaryBool::Unknown,
        }
    }
}
impl Default for TernaryBool {
    fn default() -> Self {
        TernaryBool::True
    }
}

/// Constraint tree: recursion on either structure or relations

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub enum ConstraintTree {
    VariableConstraint(String),
    LiteralConstraint(String),
    MethodConstraint(MethodConstraint),
    RelationalConstraint(RelationalConstraint),
    StructuralConstraint(StructuralConstraint),
}

/// Factory for an ident
pub fn new_ident(ident: String) -> ConstraintTree {
    ConstraintTree::VariableConstraint(ident)
}

/// Factory for a literal
pub fn new_literal(literal: String) -> ConstraintTree {
    ConstraintTree::LiteralConstraint(literal)
}

impl From<MethodConstraint> for ConstraintTree {
    fn from(c: MethodConstraint) -> ConstraintTree {
        ConstraintTree::MethodConstraint(c)
    }
}
impl From<RelationalConstraint> for ConstraintTree {
    fn from(c: RelationalConstraint) -> Self {
        ConstraintTree::RelationalConstraint(c)
    }
}
impl From<StructuralConstraint> for ConstraintTree {
    fn from(c: StructuralConstraint) -> Self {
        ConstraintTree::StructuralConstraint(c)
    }
}

// Method constraint

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub struct MethodConstraint {
    pub called: Option<Box<Constraint>>,
    pub args: Vec<Constraint>,
}
impl MethodConstraint {
    pub fn new(callee: Option<Constraint>, args: Vec<Constraint>) -> MethodConstraint {
        MethodConstraint {
            called: callee.map(Box::new),
            args,
        }
    }
}

// Structural constraints - indicate how things are linked together in a non-boolean manner

// TODO expand with needed remaining
/// Acknowledged structural relations
#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
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
    Dot,
}

impl ConstraintComposition {
    /// Conversion from operator to structural relation
    pub fn try_convert(value: &Op) -> Option<Self> {
        match value {
            Op::AndAnd | Op::And => Some(ConstraintComposition::And),
            Op::PipePipe | Op::Pipe => Some(ConstraintComposition::Or),
            Op::Plus => Some(ConstraintComposition::Plus),
            Op::Minus => Some(ConstraintComposition::Minus),
            Op::Star => Some(ConstraintComposition::Multiply),
            Op::Slash => Some(ConstraintComposition::Divide),

            // Handle later
            Op::BitShiftLeft => Some(ConstraintComposition::ShiftLeft),
            Op::BitShiftRight => Some(ConstraintComposition::ShiftRight),
            Op::UnsignedBitShiftRight => Some(ConstraintComposition::UnsignedShiftLeft),

            _ => None,
        }
    }

    pub fn reorderable(&self) -> bool {
        matches!(
            self,
            ConstraintComposition::And
                | ConstraintComposition::Or
                | ConstraintComposition::Not
                | ConstraintComposition::Plus
                | ConstraintComposition::Multiply
        )
    }
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
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

    /// Factory for AND structure
    pub fn and(children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::And, children)
    }

    /// Factory for NOT structure
    pub fn not(children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::Not, children)
    }

    pub fn dot(children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::Dot, children)
    }
}

// Logical constraints - indicate boolean information about the relation between the two sides

/// Acknowledged logical relations
#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub enum ConstraintLogic {
    NotEqual,
    Equal,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo,
}

impl ConstraintLogic {
    /// Conversion from operator into structural relation
    pub fn try_convert(value: &Op) -> Option<Self> {
        match value {
            Op::EqualEqual => Some(ConstraintLogic::Equal),
            Op::NotEqual => Some(ConstraintLogic::NotEqual),
            Op::GreaterThan => Some(ConstraintLogic::GreaterThan),
            Op::GreaterThanEqualTo => Some(ConstraintLogic::GreaterThanEqualTo),
            Op::LessThan => Some(ConstraintLogic::LessThan),
            Op::LessThanEqualTo => Some(ConstraintLogic::LessThanEqualTo),
            Op::Equal => Some(ConstraintLogic::Equal),

            // Unknown
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
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

    /// Factory for equal relation
    pub fn equal(lhs: Constraint, rhs: Constraint) -> RelationalConstraint {
        RelationalConstraint::new(ConstraintLogic::Equal, lhs, rhs)
    }
}
