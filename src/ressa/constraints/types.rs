use derive_new::new;
use serde::Deserialize;

use crate::ast::Op;

// Top-level: Constraint

/// An actual constraint (logic/structure + whether it should be met, unmet, or possibly met)
#[derive(Debug, Clone, Deserialize, new)]
pub struct Constraint {
    #[serde(default)]
    pub truth_value: TernaryBool,
    pub value: ConstraintTree,
}

impl Constraint {
    /// Quick access to all idents this constriant touches on
    pub fn find_affected_idents(&self) -> Vec<&str> {
        match &self.value {
            ConstraintTree::VariableConstraint(var) => vec![var],
            ConstraintTree::LiteralConstraint(_) => vec![],
            ConstraintTree::RelationalConstraint(rel) => {
                let mut result = rel.lhs.find_affected_idents();
                result.append(&mut rel.rhs.find_affected_idents());
                result
            }
            ConstraintTree::StructuralConstraint(structural) => structural
                .children
                .iter()
                .flat_map(|child| child.find_affected_idents())
                .collect(),
        }
    }
}

// Ternary logic

/// Three-way logic
#[derive(Debug, Clone, Copy, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub enum ConstraintTree {
    VariableConstraint(String),
    LiteralConstraint(String),
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

// Structural constraints - indicate how things are linked together in a non-boolean manner

// TODO expand with needed remaining
/// Acknowledged structural relations
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

    /// Factory for AND structure
    pub fn and(children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::And, children)
    }

    /// Factory for NOT structure
    pub fn not(children: &[Constraint]) -> StructuralConstraint {
        StructuralConstraint::new(ConstraintComposition::Not, children)
    }
}

// Logical constraints - indicate boolean information about the relation between the two sides

/// Acknowledged logical relations
#[derive(Debug, Clone, Deserialize)]
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

    /// Factory for equal relation
    pub fn equal(lhs: Constraint, rhs: Constraint) -> RelationalConstraint {
        RelationalConstraint::new(ConstraintLogic::Equal, lhs, rhs)
    }
}
