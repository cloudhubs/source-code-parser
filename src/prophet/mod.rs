mod component;
pub use component::*;

mod context;
pub use context::*;

mod model;
pub use model::*;

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
                    },
                    accessor: AccessorType::Default,
                    method_name: "function",
                    return_type: "void",
                    parameters: vec![MethodParamComponent {
                        component: ComponentInfo {
                            path: "/path/module_name".to_string(),
                            package_name: "module_name".to_string(),
                            instance_name: "String".to_string(),
                            instance_type: InstanceType::ModuleComponent,
                        },
                        annotation: None,
                        parameter_type: "String",
                        parameter_name: "str",
                    }],
                    is_static: false,
                    is_abstract: false,
                    sub_methods: vec![],
                    annotations: vec![AnnotationComponent {
                        component: ComponentInfo {
                            path: "/path/module_name".to_string(),
                            package_name: "module_name".to_string(),
                            instance_name: "@PostMapping".to_string(),
                            instance_type: InstanceType::AnnotationComponent,
                        },
                        name: "@PostMapping",
                        annotation_meta_model: "??",
                        meta_model_field_name: "??",
                        key_value_pairs: vec![],
                        value: "/endpoint",
                    }],
                    line_count: 1,
                    line_begin: 1,
                    line_end: 2,
                }],
                line_count: 3,
            },
            module_name: "module_name".to_string(),
            path: "/path/module_name".to_string(),
            module_stereotype: ModuleStereotype::Repository,
            classes: vec![ClassOrInterfaceComponent {
                component: ContainerComponent {
                    component: ComponentInfo {
                        path: "/path/module_name/class".to_string(),
                        package_name: "module_name".to_string(),
                        instance_name: "class".to_string(),
                        instance_type: InstanceType::ClassComponent,
                    },
                    accessor: AccessorType::Default,
                    stereotype: ContainerStereotype::Service,
                    methods: vec![],
                    container_name: "class".to_string(),
                    line_count: 3,
                },
                path: "/path/module_name/class".to_string(),
                declaration_type: ContainerType::Class,
                annotations: vec![],
                stereotype: ContainerStereotype::Service,
                methods: vec![],
                constructors: None,
                field_components: Some(vec![FieldComponent {
                    component: ComponentInfo {
                        path: "/path/module_name/class".to_string(),
                        package_name: "module_name".to_string(),
                        instance_name: "field_name".to_string(),
                        instance_type: InstanceType::FieldComponent,
                    },
                    annotations: vec![],
                    variables: vec!["field_name"],
                    field_name: "field_name",
                    accessor: AccessorType::Private,
                    is_static: false,
                    is_final: false,
                    default_value: "",
                    r#type: "String",
                }]),
            }],
            interfaces: vec![],
        }];
        let actual = json!(JSSAContext {
            component: ComponentInfo {
                path: "/path".to_string(),
                package_name: "repository".to_string(),
                instance_name: "repository".to_string(),
                instance_type: InstanceType::AnalysisComponent,
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
