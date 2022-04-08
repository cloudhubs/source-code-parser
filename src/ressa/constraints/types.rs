use derive_new::new;
use serde::Deserialize;

use crate::ast::Op;

// Top-level: Constraint

fn default_true() -> bool {
    true
}

/// An actual constraint (logic/structure + whether it should be met, unmet, or possibly met)
#[derive(Debug, Clone, Deserialize, new, Hash, PartialEq, Eq)]
pub struct Constraint {
    /// Callback executed on success
    #[serde(default)]
    pub callback: Option<String>,

    /// Whether this is essential to matching
    #[serde(default = "default_true")]
    pub essential: bool,

    /// Indicates whether this constraint is required to be met; doubles as the flag
    /// indicating a constraint is guaranteed as met, or possibly not met
    #[serde(default = "default_true")]
    pub guaranteed: bool,

    /// Wrapped constraint
    pub value: ConstraintTree,
}
impl Constraint {
    pub fn create_constraint<T>(value: T) -> Constraint
    where
        T: Into<ConstraintTree>,
    {
        Constraint::new(None, true, true, value.into())
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

impl ConstraintTree {
    pub(crate) fn valid_constraint(&self) -> bool {
        match self {
            ConstraintTree::RelationalConstraint(_) => true,
            ConstraintTree::StructuralConstraint(structural) => {
                structural.children.iter().any(|s| s.valid_constraint())
            }
            _ => false,
        }
    }
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
    pub called: Option<Box<ConstraintTree>>,
    pub args: Vec<ConstraintTree>,
}
impl MethodConstraint {
    pub fn new(callee: Option<ConstraintTree>, args: Vec<ConstraintTree>) -> MethodConstraint {
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
    pub children: Vec<ConstraintTree>,
}

impl StructuralConstraint {
    pub fn new(r#type: ConstraintComposition, children: &[ConstraintTree]) -> StructuralConstraint {
        StructuralConstraint {
            r#type,
            children: children.to_vec(),
        }
    }

    /// Factory for AND structure
    pub fn and(children: &[ConstraintTree]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::And, children)
    }

    /// Factory for NOT structure
    pub fn not(children: &[ConstraintTree]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::Not, children)
    }

    pub fn dot(children: &[ConstraintTree]) -> StructuralConstraint {
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
    pub lhs: Box<ConstraintTree>,
    pub rhs: Box<ConstraintTree>,
}

impl RelationalConstraint {
    pub fn new(
        r#type: ConstraintLogic,
        lhs: ConstraintTree,
        rhs: ConstraintTree,
    ) -> RelationalConstraint {
        RelationalConstraint {
            r#type,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    /// Factory for equal relation
    pub fn equal(lhs: ConstraintTree, rhs: ConstraintTree) -> RelationalConstraint {
        RelationalConstraint::new(ConstraintLogic::Equal, lhs, rhs)
    }
}
