use crate::parse::AST;
use crate::prophet::*;

pub fn find_components(ast: AST) -> Vec<ComponentType> {
    match &*ast.r#type {
        "function_definition" => match transform_into_method(ast) {
            Some(method) => vec![ComponentType::MethodComponent(method)],
            None => vec![],
        },
        _ => {
            let components: Vec<ComponentType> = ast
                .children
                .into_iter()
                .flat_map(|child| find_components(child))
                .collect();
            components
        }
    }
}

/// Transforms an AST with type label "function_definition" to a `MethodComponent`
fn transform_into_method(ast: AST) -> Option<MethodComponent> {
    // TODO: child type "compound_statement" for function block
    let ret = ast.children.iter().find(|child| match &*child.r#type {
        "primitive_type" | "scoped_type_identifier" | "type_identifier" => true,
        _ => false,
    });
    let ret_type = match ret {
        Some(ret) => type_ident(ret),
        None => "".to_string(),
    };

    let decl = ast
        .children
        .iter()
        .find(|child| child.r#type == "function_declarator")?;

    let identifier = decl
        .children
        .iter()
        .find(|child| match &*child.r#type {
            "scoped_identifier" | "identifier" => true,
            _ => false,
        })?;
    let fn_ident = func_ident(identifier);
    
    let parameter_list = decl
        .children
        .iter()
        .find(|child| child.r#type == "parameter_list")?;

    None
}

/// Get the value for a type identifier
fn type_ident(ast: &AST) -> String {
    match &*ast.r#type {
        "primitive_type" | "type_identifier" => ast.value.clone(),
        "scoped_type_identifier" | "scoped_namespace_identifier" => {
            let ret: String = ast
                .children
                .iter()
                .map(|child| match &*child.r#type {
                    "scoped_namespace_identifier" => type_ident(child),
                    _ => child.value.clone(),
                })
                .collect();
            ret
        }
        _ => "".to_string(),
    }
}

/// Get the value for a function identifier
fn func_ident(ast: &AST) -> String {
    match &*ast.r#type {
        "function_declarator" => {
            let ident = ast.children.iter().find(|child| match &*child.r#type {
                "scoped_identifier" | "identifier" => true,
                _ => false,
            });
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
        "identifier" => ast.value.clone(),
        _ => "".to_string(),
    }
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
}
