// use crate::ressa::CompiledPattern;

// use super::{
//     Constraint, ConstraintComposition, ConstraintLogic, ConstraintTree, MethodConstraint,
//     RelationalConstraint, StructuralConstraint, VariableConstraint,
// };

// trait FindVars {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str>;
// }

// impl FindVars for Constraint {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         match self.value {
//             ConstraintTree::VariableConstraint(var) => var.find_vars(pattern),
//             ConstraintTree::LiteralConstraint(_) => vec![],
//             ConstraintTree::MethodConstraint(method) => vec![method
//                 .called
//                 .map(|vars| vars.find_vars(pattern))
//                 .unwrap_or(vec![])]
//             .into_iter()
//             .flatten()
//             .collect(),
//             ConstraintTree::RelationalConstraint(_) => todo!(),
//             ConstraintTree::StructuralConstraint(_) => todo!(),
//         }
//     }
// }

// impl FindVars for VariableConstraint {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         todo!()
//     }
// }

// impl FindVars for ConstraintComposition {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         todo!()
//     }
// }

// impl FindVars for ConstraintTree {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         todo!()
//     }
// }

// impl FindVars for StructuralConstraint {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         todo!()
//     }
// }

// impl FindVars for ConstraintLogic {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         todo!()
//     }
// }

// impl FindVars for RelationalConstraint {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         todo!()
//     }
// }

// impl FindVars for MethodConstraint {
//     fn find_vars(&self, pattern: &CompiledPattern) -> Vec<&str> {
//         todo!()
//     }
// }
