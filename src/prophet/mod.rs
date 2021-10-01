mod component;
pub use component::*;

mod context;
pub use context::*;

mod model;
pub use model::*;

pub mod compat;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    pub fn serialization_transformation() {
        let modules = vec![ModuleComponent {
            component: ContainerComponent {
                component: ComponentInfo {
                    path: "/path/module_name".to_string(),
                    package_name: "module_name".to_string(),
                    instance_name: "module_name".to_string(),
                    instance_type: InstanceType::ModuleComponent,
                    language: Language::Java,
                },
                accessor: AccessorType::Default,
                stereotype: ContainerStereotype::Repository,
                container_name: "module_name".to_string(),
                methods: vec![MethodComponent {
                    component: ComponentInfo {
                        path: "/path/module_name/file".to_string(),
                        package_name: "module_name".to_string(),
                        instance_name: "function".to_string(),
                        instance_type: InstanceType::MethodComponent,
                        language: Language::Java,
                    },
                    accessor: AccessorType::Default,
                    method_name: "function".to_string(),
                    return_type: "void".to_string(),
                    parameters: vec![MethodParamComponent {
                        component: ComponentInfo {
                            path: "/path/module_name".to_string(),
                            package_name: "module_name".to_string(),
                            instance_name: "String".to_string(),
                            instance_type: InstanceType::ModuleComponent,
                            language: Language::Java,
                        },
                        annotation: None,
                        r#type: "String".to_string(),
                        parameter_name: "str".to_string(),
                    }],
                    is_static: false,
                    is_abstract: false,
                    is_final: false,
                    sub_methods: vec![],
                    annotations: vec![AnnotationComponent {
                        component: ComponentInfo {
                            path: "/path/module_name".to_string(),
                            package_name: "module_name".to_string(),
                            instance_name: "@PostMapping".to_string(),
                            instance_type: InstanceType::AnnotationComponent,
                            language: Language::Java,
                        },
                        name: "@PostMapping".to_string(),
                        annotation_meta_model: "??".to_string(),
                        meta_model_field_name: "??".to_string(),
                        key_value_pairs: vec![],
                        value: "/endpoint".to_string(),
                    }],
                    line_count: 1,
                    line_begin: 1,
                    line_end: 2,
                    body: None,
                }],
                line_count: 3,
            },
            module_name: "module_name".to_string(),
            module_stereotype: ModuleStereotype::Repository,
            classes: vec![ClassOrInterfaceComponent {
                component: ContainerComponent {
                    component: ComponentInfo {
                        path: "/path/module_name/class".to_string(),
                        package_name: "module_name".to_string(),
                        instance_name: "class".to_string(),
                        instance_type: InstanceType::ClassComponent,
                        language: Language::Java,
                    },
                    accessor: AccessorType::Default,
                    stereotype: ContainerStereotype::Service,
                    methods: vec![],
                    container_name: "class".to_string(),
                    line_count: 3,
                },
                declaration_type: ContainerType::Class,
                annotations: vec![],
                constructors: vec![],
                field_components: vec![FieldComponent {
                    component: ComponentInfo {
                        path: "/path/module_name/class".to_string(),
                        package_name: "module_name".to_string(),
                        instance_name: "field_name".to_string(),
                        instance_type: InstanceType::FieldComponent,
                        language: Language::Java,
                    },
                    annotations: vec![],
                    variables: vec!["field_name".to_string()],
                    field_name: "field_name".to_string(),
                    accessor: AccessorType::Private,
                    is_static: false,
                    is_final: false,
                    default_value: "".to_string(),
                    r#type: "String".to_string(),
                    expression: None,
                }],
            }],
            interfaces: vec![],
        }];
        let _actual = json!(JSSAContext {
            component: ComponentInfo {
                path: "/path".to_string(),
                package_name: "repository".to_string(),
                instance_name: "repository".to_string(),
                instance_type: InstanceType::AnalysisComponent,
                language: Language::Java,
            },
            succeeded: true,
            root_path: "/path",
            modules,
        });

        // let expected = json!({
        //     "path": "/path/module_name",
        //     "package_name": "module_name",
        //     "instance_name": "module_name",
        //     "instance_type": "MODULECOMPONENT",
        //     "accessor": "DEFAULT",
        //     "stereotype": "REPOSITORY",
        //     "container_name": "module_name",
        //     "line_count": 3,
        //     "methods": [],
        // });

        // assert_eq!(expected, actual);
    }
}
