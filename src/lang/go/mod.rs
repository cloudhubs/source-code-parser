use crate::parse::AST;
use crate::prophet::*;

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    return find_components_internal(ast, String::new(), path);
}


// #[derive(Debug, Eq, PartialEq, Serialize, Clone)]
// #[serde(untagged)]
// pub enum ComponentType {
//     ClassOrInterfaceComponent(ClassOrInterfaceComponent),
//     AnnotationComponent(AnnotationComponent),
//     MethodComponent(MethodComponent),
//     ModuleComponent(ModuleComponent),
//     FieldComponent(FieldComponent),
//     MethodParamComponent(MethodParamComponent),
// }
fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];

    // for node in &ast.children {
    //     println!("{}",node.r#type)
    // }

    for node in ast
        .find_all_children_by_type(&[
            "package_clause",
            "import_declaration",
            "function_declaration",
            "type_declaration",
            "method_declaration",
        ])
        .expect("Provided an invalid Go file, no package, imports or functions!")
        .iter()
    {
        match &*node.r#type {
            "package_clause" => println!("{}", parse_package(&node)),
            "import_declaration" => println!("{}", parse_import(&node)),
            "function_declaration" => println!("{}", parse_function(&node)),
            "type_declaration" => println!("{}", parse_type(&node)),
            "method_declaration" => println!("{}", parse_method(&node)),
            tag => todo!("Cannot identify provided tag {:#?}", tag),
        };
    }

    return components;
}

fn parse_package(ast: &AST) -> String {
    "1".to_string()
}

fn parse_import(ast: &AST) -> String {
    "2".to_string()
}

fn parse_function(ast: &AST) -> String {
    // get return value

    // get function name

    // get parameter list data

    // get line count = line_end  - line_begin + 1
    // get line_begin
    //get line_end
    //body
    
    // let method = MethodComponent {
    //     component: ComponentInfo {
    //         path: path.to_string(),
    //         package_name: module_name.to_string(),
    //         instance_name: fn_ident.clone(),
    //         instance_type: InstanceType::MethodComponent,
    //     },
    //     accessor: AccessorType::Default,
    //     method_name: fn_ident,
    //     return_type: ret_type,
    //     parameters: params,
    //     is_static: false,
    //     is_abstract: false,
    //     is_final: false,
    //     sub_methods: vec![],
    //     annotations: vec![],
    //     line_count: line_end - line_begin + 1,
    //     line_begin,
    //     line_end,
    //     body,
    // };

    Some(method)
    "3".to_string()
}

fn parse_type(ast: &AST) -> String {
    "4".to_string()
}

fn parse_method(ast: &AST) -> String {
    "5".to_string()
}
