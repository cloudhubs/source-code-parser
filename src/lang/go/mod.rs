
use crate::parse::AST;
use crate::prophet::*;


pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    return find_components_internal(ast, String::new(), path);
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];

    return components;
}

