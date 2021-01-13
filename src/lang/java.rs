use crate::parse::AST;
use crate::prophet::*;

pub fn find_components(ast: AST, package: &str, path: &str) -> Vec<ComponentType> {
    match &*ast.r#type {
        "class_declaration" | "interface_declaration" | "annotation_declaration" => {
            parse_class(ast, package, path)
        }
        "enum_declaration" => {
            todo!("Enum");
        }
        _ => {
            let components: Vec<ComponentType> = ast
                .children
                .into_iter()
                .flat_map(|child| find_components(child, package, path))
                .collect();
            components
        }
    }
}

fn parse_class(ast: &AST, package: &str, path: &str) -> Option<ComponentType> {
    // Get the body of the class/interface/annotation
    let body: &AST;
    match ast.find_child_by_type(&[
        "class_declaration",
        "interface_declaration",
        "annotation_declaration",
    ]) {
        Some(body_ast) => {
            body = body_ast;
        }
        None => {
            return None;
        }
    };

    // Parse the body
    let members = parse_declarations(&body.children);
    let mut fields = vec![];
    let mut methods = vec![];
    for field in field_components {
        match field {
            ComponentType::MethodComponent(method) => methods.push(method),
            ComponentType::FieldComponent(field) => fields.push(field),
            _ => {}
        }
    }
}

fn parse_declarations(ast: &[AST]) -> Vec<ComponentType> {
    for member in ast.iter() {
        match ast.r#type {
            "constructor_declaration" => {}
        }
    }
}
