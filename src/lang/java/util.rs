use crate::AST;

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
    let mut buffer = String::new();
    for member in ast.children.iter() {
        buffer.push_str(&*member.value);
    }
    buffer
}

/// Logs an unknown tag was encountered. You better not think too much about that.
pub(crate) fn log_unknown_tag(tag: &str, parent: &str) {
    eprintln!("Unknown tag {} encountered while parsing {}!", tag, parent);
}
