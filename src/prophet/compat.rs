use super::*;
use crate::ast::Block;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct JSSAContext {
    #[serde(flatten)]
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

        let method_ids = map_ids(&funcs, &mut id);
        let class_ids = map_ids(&classes, &mut id);
        let module_ids = map_ids(&other.modules, &mut id);

        let methods: Vec<MethodComponent> = method_ids
            .iter()
            .map(|(ndx, id)| {
                let func = funcs.get(*ndx).expect(&*format!(
                    "Ivalid index {} into funcs array during prophet compat conversion",
                    ndx
                ));
                MethodComponent::convert_compat(func, *id)
            })
            .collect();

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
            classes: vec![],
            interfaces: vec![],
            modules: vec![],
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

        MethodComponent {
            id,
            component: other.component.clone(),
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ContainerComponent {
    pub id: i64,
    #[serde(flatten)]
    pub component: ComponentInfo,
    pub accessor: AccessorType,
    pub stereotype: ContainerStereotype,
    pub methods: Vec<i32>,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "lineCount")]
    pub line_count: i32,
    // I don't think this is actually used in Prophet
    // #[serde(rename = "rawSource")]
    // raw_source: &'a str,
    pub sub_components: Vec<ComponentType>,
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
