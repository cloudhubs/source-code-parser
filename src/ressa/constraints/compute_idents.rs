use std::collections::HashSet;

use super::{
    Constraint, ConstraintTree, MethodConstraint, RelationalConstraint, StructuralConstraint,
};

pub trait ComputeIdent {
    fn find_idents(&self) -> HashSet<&str>;
}

impl ComputeIdent for Constraint {
    fn find_idents(&self) -> HashSet<&str> {
        self.value.find_idents()
    }
}

impl ComputeIdent for ConstraintTree {
    fn find_idents(&self) -> HashSet<&str> {
        match &self {
            ConstraintTree::VariableConstraint(var) => {
                let mut set = HashSet::new();
                set.insert(var.as_str());
                set
            }
            ConstraintTree::LiteralConstraint(_) => HashSet::new(),
            ConstraintTree::RelationalConstraint(rel) => rel.find_idents(),
            ConstraintTree::StructuralConstraint(structural) => structural.find_idents(),
            ConstraintTree::MethodConstraint(method) => method.find_idents(),
        }
    }
}

impl ComputeIdent for RelationalConstraint {
    fn find_idents(&self) -> HashSet<&str> {
        let mut result = self.lhs.find_idents();
        result.extend(self.rhs.find_idents().iter());
        result
    }
}

impl ComputeIdent for StructuralConstraint {
    fn find_idents(&self) -> HashSet<&str> {
        self.children
            .iter()
            .flat_map(|child| child.find_idents())
            .collect()
    }
}

impl ComputeIdent for MethodConstraint {
    fn find_idents(&self) -> HashSet<&str> {
        self.args
            .iter()
            .flat_map(|constraint| constraint.find_idents())
            .chain(
                self.called
                    .iter()
                    .flat_map(|callee| callee.find_idents())
                    .into_iter(),
            )
            .collect()
    }
}
