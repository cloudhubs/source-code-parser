use crate::java::util::stringify_tree_children;
use crate::AST;

/// Handles variable type parsing, needed in several modules

/// Provided an AST representing the type of a parameter/field, interprets it into a valid String representation
pub(crate) fn parse_type(ast: &AST) -> String {
    match &*ast.r#type {
        "type_identifier" => ast.value.clone(),
        "array_type" => {
            let mut result_type = String::new();
            for child in ast.children.iter() {
                result_type.push_str(&*parse_type(child));
            }
            result_type
        }
        "integral_type" | "floating_point_type" => ast
            .children
            .get(0)
            .expect("Cannot detect the type of a numeric primitive! The AST appears malformed!")
            .r#type
            .clone(),
        "boolean_type" | "void_type" => ast.value.clone(),
        "dimensions" => stringify_tree_children(ast),
        _ => String::from("N/A"),
    }
}

/// Search a provided AST for a type name. Return N/A if none found
pub(crate) fn find_type(ast: &AST) -> String {
    let r#type = ast.find_child_by_type(&[
        "type_identifier",
        "array_type",
        "integral_type",
        "floating_point_type",
        "boolean_type",
        "void_type",
        "dimensions",
    ]);
    if let Some(r#type) = r#type {
        parse_type(r#type)
    } else {
        String::from("N/A")
    }
}
