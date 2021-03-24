use crate::parse::AST;
use crate::prophet::*;

mod body;
pub use body::*;

mod expr;
pub use expr::*;

mod stmt;
pub use stmt::*;

pub fn merge_modules(modules: Vec<ModuleComponent>) -> Vec<ModuleComponent> {
    // for mut module in modules {
    //     println!("\n\nNEXT MODULE{:?}", module)
    // }

    modules
}

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
    let mut components: Vec<ComponentType> = vec![];

    // for node in &ast.children {
    //     println!("{}",node.r#type)
    // }

    let package_name = match ast.find_child_by_type(&["package_clause"]) {
        None => std::process::exit(102),
        Some(package_child) => match package_child.find_child_by_type(&["package_identifier"]) {
            None => std::process::exit(103),
            Some(package_val) => &*(package_val.value),
        },
    };

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
            // "package_clause" => println!("{:?}", components.push(parse_package(node))),
            // "import_declaration" => println!("{:?}", components.push(parse_import(node))),
            // "function_declaration" => println!("{:?}", components.push(parse_function(node, package_name, path))),
            // "type_declaration" => println!("{:?}", components.push(parse_type(node))),
            "method_declaration" | "function_declaration" => components.push(
                ComponentType::MethodComponent(parse_method(node, package_name, path)),
            ),
            tag => println!("Cannot identify provided tag {:#?}", tag),
        };
    }

    // println!("Done\n{:?}", components);

    return components;
}

fn parse_package(ast: &AST) -> String {
    // ignore this for now, I dont see a way to implement it.
    "Parse Package Not Implemeted: no obvious way...".to_string()
}

fn parse_import(ast: &AST) -> String {
    "Parse Import Not Implemented: no obvious way...".to_string()
}

fn parse_function(ast: &AST, package_name: &str, path: &str) -> String {
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

    // Some(method)
    "3".to_string()
}

fn parse_type(ast: &AST) -> String {
    "4".to_string()
}

fn parse_method(ast: &AST, package_name: &str, path: &str) -> MethodComponent {
    // get return type
    let return_type = match ast.find_child_by_type(&["type_identifier"]) {
        Some(typ) => parse_method_return_type(typ),
        None => "void",
    };

    // get function name
    let function_name = match ast.find_child_by_type(&["field_identifier"]) {
        Some(field) => &*field.value,
        None => "ERROR NO METHOD NAME",
    };

    // get parameter list data
    let parameter_list_data = match ast.find_child_by_type(&["parameter_list"]) {
        Some(param_list) => parse_method_parameter_list(param_list),
        None => std::process::exit(100),
    };

    //body
    let body = ast.find_child_by_type(&["block"]);

    // get line count = line_end  - line_begin + 1
    // get line_begin
    //get line_end
    let (line_begin, line_end) = match body {
        None => (0, 0),
        Some(body) => match body.span {
            Some((line_start, _col_start, line_end, _col_end)) => {
                (line_start as i32, line_end as i32)
            }
            None => (0, 0),
        },
    };

    // let body = body.map_or_else(|| None, |body| Some(body::func_body(body)));
    let body = None;

    let method = MethodComponent {
        component: ComponentInfo {
            path: path.to_string(),
            package_name: package_name.to_string(),
            instance_name: function_name.to_string(),
            instance_type: InstanceType::MethodComponent,
        },
        accessor: AccessorType::Default,
        method_name: function_name.to_string(),
        return_type: return_type.to_string(),
        parameters: parameter_list_data,
        is_static: false,
        is_abstract: false,
        is_final: false,
        sub_methods: vec![],
        annotations: vec![],
        line_count: line_end - line_begin + 1,
        line_begin,
        line_end,
        body,
    };

    // println!("{:?}", method);

    method
}

fn parse_method_return_type(ast: &AST) -> &str {
    // println!("{}", ast.value);
    &*ast.value
}

fn parse_method_parameter_list(parameter_list: &AST) -> Vec<MethodParamComponent> {
    let params: Vec<MethodParamComponent> = parameter_list
        .children
        .iter()
        .filter(|child| child.r#type == "parameter_declaration")
        .map(|param| parse_method_parameter_list_parameter_parse(param))
        .filter_map(|param| param)
        .collect();

    // let params: Vec<MethodParamComponent> = parameter_list
    // .children
    // .iter()
    // .filter(|child| child.r#type == "parameter_declaration")
    // .map(|param_decl| parse_method_parameter_list_parameter_parse(param_decl))
    // .filter_map(|param_decl| param_decl)
    // .collect();

    params
}

fn parse_method_parameter_list_parameter_parse(param_decl: &AST) -> Option<MethodParamComponent> {
    let mut param_type = variable_type(param_decl)?;
    let ident = variable_ident(param_decl, &mut param_type).unwrap_or(variable_ident_inner(
        param_decl.children.iter().last()?,
        &mut param_type,
    )?);

    let param = MethodParamComponent {
        component: ComponentInfo {
            path: "".to_string(),
            package_name: "".to_string(),
            instance_name: ident.clone(),
            instance_type: InstanceType::FieldComponent,
        },
        annotation: None,
        parameter_name: ident,
        r#type: param_type,
    };

    Some(param)
}

/// Get the value for a type identifier
fn type_ident(ast: &AST) -> String {
    match &*ast.r#type {
        "primitive_type" | "type_identifier" | "auto" => ast.value.clone(),
        "scoped_type_identifier" | "scoped_namespace_identifier" | "type_descriptor" => {
            let ret: String = ast
                .children
                .iter()
                .map(|child| match &*child.r#type {
                    "scoped_identifier"
                    | "scoped_namespace_identifier"
                    | "scoped_type_identifier"
                    | "template_type" => type_ident(child),
                    _ => child.value.clone(),
                })
                .collect();
            ret
        }
        "scoped_identifier" => ast.children.iter().map(|child| type_ident(child)).collect(),
        "template_type" | "template_function" => {
            let outer_type: String = ast
                .children
                .iter()
                .filter(|child| child.r#type != "template_argument_list")
                .map(|child| type_ident(&child))
                .collect();

            let type_args = ast
                .find_child_by_type(&["template_argument_list"])
                .expect("No argument list for template");

            let inner_types = type_args
                .children
                .iter()
                .filter(|child| child.r#type == "type_descriptor")
                .map(|child| type_ident(child))
                .fold(String::new(), |t1, t2| match &*t1 {
                    "" => t2,
                    _ => t1 + ", " + &t2,
                });

            format!("{}<{}>", outer_type, inner_types)
        }
        "destructor_name" | "constructor_name" => func_ident(ast),
        "struct_specifier" => {
            format!(
                "struct {}",
                type_ident(
                    ast.children
                        .iter()
                        .last()
                        .expect("Malformed struct specifier")
                )
            )
        }
        _ => ast.r#type.clone(),
    }
}

fn variable_ident(ast: &AST, variable_type: &mut String) -> Option<String> {
    let ident = ast.find_child_by_type(&[
        "pointer_declarator",
        "pointer_expression",
        "reference_declarator",
        "reference_expression",
        "identifier",
        "field_identifier",
        "type_identifier",
        "array_declarator",
    ])?;

    variable_ident_inner(ident, variable_type)
}

fn variable_ident_inner(ident: &AST, variable_type: &mut String) -> Option<String> {
    Some(match &*ident.r#type {
        "pointer_declarator"
        | "reference_declarator"
        | "pointer_expression"
        | "reference_expression"
        | "array_declarator" => {
            ident
                .children
                .iter()
                .filter(|child| match &*child.r#type {
                    "identifier" | "field_identifier" | "type_identifier" | "this" | "auto" => {
                        false
                    }
                    _ => true,
                }) // get either & or * type
                .for_each(|star| {
                    if !variable_type.contains(&star.value) {
                        // trying to prevent duplicate * and & when they shouldnt be there..
                        variable_type.push_str(&star.value);
                    }
                });
            ident
                .find_child_by_type(&[
                    "identifier",
                    "field_identifier",
                    "type_identifier",
                    "this",
                    "auto",
                ])
                .map_or_else(|| "".to_string(), |identifier| identifier.value.clone())
        }
        "identifier" | "field_identifier" | "type_identifier" | "this" | "auto" => {
            ident.value.clone()
        }
        "abstract_pointer_declarator" | "abstract_reference_declarator" => {
            variable_type.push_str(&ident.children.get(0)?.value);
            "".to_string()
        }
        _ => "".to_string(),
    })
}

fn variable_type(ast: &AST) -> Option<String> {
    let scoped_type_ident = ast.find_child_by_type(&["identifier", "pointer_type"]);

    return match scoped_type_ident {
        None => std::process::exit(101),
        Some(thing) => Some(type_ident(thing)),
    };
}

/// Get the value for a function identifier
fn func_ident(ast: &AST) -> String {
    match &*ast.r#type {
        "function_declarator" => {
            let ident = ast.find_child_by_type(&[
                "scoped_identifier",
                "identifier",
                "field_identifier",
                "destructor_name",
                "constructor_name",
                "operator_name",
                "template_type",
            ]);
            match ident {
                Some(ident) => func_ident(ident),
                None => "".to_string(),
            }
        }
        "scoped_identifier" | "template_type" => type_ident(ast),
        "destructor_name" | "constructor_name" => {
            let ident: String = ast
                .children
                .iter()
                .map(|child| child.value.clone())
                .collect();
            ident
        }
        "identifier" | "field_identifier" | "operator_name" => ast.value.clone(),
        _ => "".to_string(),
    }
}
