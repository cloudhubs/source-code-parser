use crate::go::util::stringify_tree_children;
use crate::AST;

/// Indicates when no type is found
pub const NO_NAME: &str = "N/A";

pub(crate) fn parse_identifier(ast: &AST) -> String {
    let name_node = match ast.find_child_by_type(&[
        "package_identifier",
        "field_identifier",
        "identifier"]) {
        Some(node) => node,
        None => ast,
    };

    let name_str = match &*name_node.r#type {
        "package_identifier"
        | "field_identifier"
        | "identifier" => name_node.value.clone(),

        _ => NO_NAME.into(),
    };

    name_str
}

