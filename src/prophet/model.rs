use rust_code_analysis::LANG;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum InstanceType {
    #[serde(rename = "CLASSCOMPONENT")]
    ClassComponent,
    #[serde(rename = "INTERFACECOMPONENT")]
    InterfaceComponent,
    #[serde(rename = "ANNOTATIONCOMPONENT")]
    AnnotationComponent,
    #[serde(rename = "METHODCOMPONENT")]
    MethodComponent,
    #[serde(rename = "MODULECOMPONENT")]
    ModuleComponent,
    #[serde(rename = "DIRECTORYCOMPONENT")]
    DirectoryComponent,
    #[serde(rename = "ANALYSISCOMPONENT")]
    AnalysisComponent,
    #[serde(rename = "FIELDCOMPONENT")]
    FieldComponent,
    #[serde(rename = "IMPORTCOMPONENT")]
    ImportComponent,
    #[serde(rename = "METHODPARAMCOMPONENT")]
    MethodParamComponent,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContainerStereotype {
    Fabricated,
    Controlled,
    Service,
    Response,
    Entity,
    Repository,
    Bean,
    Module,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContainerType {
    Class,
    Module,
    Interface,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct AnnotationValuePair {
    key: String,
    value: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum AccessorType {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "DEFAULT")]
    Default,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum Language {
    Java,
    Cpp,
    Python,
    Go,
    // ...
    #[serde(rename = "N/A")]
    Unknown,
}

impl Into<Language> for LANG {
    fn into(self) -> Language {
        match self {
            LANG::Cpp => Language::Cpp,
            LANG::Java => Language::Java,
            LANG::Python => Language::Python,
            LANG::Go => Language::Go,
            _ => Language::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModuleStereotype {
    Fabricated,
    Controller,
    Service,
    Response,
    Entity,
    Repository, // The rest are for future expansion
                /*
                Bounded,
                Specification,
                ClosureOfOperations,
                Aggregation
                */
}
