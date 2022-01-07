use std::collections::{hash_map::Entry, HashMap};

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
        tracing::info!("Mod. Name: {}", name);

        match packages.entry(name) {
            Entry::Occupied(mut packages) => {
                // tracing::info!("Merging...");
                let orig_module = packages.get_mut();
                orig_module.merge_into(module);
            }
            Entry::Vacant(packages) => {
                // tracing::info!("New! {}", module.module_name);
                packages.insert(module);
            }
        }
    }
    packages.into_iter().map(|kv| kv.1).collect()
}

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];

    // Find package declaration
    if let Some(node) = ast.find_child_by_type(&["package_declaration"]) {
        package = parse_package(node);
    }

    // Parse rest of object
    for node in ast
        .find_all_children_by_type(&[
            // "import_declaration",
            "class_declaration",
            "interface_declaration",
            "enum_declaration",
            "annotation_declaration",
        ])
        .get_or_insert(vec![])
        .iter()
    {
        if let Some(class) = parse_class(node, &*package, path) {
            // Save the methods
            for method in class.component.methods.iter() {
                components.push(ComponentType::MethodComponent(method.clone()));
            }

            // Save the class itself
            components.push(ComponentType::ClassOrInterfaceComponent(class));
        }
    }

    components
}

/// Take in the AST node containing the package declaration, and--if it is not malformed--return a string representing the package
fn parse_package(ast: &AST) -> String {
    ast.children.iter().map(do_parse_package_node).collect()
}

/// Parse an individual subnode of a package declaration
fn do_parse_package_node(ast: &AST) -> String {
    match &*ast.r#type {
        "identifier" | "." => ast.value.clone(),
        "scoped_identifier" => parse_package(ast),
        _ => String::new(),
    }
}

// /// Take the AST node containing an import statement, and return back the String describing the package imported
// fn parse_import(ast: &AST) -> String {
//     let mut buffer = String::new();
//     for child in ast.children.iter() {
//         match &*child.r#type {
//             "identifier" | "." | "*" => buffer.push_str(&*child.value),
//             _ => buffer.push_str(&*parse_import(child)),
//         };
//     }
//     buffer
// }
