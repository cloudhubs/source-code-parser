use crate::parse::AST;
use crate::prophet::*;
use crate::ast::*;

pub fn merge_modules(modules: Vec<ModuleComponent>) -> Vec<ModuleComponent> {
    let mut merged = vec![];
    for mut module in modules {
        // First module is not a duplicate
        if merged.len() == 0 {
            merged.push(module);
            continue;
        }

        let mergeable = merged
            .iter_mut()
            .find(|m| m.module_name == module.module_name);
        // Check if there is a module with the same name already merged
        if let Some(mergeable) = mergeable {
            // Move the classes, ifcs, functions from the module
            mergeable.classes.append(&mut module.classes);
            mergeable.interfaces.append(&mut module.interfaces);
            mergeable
                .component
                .methods
                .append(&mut module.component.methods);

            // Check if there are class methods declared in the functions
            for class in mergeable.classes.iter_mut() {
                let functions: Vec<&mut MethodComponent> = mergeable
                    .component
                    .methods
                    .iter_mut()
                    .filter(|m| m.method_name.starts_with(&class.component.container_name))
                    .collect();
                for function in functions {
                    let class_method = class.component.methods.iter_mut().find(|m| {
                        function.method_name.ends_with(&m.method_name)
                            && m.parameters == function.parameters
                    });

                    if let Some(class_method) = class_method {
                        // TODO: add the body from method into class_method
                        class_method.line_begin = function.line_begin;
                        class_method.line_end = function.line_end;
                        class_method.line_count = function.line_count;
                    }
                }
            }

            // Filter out the class methods since the merge already occurred in the previous loop
            mergeable.component.methods =
                mergeable
                    .component
                    .methods
                    .clone()
                    .into_iter()
                    .filter(|m| {
                        match mergeable.classes.iter_mut().find(|class| {
                            m.method_name.starts_with(&class.component.container_name)
                        }) {
                            Some(_) => false,
                            None => true,
                        }
                    })
                    .collect();
        } else {
            merged.push(module);
        }
    }
    merged
}

pub fn find_components(ast: AST, module_name: &str, path: &str) -> Vec<ComponentType> {
    match &*ast.r#type {
        "namespace_definition" => match transform_namespace_to_module(ast, path) {
            Some(module) => vec![ComponentType::ModuleComponent(module)],
            None => vec![],
        },
        "function_definition" => match transform_into_method(&ast, module_name, path) {
            Some(method) => vec![ComponentType::MethodComponent(method)],
            None => vec![],
        },
        "class_specifier" | "struct_specifier" | "type_definition" => {
            match transform_into_class(&ast, module_name, path) {
                Some(class) => vec![ComponentType::ClassOrInterfaceComponent(class)],
                None => vec![],
            }
        }
        _ => {
            let components: Vec<ComponentType> = ast
                .children
                .into_iter()
                .flat_map(|child| find_components(child, module_name, path))
                .collect();
            components
        }
    }
}

fn transform_namespace_to_module(ast: AST, path: &str) -> Option<ModuleComponent> {
    let name = ast
        .children
        .iter()
        .find(|child| child.r#type == "identifier")?
        .value
        .clone();

    let mut module = ModuleComponent::new(name.clone(), path.to_string());
    ast.children
        .into_iter()
        .flat_map(|child| find_components(child, &name, path))
        .for_each(|component| match component {
            ComponentType::ClassOrInterfaceComponent(component) => {
                match component.declaration_type {
                    ContainerType::Class => {
                        module.classes.push(component);
                    }
                    ContainerType::Interface => {
                        module.interfaces.push(component);
                    }
                    r#type => {
                        println!(
                            "got other label when it should have been class/ifc: {:#?}",
                            r#type
                        );
                    }
                }
            }
            ComponentType::MethodComponent(method) => {
                module.component.methods.push(method);
            }
            ComponentType::ModuleComponent(_module) => {
                unimplemented!();
            }
            _ => {
                unimplemented!();
            }
        });

    Some(module)
}

/// Transforms an AST with type label "function_definition" or "field_declaration" or "declaration" to a `MethodComponent`
fn transform_into_method(ast: &AST, module_name: &str, path: &str) -> Option<MethodComponent> {
    // TODO: child type "compound_statement" for function block
    let ret = ast.find_child_by_type(&[
        "primitive_type",
        "scoped_identifier_type",
        "type_identifier",
    ]);
    let mut ret_type = match ret {
        Some(ret) => type_ident(ret),
        None => "".to_string(),
    };

    let decl = match ast.find_child_by_type(&["reference_declarator", "pointer_declarator"]) {
        Some(reference_decl) => {
            let reference = reference_decl.find_child_by_type(&["*", "&"])?;
            ret_type.push_str(&reference.value);
            reference_decl.find_child_by_type(&["function_declarator"])
        }
        None => ast.find_child_by_type(&["function_declarator"]),
    }?;

    // let identifier = decl.find_child_by_type(&["scoped_identifier", "identifier"])?;
    let fn_ident = func_ident(decl);

    let parameter_list = decl.find_child_by_type(&["parameter_list"])?;
    let params = func_parameters(parameter_list, module_name, path);

    let body = ast.find_child_by_type(&["compound_statement"]);
    let (line_begin, line_end) = match body {
        Some(body) => match body.span {
            Some((line_start, _col_start, line_end, _col_end)) => {
                (line_start as i32, line_end as i32)
            }
            None => (0, 0),
        },
        None => (0, 0),
    };
    let body = body.map_or_else(|| None, |body| func_body(body));

    let method = MethodComponent {
        component: ComponentInfo {
            path: path.to_string(),
            package_name: module_name.to_string(),
            instance_name: fn_ident.clone(),
            instance_type: InstanceType::MethodComponent,
        },
        accessor: AccessorType::Default,
        method_name: fn_ident,
        return_type: ret_type,
        parameters: params,
        is_static: false,
        is_abstract: false,
        sub_methods: vec![],
        annotations: vec![],
        line_count: line_end - line_begin + 1,
        line_begin,
        line_end,
        body,
    };

    Some(method)
}

/// Get the value for a type identifier
fn type_ident(ast: &AST) -> String {
    match &*ast.r#type {
        "primitive_type" | "type_identifier" => ast.value.clone(),
        "scoped_type_identifier" | "scoped_namespace_identifier" | "type_descriptor" => {
            let ret: String = ast
                .children
                .iter()
                .map(|child| match &*child.r#type {
                    "scoped_namespace_identifier" | "scoped_type_identifier" => type_ident(child),
                    _ => child.value.clone(),
                })
                .collect();
            ret
        }
        "template_type" => {
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
                .map(|child| type_ident(&child))
                .fold(String::new(), |t1, t2| match &*t1 {
                    "" => t2,
                    _ => t1 + ", " + &t2,
                });

            format!("{}<{}>", outer_type, inner_types)
        }
        _ => ast.value.clone(),
    }
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
            ]);
            match ident {
                Some(ident) => func_ident(ident),
                None => "".to_string(),
            }
        }
        "scoped_identifier" => {
            let ident: String = ast
                .children
                .iter()
                .map(|child| match &*child.r#type {
                    "destructor_name" | "constructor_name" => func_ident(child),
                    _ => child.value.clone(),
                })
                .collect();
            ident
        }
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

fn func_parameters(param_list: &AST, module_name: &str, path: &str) -> Vec<MethodParamComponent> {
    let params: Vec<MethodParamComponent> = param_list
        .children
        .iter()
        .filter(|child| child.r#type == "parameter_declaration")
        .map(|param_decl| func_parameter(param_decl, module_name, path))
        .filter_map(|param_decl| param_decl)
        .collect();

    params
}

fn variable_type(ast: &AST) -> Option<String> {
    let scoped_type_ident = ast.find_child_by_type(&[
        "scoped_type_identifier",
        "primitive_type",
        "type_identifier",
        "template_type",
    ])?;
    Some(type_ident(scoped_type_ident))
}

fn variable_ident(ast: &AST, variable_type: &mut String) -> Option<String> {
    let ident = ast.find_child_by_type(&[
        "pointer_declarator",
        "reference_declarator",
        "identifier",
        "field_identifier",
    ])?;

    Some(match &*ident.r#type {
        "pointer_declarator" | "reference_declarator" => {
            ident
                .children
                .iter()
                .filter(|child| match &*child.r#type {
                    "identifier" | "field_identifier" => false,
                    _ => true,
                }) // get either & or * type
                .for_each(|star| variable_type.push_str(&star.value));
            ident
                .find_child_by_type(&["identifier", "field_identifier"])
                .map_or_else(|| "".to_string(), |identifier| identifier.value.clone())
        }
        "identifier" | "field_identifier" => ident.value.clone(),
        _ => "".to_string(),
    })
}

fn func_parameter(param_decl: &AST, module_name: &str, path: &str) -> Option<MethodParamComponent> {
    let mut param_type = variable_type(param_decl)?;
    let ident = variable_ident(param_decl, &mut param_type)?;

    let param = MethodParamComponent {
        component: ComponentInfo {
            path: path.to_string(),
            package_name: module_name.to_string(),
            instance_name: ident.clone(),
            instance_type: InstanceType::FieldComponent,
        },
        annotation: None,
        parameter_name: ident,
        parameter_type: param_type,
    };

    Some(param)
}

/// Transforms an AST with type label "class_specifier", "struct_specifier" or "type_definition" to a `ClassOrInterfaceComponent`
fn transform_into_class(
    ast: &AST,
    module_name: &str,
    path: &str,
) -> Option<ClassOrInterfaceComponent> {
    let class_name = ast
        .find_child_by_type(&["type_identifier"])
        .map_or_else(|| "".into(), |t| t.value.clone());

    // If a "type_definition" is the given AST, it should have one of these as a child.
    let struct_specifier = ast.find_child_by_type(&["struct_specifier", "class_specifier"]);
    let field_list = match struct_specifier {
        Some(struct_specifier) => struct_specifier.find_child_by_type(&["field_declaration_list"]),
        // Both class_specifier and struct_specifier both have a field_declaration_list child
        None => ast.find_child_by_type(&["field_declaration_list"]),
    }?;

    let field_components = class_fields(&field_list.children, module_name, path);
    let mut fields = vec![];
    let mut methods = vec![];
    for field in field_components {
        match field {
            ComponentType::MethodComponent(method) => methods.push(method),
            ComponentType::FieldComponent(field) => fields.push(field),
            _ => {}
        }
    }

    Some(ClassOrInterfaceComponent {
        component: ContainerComponent {
            component: ComponentInfo {
                path: path.into(),
                package_name: module_name.into(),
                instance_name: class_name.clone(),
                instance_type: InstanceType::ClassComponent,
            },
            accessor: AccessorType::Default,
            stereotype: ContainerStereotype::Entity,
            methods,
            container_name: class_name,
            line_count: 0,
        },
        declaration_type: ContainerType::Class,
        annotations: vec![],
        stereotype: ContainerStereotype::Entity,
        constructors: None,
        field_components: Some(fields),
    })
}

// ComponentType variants will always be FieldComponent or MethodComponent
fn class_fields(field_list: &[AST], module_name: &str, path: &str) -> Vec<ComponentType> {
    let mut fields = vec![];
    let mut access_specifier = AccessorType::Default;
    for field in field_list.iter() {
        match &*field.r#type {
            "access_specifier" => {
                access_specifier = field
                    .find_child_by_type(&[":"])
                    .map(|accessor| field_accessor(accessor))
                    .unwrap_or(AccessorType::Default);
            }
            "function_definition" | "field_declaration" | "declaration" => {
                // Need to consider that functions could be declared inside of the class
                // This means I need to alter class/method merging
                if let Some(mut method) = transform_into_method(field, module_name, path) {
                    method.accessor = access_specifier.clone();
                    method.is_abstract = field_is_abstract_method(field);
                    fields.push(ComponentType::MethodComponent(method));
                    continue;
                }

                assert!(&*field.r#type == "field_declaration");
                // Not a method if this is reached
                let mut field_type = variable_type(field).expect("Field declaration had no type");
                let field_name = variable_ident(field, &mut field_type)
                    .expect("Field declaration had no identifier");
                let field = FieldComponent {
                    component: ComponentInfo {
                        path: path.to_string(),
                        package_name: module_name.to_string(),
                        instance_name: field_name.clone(),
                        instance_type: InstanceType::FieldComponent,
                    },
                    annotations: vec![],
                    variables: vec![],
                    field_name,
                    accessor: access_specifier.clone(),
                    is_static: false,
                    is_final: false,
                    default_value: "".to_string(),
                    r#type: field_type,
                };

                fields.push(ComponentType::FieldComponent(field));
            }
            _ => {}
        }
    }
    fields
}

fn field_accessor(accessor: &AST) -> AccessorType {
    match &*accessor.value {
        "public" => AccessorType::Public,
        "private" => AccessorType::Private,
        "protected" => AccessorType::Protected,
        _ => AccessorType::Default,
    }
}

fn field_is_abstract_method(field: &AST) -> bool {
    let virtual_specifier = field
        .find_child_by_type(&["virtual_function_specifier"])
        .is_some();
    let eq = field.find_child_by_type(&["="]).is_some();
    let zero = field.find_child_by_value("0").is_some();
    virtual_specifier && eq && zero
}

// Takes in an AST with type field "compound_statement"
fn func_body(body: &AST) -> Option<Block> {
    let nodes = body.children.iter().map(|child| func_body_node(child)).collect();

    Some(Block {
        nodes,
    })
}

fn func_body_node(node: &AST) -> Node {
    match &*node.r#type {
        "declaration" => {},
        "while_statement" => {},
        "expression_statement" => {},
        "using_declaration" => {},
        "return_statement" => {},
        // ...
        _ => {}
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_ident_primitive() {
        let prim = AST {
            children: vec![],
            span: None,
            r#type: "primitive_type".to_string(),
            value: "uint32_t".to_string(),
        };
        assert_eq!("uint32_t".to_string(), type_ident(&prim));
    }

    #[test]
    fn type_ident_scoped() {
        let t = AST {
            children: vec![
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "::".to_string(),
                            value: "::".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "namespace_identifier".to_string(),
                            value: "thrift".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "scoped_namespace_identifier".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: "::".to_string(),
                    value: "::".to_string(),
                },
                AST {
                    children: vec![],
                    span: None,
                    r#type: "namespace_identifier".to_string(),
                    value: "protocol".to_string(),
                },
            ],
            span: None,
            r#type: "scoped_namespace_identifier".to_string(),
            value: "".to_string(),
        };
        assert_eq!("::thrift::protocol".to_string(), type_ident(&t));
    }

    #[test]
    fn type_ident_generics() {
        let t = AST {
            children: vec![
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "namespace_identifier".to_string(),
                            value: "stdcxx".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "::".to_string(),
                            value: "::".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "type_identifier".to_string(),
                            value: "shared_ptr".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "scoped_type_identifier".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![AST {
                        children: vec![AST {
                            children: vec![
                                AST {
                                    children: vec![
                                        AST {
                                            children: vec![
                                                AST {
                                                    children: vec![],
                                                    span: None,
                                                    r#type: "::".to_string(),
                                                    value: "::".to_string(),
                                                },
                                                AST {
                                                    children: vec![],
                                                    span: None,
                                                    r#type: "namespace_identifier".to_string(),
                                                    value: "apache".to_string(),
                                                },
                                            ],
                                            span: None,
                                            r#type: "scoped_namespace_identifier".to_string(),
                                            value: "".to_string(),
                                        },
                                        AST {
                                            children: vec![],
                                            span: None,
                                            r#type: "::".to_string(),
                                            value: "::".to_string(),
                                        },
                                        AST {
                                            children: vec![],
                                            span: None,
                                            r#type: "namespace_identifier".to_string(),
                                            value: "thrift".to_string(),
                                        },
                                    ],
                                    span: None,
                                    r#type: "scoped_namespace_identifier".to_string(),
                                    value: "".to_string(),
                                },
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "::".to_string(),
                                    value: "::".to_string(),
                                },
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "type_identifier".to_string(),
                                    value: "TProcessor".to_string(),
                                },
                            ],
                            span: None,
                            r#type: "scoped_type_identifier".to_string(),
                            value: "".to_string(),
                        }],
                        span: None,
                        r#type: "type_descriptor".to_string(),
                        value: "".to_string(),
                    }],
                    span: None,
                    r#type: "template_argument_list".to_string(),
                    value: "".to_string(),
                },
            ],
            span: None,
            r#type: "template_type".to_string(),
            value: "".to_string(),
        };
        assert_eq!(
            "stdcxx::shared_ptr<::apache::thrift::TProcessor>".to_string(),
            type_ident(&t)
        );
    }

    #[test]
    fn func_ident_destructor() {
        let f = AST {
            children: vec![AST {
                children: vec![
                    AST {
                        children: vec![],
                        span: None,
                        r#type: "namespace_identifier".to_string(),
                        value: "CastInfoService_WriteCastInfo_args".to_string(),
                    },
                    AST {
                        children: vec![],
                        span: None,
                        r#type: "::".to_string(),
                        value: "::".to_string(),
                    },
                    AST {
                        children: vec![
                            AST {
                                children: vec![],
                                span: None,
                                r#type: "~".to_string(),
                                value: "~".to_string(),
                            },
                            AST {
                                children: vec![],
                                span: None,
                                r#type: "identifier".to_string(),
                                value: "CastInfoService_WriteCastInfo_args".to_string(),
                            },
                        ],
                        span: None,
                        r#type: "destructor_name".to_string(),
                        value: "".to_string(),
                    },
                ],
                span: None,
                r#type: "scoped_identifier".to_string(),
                value: "".to_string(),
            }],
            span: None,
            r#type: "function_declarator".to_string(),
            value: "".to_string(),
        };
        assert_eq!(
            "CastInfoService_WriteCastInfo_args::~CastInfoService_WriteCastInfo_args".to_string(),
            func_ident(&f)
        );
    }

    #[test]
    fn func_ident_regular() {
        let f = AST {
            children: vec![AST {
                children: vec![
                    AST {
                        children: vec![],
                        span: None,
                        r#type: "namespace_identifier".to_string(),
                        value: "CastInfoService_WriteCastInfo_args".to_string(),
                    },
                    AST {
                        children: vec![],
                        span: None,
                        r#type: "::".to_string(),
                        value: "::".to_string(),
                    },
                    AST {
                        children: vec![],
                        span: None,
                        r#type: "identifier".to_string(),
                        value: "read".to_string(),
                    },
                ],
                span: None,
                r#type: "scoped_identifier".to_string(),
                value: "".to_string(),
            }],
            span: None,
            r#type: "function_declarator".to_string(),
            value: "".to_string(),
        };
        assert_eq!(
            "CastInfoService_WriteCastInfo_args::read".to_string(),
            func_ident(&f)
        );
    }

    #[test]
    fn func_param_single() {
        let param = AST {
            children: vec![
                AST {
                    children: vec![
                        AST {
                            children: vec![
                                AST {
                                    children: vec![
                                        AST {
                                            children: vec![
                                                AST {
                                                    children: vec![],
                                                    span: None,
                                                    r#type: "::".to_string(),
                                                    value: "::".to_string(),
                                                },
                                                AST {
                                                    children: vec![],
                                                    span: None,
                                                    r#type: "namespace_identifier".to_string(),
                                                    value: "apache".to_string(),
                                                },
                                            ],
                                            span: None,
                                            r#type: "scoped_namespace_identifier".to_string(),
                                            value: "".to_string(),
                                        },
                                        AST {
                                            children: vec![],
                                            span: None,
                                            r#type: "::".to_string(),
                                            value: "::".to_string(),
                                        },
                                        AST {
                                            children: vec![],
                                            span: None,
                                            r#type: "namespace_identifier".to_string(),
                                            value: "thrift".to_string(),
                                        },
                                    ],
                                    span: None,
                                    r#type: "scoped_namespace_identifier".to_string(),
                                    value: "".to_string(),
                                },
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "::".to_string(),
                                    value: "::".to_string(),
                                },
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "namespace_identifier".to_string(),
                                    value: "protocol".to_string(),
                                },
                            ],
                            span: None,
                            r#type: "scoped_namespace_identifier".to_string(),
                            value: "".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "::".to_string(),
                            value: "::".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "type_identifier".to_string(),
                            value: "TProtocol".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "scoped_type_identifier".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "*".to_string(),
                            value: "*".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "identifier".to_string(),
                            value: "name".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "pointer_declarator".to_string(),
                    value: "".to_string(),
                },
            ],
            span: None,
            r#type: "parameter_declarator".to_string(),
            value: "".to_string(),
        };
        let actual_param = func_parameter(&param, "", "").unwrap();
        assert_eq!(
            "::apache::thrift::protocol::TProtocol*".to_string(),
            actual_param.parameter_type,
        );
        assert_eq!("name".to_string(), actual_param.parameter_name,);
    }

    #[test]
    fn destructor_method() {
        let destructor = AST {
            children: vec![
                AST {
                    children: vec![AST {
                        children: vec![],
                        span: None,
                        r#type: "virtual".to_string(),
                        value: "virtual".to_string(),
                    }],
                    span: None,
                    r#type: "virtual_function_specifier".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "~".to_string(),
                                    value: "~".to_string(),
                                },
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "identifier".to_string(),
                                    value: "CastInfoServiceIf".to_string(),
                                },
                            ],
                            span: None,
                            r#type: "destructor_name".to_string(),
                            value: "".to_string(),
                        },
                        AST {
                            children: vec![
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: "(".to_string(),
                                    value: "(".to_string(),
                                },
                                AST {
                                    children: vec![],
                                    span: None,
                                    r#type: ")".to_string(),
                                    value: ")".to_string(),
                                },
                            ],
                            span: None,
                            r#type: "parameter_list".to_string(),
                            value: "".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "function_declarator".to_string(),
                    value: "".to_string(),
                },
                AST {
                    children: vec![
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "{".to_string(),
                            value: "{".to_string(),
                        },
                        AST {
                            children: vec![],
                            span: None,
                            r#type: "}".to_string(),
                            value: "}".to_string(),
                        },
                    ],
                    span: None,
                    r#type: "compound_statement".to_string(),
                    value: "".to_string(),
                },
            ],
            span: None,
            r#type: "function_definition".to_string(),
            value: "".to_string(),
        };
        let destructor = transform_into_method(&destructor, "", "").unwrap();
        assert_eq!("~CastInfoServiceIf", destructor.method_name);
    }
}
