use crate::parse::AST;
use crate::prophet::*;

mod class_def;
use class_def::*;

mod body;
mod method_def;
mod modifier;
mod util;
mod vartype;

/// Topmost level of the Java parser, provides public API

pub fn merge_modules(modules: Vec<ModuleComponent>) -> Vec<ModuleComponent> {
    // TODO implement correctly
    modules
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
        .expect("Provided an invalid Java file, no class, interface, annotation, enum, packages, or imports found!")
         .iter()
    {
        match &*node.r#type {
            "import_declaration" => println!("{}", parse_import(&node)),
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

                    // Save the class itself
                    components.push(ComponentType::ClassOrInterfaceComponent(class));
                }
                None => {}
            },
            tag => todo!("Cannot identify provided tag {:#?}", tag),
        };
    }

    components
}

/// Take in the AST node containing the package declaration, and--if it is not malformed--return a string representing the package
fn parse_package(ast: &AST) -> Option<String> {
    let result = ast.find_child_by_type(&["scoped_identifier"])?;
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
