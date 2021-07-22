use std::collections::HashMap;

use crate::parse::AST;
use crate::prophet::*;

mod class_def;
use class_def::*;

mod method_body;
mod method_def;
mod modifier;
mod util;

/// Topmost level of the Java parser, provides public API

pub fn merge_modules(modules: Vec<ModuleComponent>) -> Vec<ModuleComponent> {
    let mut packages: HashMap<String, ModuleComponent> = HashMap::new();

    // Merge same-name modules
    for module in modules.into_iter() {
        let name = module.module_name.clone();
        if packages.contains_key(&name) {
            let orig_module = packages.get_mut(&name).expect("Contains key lied to me!");
            orig_module.merge_into(module);
        } else {
            packages.insert(name, module);
        }
    }
    packages.into_iter().map(|kv| kv.1).collect()
}

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];

    for node in ast
        .find_all_children_by_type(&[
            "import_declaration",
            "package_declaration",
            "class_declaration",
            "interface_declaration",
            "enum_declaration",
            "annotation_declaration",
        ])
        .get_or_insert(vec![])
        .iter()
    {
        match &*node.r#type {
            "import_declaration" => {parse_import(&node);},
            "package_declaration" => {
                package = parse_package(&node)
                    .expect(&*format!("Malformed package declaration {:#?}!", node));
            }
            "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "annotation_declaration" => match parse_class(node, &*package, path) {
                Some(class) => {
                    // Save the methods
                    for method in class.component.methods.iter() {
                        components.push(ComponentType::MethodComponent(method.clone()));
                    }
                    // class.component.methods = vec![];

                    // Save the class itself
                    components.push(ComponentType::ClassOrInterfaceComponent(class));
                }
                None => {}
            },
            tag => todo!("Cannot identify provided tag {:#?}", tag),
        };
    }

    // Create module this falls in. If default package, use "default" (no name collisions possible, it's a reserved word)
    if package == "" {
        package = "default".into();
    }
    // let module = create_module(package.as_str(), path, &components);
    // vec![module]
    components
}

fn create_module(package: &str, path: &str, components: &Vec<ComponentType>) -> ComponentType {
    let mut module = ModuleComponent::new(package.to_string(), path.to_string());
    let classes = components.iter().filter_map(|comp| match comp {
        ComponentType::ClassOrInterfaceComponent(class_ix) => Some(class_ix),
        _ => None,
    });

    // Get classes
    for comp in classes {
        if comp.declaration_type == ContainerType::Class {
            module.classes.push(comp.clone());
        } else if comp.declaration_type == ContainerType::Interface {
            module.interfaces.push(comp.clone());
        }
    }

    return ComponentType::ModuleComponent(module);
}

/// Take in the AST node containing the package declaration, and--if it is not malformed--return a string representing the package
fn parse_package(ast: &AST) -> Option<String> {
    let result = ast.find_child_by_type(&["identifier", "scoped_identifier"])?;
    let mut buffer = String::new();
    for member in result.children.iter() {
        if member.r#type == "scoped_identifier" {
            buffer = format!("{}{}", parse_package(result)?, buffer);
        } else {
            buffer.push_str(&*member.value);
        }
    }
    Some(buffer)
}

/// Take the AST node containing an import statement, and return back the String describing the package imported
fn parse_import(ast: &AST) -> String {
    let mut buffer = String::new();
    for child in ast.children.iter() {
        match &*child.r#type {
            "identifier" | "." | "*" => buffer.push_str(&*child.value),
            _ => buffer.push_str(&*parse_import(child)),
        };
    }
    buffer
}
