use super::*;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ComponentInfo<'a> {
    pub path: String,
    pub package_name: &'a str,
    pub instance_name: &'a str,
    pub instance_type: InstanceType,
    // I don't know whether this part is really necessary?
    // It just makes things a lot more complicated if so.
    // sub_components: Vec<ComponentType<'a>>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ComponentType<'a> {
    ClassOrInterfaceComponent(ClassOrInterfaceComponent<'a>),
    AnnotationComponent(AnnotationComponent<'a>),
    MethodComponent(MethodComponent<'a>),
    ModuleComponent(ModuleComponent<'a>),
    FieldComponent(FieldComponent<'a>),
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MethodComponent<'a> {
    #[serde(flatten)]
    pub component: ComponentInfo<'a>,

    // excluding the IDs for now since I'm not sure why they're here
    // pub id: i64,
    pub accessor: AccessorType,
    pub method_name: &'a str,
    pub return_type: &'a str,
    pub parameters: Vec<MethodParamComponent<'a>>,
    #[serde(rename = "static_method")]
    pub is_static: bool,
    #[serde(rename = "abstract_method")]
    pub is_abstract: bool,
    #[serde(rename = "subroutines")]
    pub sub_methods: Vec<MethodComponent<'a>>,
    pub annotations: Vec<AnnotationComponent<'a>>,
    pub line_count: i32,
    pub line_begin: i32,
    pub line_end: i32,
    // Statements were @JsonIgnore for some reason..
    // statements: Vec<&'a str>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MethodParamComponent<'a> {
    pub component: ComponentInfo<'a>,
    // r#type: ??? -- this is Class<?> in prophet
    pub annotation: Option<AnnotationComponent<'a>>,
    pub parameter_type: &'a str,
    pub parameter_name: &'a str,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ModuleComponent<'a> {
    // can contain functions here
    #[serde(flatten)]
    pub component: ContainerComponent<'a>,
    pub module_name: &'a str,
    pub path: String,
    #[serde(rename = "moduleStereotype")]
    pub module_stereotype: ModuleStereotype,
    // class_names, interface_names, method_names
    // containers
    pub classes: Vec<ClassOrInterfaceComponent<'a>>,
    pub interfaces: Vec<ClassOrInterfaceComponent<'a>>,
}

// #[derive(Debug, Eq, PartialEq, Serialize)]
// pub struct ModulePackageMap<'a> {
//     module_path_map: Vec<ModuleComponent<'a>>,
// }

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ContainerComponent<'a> {
    #[serde(flatten)]
    pub component: ComponentInfo<'a>,
    // pub id: i64,
    pub accessor: AccessorType,
    pub stereotype: ContainerStereotype,
    pub methods: Vec<MethodComponent<'a>>,
    #[serde(rename = "containerName")]
    pub container_name: &'a str,
    #[serde(rename = "lineCount")]
    pub line_count: i32,
    // Not including this would also make things easier
    // #[serde(rename = "rawSource")]
    // raw_source: &'a str,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ClassOrInterfaceComponent<'a> {
    #[serde(flatten)]
    pub component: ContainerComponent<'a>,
    pub path: String,
    pub declaration_type: ContainerType,
    pub annotations: Vec<AnnotationComponent<'a>>,
    pub stereotype: ContainerStereotype,

    pub methods: Vec<MethodComponent<'a>>,

    // Class-specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructors: Option<Vec<MethodComponent<'a>>>,
    #[serde(rename = "fieldComponents", skip_serializing_if = "Option::is_none")]
    pub field_components: Option<Vec<FieldComponent<'a>>>,
}

// #[derive(Debug, Eq, PartialEq, Serialize)]
// pub struct ClassComponent<'a> {
//     #[serde(flatten)]
//     container: ClassOrInterfaceComponent<'a>,
//     constructors: Vec<MethodComponent<'a>>,
//     #[serde(rename = "fieldComponents")]
//     field_components: Vec<FieldComponent<'a>>,
// }

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct FieldComponent<'a> {
    #[serde(flatten)]
    pub component: ComponentInfo<'a>,
    pub annotations: Vec<AnnotationComponent<'a>>,
    /// The declared variables. e.g. int x, y, z;
    pub variables: Vec<&'a str>,
    pub field_name: &'a str,
    pub accessor: AccessorType,
    #[serde(rename = "static")]
    pub is_static: bool,
    #[serde(rename = "final")]
    pub is_final: bool,
    #[serde(rename = "default_value_string")]
    pub default_value: &'a str,
    pub r#type: &'a str,
    // is_collection -- may make more sense as a field due to language differences
}

// For some reason prophet-utils relies on an actual javaparser AnnotationExpr instead of putting the info here. Needs fix.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct AnnotationComponent<'a> {
    pub component: ComponentInfo<'a>,
    /// The annotation name as a string, including @
    pub name: &'a str,
    #[serde(rename = "annotationMetaModel")]
    pub annotation_meta_model: &'a str,
    #[serde(rename = "metaModelFieldName")]
    pub meta_model_field_name: &'a str,
    // For now I mimiced the data structures used but this should really just be
    // a HashMap<&'a str, &'a str> and we could just implement serde::Serialize manually.
    pub key_value_pairs: Vec<AnnotationValuePair<'a>>,
    pub value: &'a str,
}

// Seems useless since it can be represented by the other component.
// #[derive(Debug, Eq, PartialEq, Serialize)]
// pub struct InterfaceComponent<'a> {
//     #[serde(flatten)]
//     container: ClassOrInterfaceComponent<'a>,
// }
