use serde::Serialize;
use super::*;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ComponentInfo<'a> {
    path: &'a str,
    package_name: &'a str,
    instance_name: &'a str,
    instance_type: InstanceType,
    // I don't know whether this part is really necessary?
    // It just makes things a lot more complicated if so.
    // sub_components: Vec<ComponentType<'a>>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ComponentType<'a> {
    ClassComponent(ClassComponent<'a>),
    InterfaceComponent(InterfaceComponent<'a>),
    AnnotationComponent(AnnotationComponent<'a>),
    MethodComponent(MethodComponent<'a>),
    ModuleComponent(ModuleComponent<'a>),
    FieldComponent(FieldComponent<'a>),
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MethodComponent<'a> {
    #[serde(flatten)]
    component: ComponentInfo<'a>,
    
    id: i64,
    accessor: AccessorType,
    method_name: &'a str,
    return_type: &'a str,
    parameters: Vec<MethodParamComponent<'a>>,
    #[serde(rename = "static_method")]
    is_static: bool,
    #[serde(rename = "abstract_method")]
    is_abstract: bool,
    #[serde(rename = "subroutines")]
    sub_methods: Vec<MethodComponent<'a>>,
    annotations: Vec<AnnotationComponent<'a>>,
    line_count: i32,
    line_begin: i32,
    line_end: i32,

    // Statements were @JsonIgnore for some reason..
    // statements: Vec<&'a str>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MethodParamComponent<'a> {
    // r#type: ??? -- this is Class<?> in prophet
    annotation: Option<AnnotationComponent<'a>>,
    parameter_type: &'a str,
    parameter_name: &'a str,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ModuleComponent<'a> {
    module_name: &'a str,
    path: &'a str,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ModulePackageMap<'a> {
    module_path_map: Vec<ModuleComponent<'a>>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ContainerComponent<'a> {
    #[serde(flatten)]
    component: ComponentInfo<'a>,
    id: i64,
    accessor: AccessorType,
    stereotype: ContainerStereotype,
    methods: Vec<MethodComponent<'a>>,
    #[serde(rename = "containerName")]
    container_name: &'a str,
    #[serde(rename = "lineCount")]
    line_count: i32,

    // Not including this would also make things easier
    // #[serde(rename = "rawSource")]
    // raw_source: &'a str,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ClassOrInterfaceComponent<'a> {
    #[serde(flatten)]
    component: ContainerComponent<'a>,
    path: &'a str,
    declaration_type: ContainerType,
    annotations: Vec<AnnotationComponent<'a>>,
    stereotype: ContainerStereotype,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ClassComponent<'a> {
    #[serde(flatten)]
    container: ClassOrInterfaceComponent<'a>,
    constructors: Vec<MethodComponent<'a>>,
    #[serde(rename = "fieldComponents")]
    field_components: Vec<FieldComponent<'a>>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct FieldComponent<'a> {
    #[serde(flatten)]
    component: ComponentInfo<'a>,
    annotations: Vec<AnnotationComponent<'a>>,
    /// The declared variables. e.g. int x, y, z;
    variables: Vec<&'a str>,
    field_name: &'a str,
    accessor: AccessorType,
    #[serde(rename = "static")]
    is_static: bool,
    #[serde(rename = "final")]
    is_final: bool,
    #[serde(rename = "default_value_string")]
    default_value: &'a str,
    r#type: &'a str,
}

// For some reason prophet-utils relies on an actual javaparser AnnotationExpr instead of putting the info here. Needs fix.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct AnnotationComponent<'a> {
    /// The annotation name as a string, including @
    name: &'a str,
    #[serde(rename = "annotationMetaModel")]
    annotation_meta_model: &'a str,
    #[serde(rename = "metaModelFieldName")]
    meta_model_field_name: &'a str,
    // For now I mimiced the data structures used but this should really just be
    // a HashMap<&'a str, &'a str> and we could just implement serde::Serialize manually.
    key_value_pairs: Vec<AnnotationValuePair<'a>>,
    value: &'a str,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct InterfaceComponent<'a> {
    #[serde(flatten)]
    container: ClassOrInterfaceComponent<'a>,
}