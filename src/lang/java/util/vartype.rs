/// Handles variable type parsing, needed in several modules
use crate::java::util::stringify_tree_children;
use crate::AST;

/// Type indicating no type found
pub const NO_TYPE: &str = "N/A";

/// Provided an AST representing the type of a parameter/field, interprets it into a valid String representation
fn parse_type(ast: &AST) -> String {
    match &*ast.r#type {
        "type_identifier" | "boolean_type" | "void_type" => ast.value.clone(),
        "array_type" => make_array_type(ast, ""),
        "integral_type" | "floating_point_type" => ast
            .children
            .get(0)
            .expect("Cannot detect the type of a numeric primitive! The AST appears malformed!")
            .r#type
            .clone(),
        "dimensions" | "wildcard" => stringify_tree_children(ast),
        "generic_type" => parse_type_args(ast),
        _ => NO_TYPE.into(),
    }
}

/// Search a provided AST for a type name. Return N/A if none found
pub(crate) fn find_type(ast: &AST) -> String {
    // Look for an "easy case" of handed the raw type
    let r#type = parse_type(ast);
    if r#type != NO_TYPE {
        return r#type;
    }

    // Find express type
    let r#type = ast.find_child_by_type(&[
        "type_identifier",
        "array_type",
        "integral_type",
        "floating_point_type",
        "boolean_type",
        "void_type",
        "dimensions",
        "generic_type",
    ]);

    // If type found, record it
    let mut result: String;
    if let Some(r#type) = r#type {
        result = parse_type(r#type);
    } else {
        result = NO_TYPE.into();
    }

    // Check for C-style array declaration
    if let Some(ident) = ast.find_child_by_type(&["variable_declarator"]) {
        if let Some(dimensions) = ident.find_child_by_type(&["dimensions", "dimensions_expr"]) {
            result = make_array_type(dimensions, &*result);
        }
    }

    // Check for spread operator
    if ast.find_child_by_type(&["..."]).is_some() {
        result.push_str("...");
    }

    // Return resulting type definition
    result
}

/// Parse the a `type_arguments` node
pub(crate) fn parse_type_args(ast: &AST) -> String {
    // Vincent's changes here
    let mut generic = String::new();
    if let Some(basetype) = ast.find_all_children_by_type(&["type_identifier"]) {
        generic.push_str(&*basetype[0].r#value);
        if let Some(args) = ast.find_all_children_by_type(&["type_arguments"]) {
            for param in args[0].children.iter() {
                match &*param.r#value {
                    "," | "<" | ">" => generic.push_str(&*param.r#value),
                    _ => generic.push_str(&*parse_type(param)),
                }
            }
        }
    }
    generic
}

fn make_array_type(ast: &AST, base: &str) -> String {
    let mut result_type: String = base.to_string();
    for child in ast.children.iter() {
        let tmp_result = parse_type(child);
        if tmp_result != NO_TYPE {
            result_type.push_str(&*tmp_result);
        } else {
            result_type.push_str(&*child.value);
        }
    }
    result_type
}
