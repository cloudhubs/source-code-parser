use crate::go::util::stringify_tree_children;
use crate::AST;

/// Indicates when no type is found
pub const NO_TYPE: &str = "N/A";


pub(crate) fn find_type(ast: &AST) -> String {
    let id_node = match ast.find_child_by_type(&["type_identifier"]) {
        Some(node) => node,
        None => ast,
    };
    let r#type = parse_type(id_node);
    r#type
}

fn parse_type(ast: &AST) -> String {
    match &*ast.r#type {
        "type_identifier" => ast.value.clone(),
        _ => NO_TYPE.into()
    }
}