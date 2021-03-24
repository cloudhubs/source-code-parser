use super::*;
use crate::ast::Block;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ComponentInfo {
    pub path: String,
    pub package_name: String,
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    #[serde(rename = "instanceType")]
    pub instance_type: InstanceType,
    // This does not seem to be used in Prophet.
    // sub_components: Vec<ComponentType<'a>>,
}

impl ComponentInfo {
    pub fn is_equiv(&self, other: &ComponentInfo) -> bool {
        let equiv_path =
            self.path == format!("{}::MethodDeclaration::{}", other.path, other.instance_name);
        let equiv_pkg = self.package_name == other.package_name;
        let equiv_inst_type = self.instance_type == other.instance_type;

        // Instance name fields shouldn't be directly compared here since it gets mutated
        // to ClassName::ClassComponent::MethodInfoComponent::I'D where "other" is just
        // the method name. The path covers this part.

        equiv_path && equiv_pkg && equiv_inst_type
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

#[derive(Debug, Eq, Serialize, Clone)]
pub struct MethodParamComponent {
    #[serde(flatten)]
    pub component: ComponentInfo,
    // r#type: ??? -- this is Class<?> in prophet, not sure if used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<AnnotationComponent>>,
    /// The parameter type
    pub r#type: String,
    pub parameter_name: String,
}

impl PartialEq for MethodParamComponent {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.annotation == other.annotation
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct ModuleComponent {
    // can contain functions here
    #[serde(flatten)]
    pub component: ContainerComponent,
    pub module_name: String,
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
            instance_name: format!("{}::ModuleComponent", path),
            path,
            package_name: name.clone(),
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
            module_stereotype: ModuleStereotype::Fabricated,
            classes: vec![],
            interfaces: vec![],
        };
        module
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
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
    // I don't think this is actually used in Prophet
    // #[serde(rename = "rawSource")]
    // raw_source: &'a str,
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

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
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
    // is_collection -- may make sense as a field due to language differences
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
    // For now I mimiced the data structure here but this should really just be
    // a HashMap<&'a str, &'a str> with a manual serde::Serialize implementation.
    // I also am not sure where this is used.
    pub key_value_pairs: Vec<AnnotationValuePair>,
    pub value: String,
}
