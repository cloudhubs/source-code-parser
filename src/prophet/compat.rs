use super::*;
use crate::ast::Block;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JSSAContext {
    #[serde(rename = "instanceType")]
    pub instance_type: InstanceType,
    // module_package_map: ModulePackageMap<'a>,
    pub succeeded: bool,
    pub class_names: Vec<String>,
    pub interface_names: Vec<String>,
    pub containers: Vec<ClassOrInterfaceComponent>, // classes + interfaces
    pub classes: Vec<ClassOrInterfaceComponent>,
    pub interfaces: Vec<ClassOrInterfaceComponent>,
    pub modules: Vec<ModuleComponent>,
    pub methods: Vec<MethodComponent>,
}

impl From<super::JSSAContext<'_>> for JSSAContext {
    fn from(other: super::JSSAContext<'_>) -> Self {
        let mut id = 2i64;

        let classes: Vec<_> = other
            .modules
            .iter()
            .flat_map(|module| module.classes.clone())
            .collect();

        let funcs: Vec<_> = {
            let constructors: Vec<_> = classes
                .iter()
                .flat_map(|class| class.constructors.clone())
                .collect();

            let module_functions: Vec<_> = other
                .modules
                .iter()
                .flat_map(|module| module.component.methods.clone())
                .collect();

            classes
                .iter()
                .flat_map(|class| class.component.methods.clone())
                .chain(constructors)
                .chain(module_functions)
                .collect()
        };

        let class_names = classes
            .iter()
            .filter(|component| matches!(component.declaration_type, ContainerType::Class))
            .map(|class| class.component.container_name.clone())
            .collect();

        let interface_names = classes
            .iter()
            .filter(|component| matches!(component.declaration_type, ContainerType::Interface))
            .map(|class| class.component.container_name.clone())
            .collect();

        let methods: Vec<_> = add_ids(&funcs, &mut id, |func, id| {
            MethodComponent::convert_compat(func, id)
        });
        let class_ix_list = add_ids(&classes, &mut id, |interface, id| {
            ClassOrInterfaceComponent::convert_compat(interface, id, &methods)
        });
        let interfaces: Vec<ClassOrInterfaceComponent> = class_ix_list
            .clone()
            .into_iter()
            .filter(|interface| matches!(interface.declaration_type, ContainerType::Interface))
            .collect();
        let classes: Vec<ClassOrInterfaceComponent> = class_ix_list
            .into_iter()
            .filter(|class| matches!(class.declaration_type, ContainerType::Class))
            .collect();
        let modules: Vec<_> = add_ids(&other.modules, &mut id, |module, id| {
            ModuleComponent::convert_compat(module, *id, &methods, &classes)
        });

        JSSAContext {
            instance_type: other.component.instance_type,
            succeeded: other.succeeded,
            class_names,
            interface_names,
            containers: {
                classes
                    .clone()
                    .into_iter()
                    .chain(interfaces.clone().into_iter())
                    .collect()
            },
            classes,
            interfaces,
            modules,
            methods,
        }
        // tracing::info!("{:#?}", ctx);
    }
}

fn add_ids<T, F, R>(components: &[T], id: &mut i64, id_adder: F) -> Vec<R>
where
    F: Fn(&T, &mut i64) -> R,
{
    components
        .iter()
        .map(|comp| {
            let ret = id_adder(comp, id);
            *id += 1;
            ret
        })
        .collect()
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct MethodComponent {
    pub id: i64,
    #[serde(flatten)]
    pub component: ComponentInfo,
    pub accessor: AccessorType,
    pub method_name: String,
    pub return_type: String,
    pub parameters: Vec<MethodParamComponent>,
    #[serde(rename = "static_method")]
    pub is_static: bool,
    #[serde(rename = "abstract_method")]
    pub is_abstract: bool,
    #[serde(rename = "final_method")]
    pub is_final: bool,
    #[serde(rename = "subroutines")]
    pub sub_methods: Vec<MethodComponent>,
    #[serde(rename = "subComponents")]
    pub sub_components: Vec<ComponentType>,
    pub annotations: Vec<AnnotationComponent>,
    pub line_count: i32,
    pub line_begin: i32,
    pub line_end: i32,
    pub body: Option<Block>,
}

impl MethodComponent {
    fn convert_compat(other: &super::MethodComponent, id: &mut i64) -> MethodComponent {
        let annotations = AnnotationComponent::convert_all(&other.annotations, id);

        let parameters: Vec<MethodParamComponent> = other
            .parameters
            .clone()
            .iter()
            .map(|param| MethodParamComponent::convert_compat(param, id))
            .collect();
        let sub_components = parameters
            .clone()
            .into_iter()
            .map(ComponentType::MethodParamComponent)
            .chain(
                annotations
                    .clone()
                    .into_iter()
                    .map(ComponentType::AnnotationComponent),
            )
            .collect();

        // Add the ID onto the end of meta names
        let component = ComponentInfo {
            path: format!(
                "{}::MethodDeclaration::{}",
                other.component.path, other.method_name
            ),
            instance_name: format!("MethodInfoComponent::{}", id),
            ..other.component.clone()
        };

        MethodComponent {
            id: *id,
            component,
            accessor: other.accessor.clone(),
            method_name: other.method_name.clone(),
            return_type: other.return_type.clone(),
            parameters, // add to subcomponents as well
            is_static: other.is_static,
            is_abstract: other.is_abstract,
            is_final: other.is_final,
            sub_methods: vec![],
            sub_components,
            annotations, // add to subcomponents as well
            line_count: other.line_count,
            line_begin: other.line_begin,
            line_end: other.line_end,
            body: other.body.clone(),
        }
    }

    fn is_equiv(&self, other: &super::MethodComponent) -> bool {
        self.component.is_equiv(&other.component)
            && self.accessor == other.accessor
            && self.method_name == other.method_name
            && self.return_type == other.return_type
            && self.parameters.len() == other.parameters.len()
            && self.is_static == other.is_static
            && self.is_abstract == other.is_abstract
            && self.is_final == other.is_final
            && self.annotations.len() == other.annotations.len()
            && self.line_count == other.line_count
            && self.line_begin == other.line_begin
            && self.line_end == other.line_end
            && self.body == other.body
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct MethodParamComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    pub annotation: Vec<AnnotationComponent>,
    pub r#type: String,
    pub parameter_name: String,
}
impl MethodParamComponent {
    fn convert_compat(param: &super::MethodParamComponent, id: &mut i64) -> MethodParamComponent {
        let annotation = match &param.annotation {
            Some(annotation_list) => AnnotationComponent::convert_all(annotation_list, id),
            _ => vec![],
        };
        MethodParamComponent {
            component: ComponentInfo {
                instance_name: format!("{}::MethodParamComponent", param.component.instance_name),
                ..param.component.clone()
            },
            annotation,
            r#type: param.r#type.clone(),
            parameter_name: param.parameter_name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct AnnotationComponent {
    pub id: i64,
    #[serde(flatten)]
    pub annotation: super::AnnotationComponent,
}
impl AnnotationComponent {
    fn convert_compat(other: &super::AnnotationComponent, id: &i64) -> AnnotationComponent {
        AnnotationComponent {
            id: *id,
            annotation: other.clone(),
        }
    }

    fn convert_all(
        annotations: &[super::AnnotationComponent],
        id: &mut i64,
    ) -> Vec<AnnotationComponent> {
        add_ids(annotations, id, |anno, id| {
            AnnotationComponent::convert_compat(anno, id)
        })
    }
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct ModuleComponent {
    // can contain functions here
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub module_name: String,
    #[serde(rename = "moduleStereotype")]
    pub module_stereotype: ModuleStereotype,
    pub class_names: Vec<String>,
    pub interface_names: Vec<String>,
    pub containers: Vec<ClassOrInterfaceComponent>,
    pub classes: Vec<ClassOrInterfaceComponent>,
    pub interfaces: Vec<ClassOrInterfaceComponent>,
}

impl ModuleComponent {
    fn convert_compat(
        other: &super::ModuleComponent,
        id: i64,
        methods: &[MethodComponent],
        classes: &[ClassOrInterfaceComponent],
    ) -> ModuleComponent {
        let class_names = other
            .classes
            .iter()
            .filter(|component| matches!(component.declaration_type, ContainerType::Class))
            .map(|class| class.component.container_name.clone())
            .collect();

        let interface_names = other
            .classes
            .iter()
            .filter(|component| matches!(component.declaration_type, ContainerType::Interface))
            .map(|class| class.component.container_name.clone())
            .collect();

        let class_ids: Vec<_> = classes
            .iter()
            .filter(|class| {
                other
                    .classes
                    .iter()
                    .filter(|other_class| {
                        matches!(other_class.declaration_type, ContainerType::Class)
                    })
                    .any(|other_class| class.is_equiv(other_class))
            })
            .cloned()
            .collect();

        let interface_ids: Vec<_> = classes
            .iter()
            .filter(|class| {
                other
                    .classes
                    .iter()
                    .filter(|other_class| {
                        matches!(other_class.declaration_type, ContainerType::Interface)
                    })
                    .any(|other_class| class.is_equiv(other_class))
            })
            .cloned()
            .collect();

        ModuleComponent {
            component: ContainerComponent::convert_compat(&other.component, id, methods, &[], true),
            module_name: other.module_name.clone(),
            module_stereotype: other.module_stereotype.clone(),
            class_names,
            interface_names,
            containers: class_ids
                .clone()
                .into_iter()
                .chain(interface_ids.clone().into_iter())
                .collect(),
            classes: class_ids,
            interfaces: interface_ids,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct FieldComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    pub annotations: Vec<AnnotationComponent>,
    pub variables: Vec<String>,
    pub field_name: String,
    pub accessor: AccessorType,
    #[serde(rename = "static")]
    pub is_static: bool,
    #[serde(rename = "final")]
    pub is_final: bool,
    #[serde(rename = "default_value_string")]
    pub default_value: String,
    pub r#type: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ClassOrInterfaceComponent {
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub declaration_type: ContainerType,
    pub annotations: Vec<AnnotationComponent>,
    pub constructors: Vec<MethodComponent>,
    #[serde(rename = "fieldComponents")]
    pub field_components: Vec<FieldComponent>,
}

impl ClassOrInterfaceComponent {
    fn convert_compat(
        other: &super::ClassOrInterfaceComponent,
        id: &mut i64,
        methods: &[MethodComponent],
    ) -> ClassOrInterfaceComponent {
        let constructors: Vec<_> = methods
            .iter()
            .filter(|method| {
                other
                    .constructors
                    .iter()
                    .any(|constructor| method.is_equiv(constructor))
            })
            .cloned()
            .collect();

        let methods: Vec<_> = methods
            .iter()
            .map(|method| MethodComponent {
                component: ComponentInfo {
                    // Adjust name to be ClassName::ClassComponent::MethodInfoComponent::ID
                    instance_name: format!(
                        "{}::ClassComponent::{}",
                        other.component.container_name, method.component.instance_name
                    ),
                    ..method.component.clone()
                },
                ..method.clone()
            })
            .collect();

        let annotations: Vec<_> = AnnotationComponent::convert_all(&other.annotations, id);

        // The instance_name should have already been adjusted to
        // name::ClassComponent or name::InterfaceComponent
        ClassOrInterfaceComponent {
            component: ContainerComponent::convert_compat(
                &other.component,
                *id,
                &methods,
                &annotations,
                false,
            ),
            declaration_type: other.declaration_type.clone(),
            annotations,
            constructors,
            field_components: other
                .field_components
                .iter()
                .map(|f| FieldComponent {
                    component: ComponentInfo {
                        instance_name: format!("{}::FieldComponent", f.component.instance_name),
                        ..f.component.clone()
                    },
                    annotations: AnnotationComponent::convert_all(&f.annotations, id),
                    variables: f.variables.clone(),
                    field_name: f.field_name.clone(),
                    accessor: f.accessor.clone(),
                    is_static: f.is_static,
                    is_final: f.is_final,
                    default_value: f.default_value.clone(),
                    r#type: f.r#type.clone(),
                })
                .collect(),
        }
    }

    fn is_equiv(&self, other: &super::ClassOrInterfaceComponent) -> bool {
        self.component.is_equiv(&other.component)
            && self.declaration_type == other.declaration_type
            && self.field_components.len() == other.field_components.len()
            && self.constructors.len() == other.constructors.len()
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ContainerComponent {
    pub id: i64,
    #[serde(flatten)]
    pub component: ComponentInfo,
    pub accessor: AccessorType,
    pub stereotype: ContainerStereotype,
    pub methods: Vec<MethodComponent>,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "lineCount")]
    pub line_count: i32,
    // I don't think this is actually used in Prophet
    // #[serde(rename = "rawSource")]
    // raw_source: &'a str,
    #[serde(rename = "subComponents")]
    pub sub_components: Vec<ComponentType>,
}

impl ContainerComponent {
    fn convert_compat(
        other: &super::ContainerComponent,
        id: i64,
        methods: &[MethodComponent],
        annotations: &[AnnotationComponent],
        is_module: bool,
    ) -> ContainerComponent {
        let methods: Vec<_> = methods
            .iter()
            .filter(|method| {
                other
                    .methods
                    .iter()
                    .any(|other_method| method.is_equiv(other_method))
            })
            .cloned()
            .collect();

        let sub_components = methods
            .clone()
            .into_iter()
            .map(ComponentType::MethodComponent)
            .chain(
                annotations
                    .iter()
                    .cloned()
                    .map(ComponentType::AnnotationComponent),
            )
            .collect();

        // Only modules have their ID appended to their package name it seems
        let package_name = if is_module {
            format!("{}::{}", other.component.package_name, id)
        } else {
            other.component.package_name.clone()
        };

        // Add the ID onto the end of meta names
        let component = ComponentInfo {
            instance_name: format!("{}::{}", other.component.instance_name, id),
            package_name,
            ..other.component.clone()
        };

        ContainerComponent {
            id,
            component,
            accessor: other.accessor.clone(),
            stereotype: other.stereotype.clone(),
            methods,
            container_name: other.container_name.clone(),
            line_count: other.line_count,
            sub_components,
        }
    }

    fn is_equiv(&self, other: &super::ContainerComponent) -> bool {
        self.accessor == other.accessor
            && self.stereotype == other.stereotype
            && self.container_name == other.container_name // todo: this won't work once we add ::{id}
            && self.line_count == other.line_count
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(untagged)]
pub enum ComponentType {
    ClassOrInterfaceComponent(ClassOrInterfaceComponent),
    AnnotationComponent(AnnotationComponent),
    MethodComponent(MethodComponent),
    ModuleComponent(ModuleComponent),
    FieldComponent(FieldComponent),
    MethodParamComponent(MethodParamComponent),
}
