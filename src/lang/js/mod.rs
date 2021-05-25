use crate::parse::AST;
use crate::prophet::*;

mod class_def;
use class_def::*;

mod modifier;
use modifier::*;

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![]; // list of components

    // finding top-level items of interest
    // TODO: handle things besides classes lol
    for node in ast
        .find_all_children_by_type(&[
            "class_declaration"
        ])
        .get_or_insert(vec![])
        .iter()
    {
        match &*node.r#type {
            "class_declaration" => match parse_class(node, &*package, path) {
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
        }
    }
}

