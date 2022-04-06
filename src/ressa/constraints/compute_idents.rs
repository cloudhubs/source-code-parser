use super::{
    Constraint, ConstraintTree, MethodConstraint, RelationalConstraint, StructuralConstraint,
};

pub trait ComputeIdent {
    fn find_idents(&self) -> Vec<&str>;
}

impl ComputeIdent for Constraint {
    fn find_idents(&self) -> Vec<&str> {
        match &self.value {
            ConstraintTree::VariableConstraint(var) => vec![var],
            ConstraintTree::LiteralConstraint(_) => vec![],
            ConstraintTree::RelationalConstraint(rel) => rel.find_idents(),
            ConstraintTree::StructuralConstraint(structural) => structural.find_idents(),
            ConstraintTree::MethodConstraint(method) => method.find_idents(),
        }
    }
}

impl ComputeIdent for RelationalConstraint {
    fn find_idents(&self) -> Vec<&str> {
        let mut result = self.lhs.find_idents();
        result.append(&mut self.rhs.find_idents());
        result
    }
}

impl ComputeIdent for StructuralConstraint {
    fn find_idents(&self) -> Vec<&str> {
        self.children
            .iter()
            .flat_map(|child| child.find_idents())
            .collect()
    }
}

impl ComputeIdent for MethodConstraint {
    fn find_idents(&self) -> Vec<&str> {
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
