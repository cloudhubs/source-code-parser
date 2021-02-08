use super::*;
use crate::ast::Block;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct JSSAContext {
    pub instance_type: InstanceType,
    // module_package_map: ModulePackageMap<'a>,
    pub succeeded: bool,
    pub class_names: Vec<String>,
    pub interface_names: Vec<String>,
    pub containers: Vec<i64>,
    pub classes: Vec<ClassOrInterfaceComponent>,
    pub interfaces: Vec<ClassOrInterfaceComponent>,
    pub modules: Vec<ModuleComponent>,
    pub methods: Vec<i64>,
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
                .flat_map(|constructors| constructors)
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
            .filter(|component| {
                component.constructors.is_some() && component.field_components.is_some()
            })
            .map(|class| class.component.container_name.clone())
            .collect();

        let interface_names = classes
            .iter()
            .filter(|component| {
                component.constructors.is_none() && component.field_components.is_none()
            })
            .map(|class| class.component.container_name.clone())
            .collect();

        let method_ids = map_ids(&funcs, &mut id);
        let class_ids = map_ids(&classes, &mut id);
        let module_ids = map_ids(&other.modules, &mut id);

        let methods: Vec<_> = method_ids
            .iter()
            .map(|(ndx, id)| {
                let func = funcs.get(*ndx).expect(&*format!(
                    "Ivalid index {} into funcs array during prophet compat conversion",
                    ndx
                ));
                MethodComponent::convert_compat(func, *id)
            })
            .collect();

        let interfaces: Vec<_> = class_ids
            .iter()
            .map(|(ndx, id)| {
                let interface = classes.get(*ndx).expect(&*format!(
                    "Ivalid index {} into funcs classes during prophet compat conversion",
                    ndx
                ));
                if interface.constructors.is_none() && interface.field_components.is_none() {
                    Some(ClassOrInterfaceComponent::convert_compat(
                        interface, *id, &methods,
                    ))
                } else {
                    None
                }
            })
            .flat_map(|interface| interface)
            .collect();

        let classes: Vec<_> = class_ids
            .iter()
            .map(|(ndx, id)| {
                let class = classes.get(*ndx).expect(&*format!(
                    "Ivalid index {} into funcs classes during prophet compat conversion",
                    ndx
                ));
                if class.constructors.is_some() && class.field_components.is_some() {
                    Some(ClassOrInterfaceComponent::convert_compat(
                        class, *id, &methods,
                    ))
                } else {
                    None
                }
            })
            .flat_map(|class| class)
            .collect();

        let modules: Vec<_> = module_ids
            .iter()
            .map(|(ndx, id)| {
                let module = other.modules.get(*ndx).expect(&*format!(
                    "Ivalid index {} into modules array during prophet compat conversion",
                    ndx
                ));
                ModuleComponent::convert_compat(module, *id, &methods, &classes)
            })
            .collect();

        JSSAContext {
            instance_type: other.component.instance_type,
            succeeded: other.succeeded,
            class_names,
            interface_names,
            containers: class_ids
                .iter()
                .map(|(_, id)| *id)
                .chain(module_ids.iter().map(|(_, id)| *id))
                .collect(),
            classes,
            interfaces,
            modules,
            methods: method_ids.iter().map(|(_, id)| *id).collect(),
        }
    }
}

fn map_ids<T>(components: &Vec<T>, id: &mut i64) -> HashMap<usize, i64> {
    let mut ids = HashMap::new();
    for (i, _) in components.iter().enumerate() {
        ids.insert(i, *id);
        *id += 1;
    }
    ids
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
    pub sub_methods: Vec<i64>,
    pub sub_components: Vec<ComponentType>,
    pub annotations: Vec<AnnotationComponent>,
    pub line_count: i32,
    pub line_begin: i32,
    pub line_end: i32,
    pub body: Option<Block>,
}

impl MethodComponent {
    fn convert_compat(other: &super::MethodComponent, id: i64) -> MethodComponent {
        let sub_components = other
            .parameters
            .clone()
            .iter()
            .map(|param| ComponentType::MethodParamComponent(param.clone()))
            .chain(
                other
                    .annotations
                    .clone()
                    .iter()
                    .map(|annotation| ComponentType::AnnotationComponent(annotation.clone())),
            )
            .collect();

        // Add the ID onto the end of meta names
        let component = ComponentInfo {
            instance_name: format!("{}::{}", other.component.instance_name, id),
            ..other.component.clone()
        };

        MethodComponent {
            id,
            component,
            accessor: other.accessor.clone(),
            method_name: other.method_name.clone(),
            return_type: other.return_type.clone(),
            parameters: other.parameters.clone(), // add to subcomponents as well
            is_static: other.is_static,
            is_abstract: other.is_abstract,
            is_final: other.is_final,
            sub_methods: vec![],
            sub_components,
            annotations: other.annotations.clone(), // add to subcomponents as well
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
            && self.parameters == other.parameters
            && self.is_static == other.is_static
            && self.is_abstract == other.is_abstract
            && self.is_final == other.is_final
            && self.annotations == other.annotations
            && self.line_count == other.line_count
            && self.line_begin == other.line_begin
            && self.line_end == other.line_end
            && self.body == other.body
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
    pub containers: Vec<i64>,
    pub classes: Vec<i64>,
    pub interfaces: Vec<i64>,
}

impl ModuleComponent {
    fn convert_compat(
        other: &super::ModuleComponent,
        id: i64,
        methods: &Vec<MethodComponent>,
        classes: &Vec<ClassOrInterfaceComponent>,
    ) -> ModuleComponent {
        let class_names = other
            .classes
            .iter()
            .filter(|component| {
                component.constructors.is_some() && component.field_components.is_some()
            })
            .map(|class| class.component.container_name.clone())
            .collect();

        let interface_names = other
            .classes
            .iter()
            .filter(|component| {
                component.constructors.is_none() && component.field_components.is_none()
            })
            .map(|class| class.component.container_name.clone())
            .collect();

        let class_ids: Vec<_> = classes
            .iter()
            .filter(|class| {
                other
                    .classes
                    .iter()
                    .filter(|other_class| {
                        other_class.constructors.is_some() && other_class.field_components.is_some()
                    })
                    .find(|other_class| class.is_equiv(other_class))
                    .is_some()
            })
            .map(|class| class.component.id)
            .collect();

        let interface_ids: Vec<_> = classes
            .iter()
            .filter(|class| {
                other
                    .classes
                    .iter()
                    .filter(|other_class| {
                        other_class.constructors.is_none() && other_class.field_components.is_none()
                    })
                    .find(|other_class| class.is_equiv(other_class))
                    .is_some()
            })
            .map(|class| class.component.id)
            .collect();

        ModuleComponent {
            component: ContainerComponent::convert_compat(&other.component, id, methods, &vec![]),
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
pub struct ClassOrInterfaceComponent {
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub declaration_type: ContainerType,
    pub annotations: Vec<AnnotationComponent>,

    // Class-specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructors: Option<Vec<i64>>,
    #[serde(rename = "fieldComponents", skip_serializing_if = "Option::is_none")]
    pub field_components: Option<Vec<FieldComponent>>,
}

impl ClassOrInterfaceComponent {
    fn convert_compat(
        other: &super::ClassOrInterfaceComponent,
        id: i64,
        methods: &Vec<MethodComponent>,
    ) -> ClassOrInterfaceComponent {
        let constructors: Vec<_> = methods
            .iter()
            .filter(|method| match &other.constructors {
                Some(constructors) => constructors
                    .iter()
                    .find(|constructor| method.is_equiv(constructor))
                    .is_some(),
                None => false,
            })
            .map(|constructor| constructor.id)
            .collect();

        let constructors = if constructors.len() > 0 {
            Some(constructors)
        } else {
            None
        };

        ClassOrInterfaceComponent {
            component: ContainerComponent::convert_compat(
                &other.component,
                id,
                methods,
                &other.annotations,
            ),
            declaration_type: other.declaration_type.clone(),
            annotations: other.annotations.clone(),
            constructors,
            field_components: other.field_components.clone(),
        }
    }

    fn is_equiv(&self, other: &super::ClassOrInterfaceComponent) -> bool {
        self.component.is_equiv(&other.component)
            && self.declaration_type == other.declaration_type
            && self.field_components == other.field_components
            && self.constructors.is_some() == other.constructors.is_some()
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ContainerComponent {
    pub id: i64,
    #[serde(flatten)]
    pub component: ComponentInfo,
    pub accessor: AccessorType,
    pub stereotype: ContainerStereotype,
    pub methods: Vec<i64>,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "lineCount")]
    pub line_count: i32,
    // I don't think this is actually used in Prophet
    // #[serde(rename = "rawSource")]
    // raw_source: &'a str,
    pub sub_components: Vec<ComponentType>,
}

impl ContainerComponent {
    fn convert_compat(
        other: &super::ContainerComponent,
        id: i64,
        methods: &Vec<MethodComponent>,
        annotations: &Vec<AnnotationComponent>,
    ) -> ContainerComponent {
        let methods: Vec<_> = methods
            .iter()
            .filter(|method| {
                other
                    .methods
                    .iter()
                    .find(|other_method| method.is_equiv(other_method))
                    .is_some()
            })
            .map(|method| method.clone())
            .collect();

        let method_ids = methods.iter().map(|method| method.id).collect();

        let sub_components = methods
            .into_iter()
            .map(|method| ComponentType::MethodComponent(method))
            .chain(
                annotations
                    .clone()
                    .into_iter()
                    .map(|annotation| ComponentType::AnnotationComponent(annotation)),
            )
            .collect();

        // Add the ID onto the end of meta names
        let component = ComponentInfo {
            instance_name: format!("{}::{}", other.component.instance_name, id),
            package_name: format!("{}::{}", other.component.package_name, id),
            ..other.component.clone()
        };

        ContainerComponent {
            id,
            component,
            accessor: other.accessor.clone(),
            stereotype: other.stereotype.clone(),
            methods: method_ids,
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
