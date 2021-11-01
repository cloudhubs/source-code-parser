use rust_code_analysis::LANG;
use serde::{Deserialize, Serialize};
use source_code_parser_macro::{ChildFields, NodeLanguage};

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

#[derive(Debug, Eq, PartialEq, Serialize, Clone, NodeLanguage, ChildFields)]
pub struct AnnotationValuePair {
    #[serde(skip_serializing)]
    pub language: Language,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum AccessorType {
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "PROTECTED")]
    Protected,
    #[serde(rename = "DEFAULT")]
    Default,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Language {
    Java,
    Cpp,
    Python,
    Go,
    // ...
    #[serde(rename = "N/A")]
    Unknown,
}

impl Language {
    /// TODO figure out how to handle correctly
    pub fn get_index(lang: &Language) -> usize {
        match lang {
            Language::Java => 1,
            Language::Cpp => 2,
            Language::Python => 3,
            Language::Go => 4,
            Language::Unknown => 0,
        }
    }
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
