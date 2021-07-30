use std::collections::HashMap;

use crate::parse::AST;
use crate::prophet::*;

mod class_def;
mod function_def;
mod util;

use class_def::*;
use function_def::*;
use crate::go::util::identifier::parse_identifier;
use std::borrow::Borrow;

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];
    let mut types = HashMap::new();

    //first parse for all nodes EXCEPT for "method_declaration"
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
                match parse_type(node, &*package, path) {
                    Some(type_decl) => {
                        let type_name = type_decl.component.container_name.clone();
                        types.insert(type_name, type_decl);
                    }
                    None => {}
                }
            },
            "import_declaration" => println!("{}", parse_import(node)),
            tag => todo!("Cannot identify provided tag {:#?}", tag),
        };
    }

    //now parse "method_declaration" nodes
    for node in ast
        .find_all_children_by_type(&["method_declaration"])
        .get_or_insert(vec![])
        .iter()
    {
        let tuple = parse_method(node, &*package, path);

        match types.get(&tuple.0) {
            Some(parent_struct) => {
                //create a copy of the instance of the original struct and add the method to it
                let mut new_methods = parent_struct.component.methods.clone();
                new_methods.push(tuple.1.clone());

                let parent_component = parent_struct.component.clone();
                let new_parent_struct = ClassOrInterfaceComponent {
                        component: ContainerComponent {
                            component: parent_component.component.clone(),
                            accessor: parent_component.accessor.clone(),
                            stereotype: parent_component.stereotype.clone(),
                            methods: new_methods,
                            container_name: parent_component.container_name.clone(),
                            line_count: parent_component.line_count.clone(),
                        },
                        declaration_type: parent_struct.declaration_type.clone(),
                        annotations: parent_struct.annotations.clone(),
                        constructors: parent_struct.constructors.clone(),
                        field_components: parent_struct.field_components.clone(),
                };

                types.insert(tuple.0, new_parent_struct);
            },
            None => {},
        }

    }

    //push the now updated types with their member methods onto the components vector
    for (k, v) in types {
        components.push(ComponentType::ClassOrInterfaceComponent(v));
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
                    Some(import) => buffer.push_str(&*trim_import((&*import.value).to_string())),
                    None => {}
                }
            },
            "import_spec_list" => {
                for import_node in node.children.iter() {
                    match &*import_node.r#type {
                        "import_spec" => {
                            match import_node.find_child_by_type(&["interpreted_string_literal"]) {
                                Some(import) => {
                                    buffer.push_str(&*trim_import((&*import.value).to_string()));
                                    buffer.push_str("\n");
                                },
                                None => {}
                            }
                        },
                        _ => {}
                    }
                }
            }
            _ => buffer.push_str(&*parse_import(node))
        }
    }

    //to remove the last newline for multiple imports
    if buffer.ends_with('\n') {
        buffer.pop();
    }

    buffer
}

/// removes the quotations surrounding the values of the "interpreted_string_literal" nodes
fn trim_import(import_str: String) -> String {
    let mut str = import_str.clone();

    /// additional checks to see if the string actually begins and ends with quotation marks
    if str.starts_with('\"') {
        str.remove(0);
    }
    if str.ends_with('\"') {
        str.pop();
    }

    str
}