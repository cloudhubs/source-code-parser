use super::*;
use crate::ast::Block;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JSSAContext<'a> {
    #[serde(flatten)]
    pub instace_type: ComponentInfo,
    // module_package_map: ModulePackageMap<'a>,
    pub succeeded: bool,
    pub root_path: &'a str,
    pub class_names: Vec<String>,
    pub interface_names: Vec<String>,
    pub containers: Vec<i32>,
    pub classes: Vec<ClassOrInterfaceComponent>,
    pub interfaces: Vec<ClassOrInterfaceComponent>,
    pub modules: Vec<ModuleComponent>,
    pub methods: Vec<i32>,
}

impl From<super::JSSAContext<'_>> for JSSAContext<'_> {
    fn from(_: super::JSSAContext<'_>) -> Self {
        todo!()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct MethodComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,

    // excluding the IDs for now since I'm not sure why they're here
    // pub id: i64,
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
    pub annotations: Vec<AnnotationComponent>,
    pub line_count: i32,
    pub line_begin: i32,
    pub line_end: i32,
    pub body: Option<Block>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MethodParamComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    // r#type: ??? -- this is Class<?> in prophet, not sure if used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<AnnotationComponent>>,
    pub parameter_type: String,
    pub parameter_name: String,
}

#[derive(Debug, Serialize)]
pub struct ModuleComponent {
    // can contain functions here
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub module_name: String,
    pub path: String,
    #[serde(rename = "moduleStereotype")]
    pub module_stereotype: ModuleStereotype,
    // class_names, interface_names, method_names
    // containers
    pub classes: Vec<ClassOrInterfaceComponent>,
    pub interfaces: Vec<ClassOrInterfaceComponent>,
}

#[derive(Debug, Serialize)]
pub struct ClassOrInterfaceComponent {
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
