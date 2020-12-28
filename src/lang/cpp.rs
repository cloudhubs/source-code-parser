use crate::parse::AST;
use crate::prophet::*;

pub fn find_components(ast: AST) -> Vec<ComponentType> {
    match &*ast.r#type {
        "function_definition" => {
            match transform_into_method(ast) {
                Some(method) => vec![ComponentType::MethodComponent(method)],
                None => vec![],
            }
        }
        _ => {
            let components: Vec<ComponentType> = ast
                .children
                .into_iter()
                .flat_map(|child| find_components(child))
                .collect();
            components
        }
    }
}

/// Transforms an AST with type label "function_definition" to a `MethodComponent`
fn transform_into_method(ast: AST) -> Option<MethodComponent> {
    // TODO: child type "compound_statement" for function block
    let ret = ast.children.iter().find(|child| match &*child.r#type {
        "primitive_type" | "scoped_type_identifier" | "type_identifier" => true,
        _ => false,
    });
    let decl = ast.children.iter().find(|child| child.r#type == "function_declarator")?;

    let scoped_identifier = decl.children.iter().find(|child| child.r#type == "scoped_identifier")?;
    let parameter_list = decl.children.iter().find(|child| child.r#type == "parameter_list")?;


    None
}

/// Get the value for a type identifier
fn type_ident(ast: &AST) -> String {
    match &*ast.r#type {
        "primitive_type" | "type_identifier" => ast.value.clone(),
        "scoped_type_identifier" | "scoped_namespace_identifier" => {
            let ret: String = ast.children.iter().map(|child| match &*child.r#type {
                "scoped_namespace_identifier" => type_ident(ast),
                _ => child.value.clone(),
            }).collect();
            ret
        }
        _ => "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_ident_primitive() {
        let prim = AST {
            children: vec![],
            span: None,
            r#type: "primitive_type".to_string(),
            value: "uint32_t".to_string(),
        };
        assert_eq!("uint32_t".to_string(), type_ident(&prim));
    }
}