use itertools::Itertools;

use crate::AST;

pub(crate) mod parameter;
pub(crate) mod vartype;

/// Convert a vector into an Option. If the vector is empty, swaps it out for None; otherwise is Some(vector)
pub(crate) fn fold_vec<T>(vector: Vec<T>) -> Option<Vec<T>> {
    if !vector.is_empty() {
        Some(vector)
    } else {
        None
    }
}

/// Convert the children of a provided tree into a single, consecutive string
pub(crate) fn stringify_tree_children(ast: &AST) -> String {
    ast.children.iter().map(|child| &*child.value).join("")
}
