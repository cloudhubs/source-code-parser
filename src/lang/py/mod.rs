use std::collections::HashMap;

use crate::parse::AST;
use crate::prophet::*;

mod class_def;
use class_def::*;

mod method_body;
mod method_def;
mod modifier;
mod util;
//mod function_def;

/// Topmost level of the Python parser, provides public API

pub fn merge_modules(modules: Vec<ModuleComponent>) -> Vec<ModuleComponent> {
    let mut packages: HashMap<String, ModuleComponent> = HashMap::new();

    // Merge same-name modules
    for module in modules.into_iter() {
        let name = module.module_name.clone();
        println!("Mod. Name: {}", name);
        if packages.contains_key(&name) {
            println!("Merging...");
            let orig_module = packages.get_mut(&name).expect("Contains key lied to me!");
            orig_module.merge_into(module);
        } else {
            println!("New! {}", module.module_name);
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
            "import_from_statement",
            "import_statement",
            "class_definition",
            "function_definition",
        ])
        .get_or_insert(vec![])
        .iter()
    {
        match &*node.r#type {
            "import_statement" => println!("{}", parse_import(&node)),
            "import_from_statement" => println!("{}", parse_import_from(&node)),
            | "class_definition" => match parse_class(node, &*package, path) {
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


/// Take the AST node containing an import statement, and return back the String describing the package imported
fn parse_import(ast: &AST) -> String {
    let mut buffer = String::new();
    for child in ast.children.iter() {
        match &*child.r#type {
            "identifier" | "dotted_name" | "as" | "aliased_import" | "." => buffer.push_str(&*child.value),
            _ => buffer.push_str(&*parse_import(child)),
        };
    }
    buffer
}

fn parse_import_from(ast: &AST) -> String {
    let mut buffer = String::new();
    for child in ast.children.iter() {
        match &*child.r#type {
            "from" | "identifier" | "dotted_name" | "as" | "aliased_import" | "." => buffer.push_str(&*child.value),
            _ => buffer.push_str(&*parse_import_from(child)),
        };
    }
    buffer
}