use std::collections::HashMap;

use crate::parse::AST;
use crate::prophet::*;

mod class_def;
mod function_def;
mod util;

use class_def::*;
use function_def::*;
use crate::go::util::identifier::parse_identifier;

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];

    for node in ast
        .find_all_children_by_type(&[
            "type_declaration",
            "function_declaration",
            "package_clause",
            "import_declaration",
        ])
        .get_or_insert(vec![])
        .iter()
    {
        match &*node.r#type {
            "function_declaration" => match parse_function(node, &*package, path) {
                Some(function) => components.push(ComponentType::MethodComponent(function.clone())),
                None => {},
            },
            "package_clause" => {
                package = parse_package(node);
            },
            "type_declaration" => {
                let types = parse_struct(node, &*package, path);

                for component in types {
                    components.push(component);
                }
            },
            "import_declaration" => println!("{}", parse_import(node)),
            tag => todo!("Cannot identify provided tag {:#?}", tag),
        };
    }

    components
}

fn parse_package(ast: &AST) -> String {
   parse_identifier(ast)
}

fn parse_import(ast: &AST) -> String {
    let mut buffer = String::new();
    for node in ast.children.iter() {
        match &*node.r#type {
            "import_spec" => {
                match node.find_child_by_type(&["interpreted_string_literal"]) {
                    Some(import) => buffer.push_str(&*import.value),
                    None => {}
                }
            },
            _ => buffer.push_str(&*parse_import(node))
        }
    }

    buffer
}

