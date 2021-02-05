use super::*;
use crate::ast::Block;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JSSAContext {
    pub id: i64,
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
        let classes: Vec<_> = other
            .modules
            .iter()
            .flat_map(|module| module.classes.clone())
            .collect();

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

        let funcs: Vec<_> = classes
            .iter()
            .flat_map(|class| class.component.methods.clone())
            .chain(constructors)
            .chain(module_functions)
            .collect();

        JSSAContext {
            id: 1,
            instance_type: other.component.instance_type,
            succeeded: other.succeeded,
            class_names: vec![],
            interface_names: vec![],
            containers: vec![],
            classes: vec![],
            interfaces: vec![],
            modules: vec![],
            methods: vec![],
        }
    }
}

#[derive(Debug, Serialize, Clone)]
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
    pub annotations: Vec<AnnotationComponent>,
    pub line_count: i32,
    pub line_begin: i32,
    pub line_end: i32,
    pub body: Option<Block>,
}

#[derive(Debug, Serialize)]
pub struct ModuleComponent {
    pub id: i64,
    // can contain functions here
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub module_name: String,
    pub path: String, // dupe field I think
    #[serde(rename = "moduleStereotype")]
    pub module_stereotype: ModuleStereotype,
    pub class_names: Vec<String>,
    pub interface_names: Vec<String>,
    pub containers: Vec<i32>,
    pub classes: Vec<ClassOrInterfaceComponent>,
    pub interfaces: Vec<ClassOrInterfaceComponent>,
}

#[derive(Debug, Serialize)]
pub struct ClassOrInterfaceComponent {
    pub id: i64,
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub declaration_type: ContainerType,
    pub annotations: Vec<AnnotationComponent>,

    // Class-specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructors: Option<Vec<MethodComponent>>,
    #[serde(rename = "fieldComponents", skip_serializing_if = "Option::is_none")]
    pub field_components: Option<Vec<FieldComponent>>,
}
