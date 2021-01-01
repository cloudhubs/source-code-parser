use super::*;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ComponentInfo {
    pub path: String,
    pub package_name: String,
    pub instance_name: String,
    pub instance_type: InstanceType,
    // I don't know whether this part is really necessary?
    // It just makes things a lot more complicated if so.
    // sub_components: Vec<ComponentType<'a>>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ComponentType {
    ClassOrInterfaceComponent(ClassOrInterfaceComponent),
    // AnnotationComponent(AnnotationComponent<'a>),
    MethodComponent(MethodComponent),
    ModuleComponent(ModuleComponent),
    // FieldComponent(FieldComponent<'a>),
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
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
    #[serde(rename = "subroutines")]
    pub sub_methods: Vec<MethodComponent>,
    pub annotations: Vec<AnnotationComponent>,
    pub line_count: i32,
    pub line_begin: i32,
    pub line_end: i32,
    // Statements were @JsonIgnore for some reason..
    // statements: Vec<&'a str>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct MethodParamComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    // r#type: ??? -- this is Class<?> in prophet
    pub annotation: Option<AnnotationComponent>,
    pub parameter_type: String,
    pub parameter_name: String,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
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

impl ModuleComponent {
    pub fn new(name: String, path: String) -> Self {
        let info = ComponentInfo {
            path: path.clone(),
            package_name: name.clone(),
            instance_name: name.clone(),
            instance_type: InstanceType::ModuleComponent,
        };
        let container = ContainerComponent {
            component: info,
            accessor: AccessorType::Default,
            stereotype: ContainerStereotype::Module,
            methods: vec![],
            container_name: name.clone(),
            line_count: 0,
        };
        let module = ModuleComponent {
            component: container,
            module_name: name,
            path,
            module_stereotype: ModuleStereotype::Controller,
            classes: vec![],
            interfaces: vec![],
        };
        module
    }
}

// #[derive(Debug, Eq, PartialEq, Serialize)]
// pub struct ModulePackageMap<'a> {
//     module_path_map: Vec<ModuleComponent<'a>>,
// }

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ContainerComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    // pub id: i64,
    pub accessor: AccessorType,
    pub stereotype: ContainerStereotype,
    pub methods: Vec<MethodComponent>,
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[serde(rename = "lineCount")]
    pub line_count: i32,
    // Not including this would also make things easier
    // #[serde(rename = "rawSource")]
    // raw_source: &'a str,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ClassOrInterfaceComponent {
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub declaration_type: ContainerType,
    pub annotations: Vec<AnnotationComponent>,
    pub stereotype: ContainerStereotype,

    pub methods: Vec<MethodComponent>,

    // Class-specific fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructors: Option<Vec<MethodComponent>>,
    #[serde(rename = "fieldComponents", skip_serializing_if = "Option::is_none")]
    pub field_components: Option<Vec<FieldComponent>>,
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
pub struct FieldComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    pub annotations: Vec<AnnotationComponent>,
    /// The declared variables. e.g. int x, y, z;
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
    // is_collection -- may make more sense as a field due to language differences
}

// For some reason prophet-utils relies on an actual javaparser AnnotationExpr instead of putting the info here. Needs fix.
#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct AnnotationComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    /// The annotation name as a string, including @
    pub name: String,
    #[serde(rename = "annotationMetaModel")]
    pub annotation_meta_model: String,
    #[serde(rename = "metaModelFieldName")]
    pub meta_model_field_name: String,
    // For now I mimiced the data structures used but this should really just be
    // a HashMap<&'a str, &'a str> and we could just implement serde::Serialize manually.
    pub key_value_pairs: Vec<AnnotationValuePair>,
    pub value: String,
}

// Seems useless since it can be represented by the other component.
// #[derive(Debug, Eq, PartialEq, Serialize)]
// pub struct InterfaceComponent<'a> {
//     #[serde(flatten)]
//     container: ClassOrInterfaceComponent<'a>,
// }
