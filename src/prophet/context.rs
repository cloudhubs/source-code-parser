use serde::Serialize;
use super::*;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct JSSAContext<'a> {
    #[serde(flatten)]
    component: ComponentInfo<'a>,
    // module_package_map: ModulePackageMap<'a>,
    succeeded: bool,
    root_path: &'a str,

    // The following two fields could be done with a manual 
    // serde::Serialization implementation rather than deriving it.
    // class_names: Vec<&'a str>,
    // interface_names: Vec<&'a str>,
    
    // Classes and interfaces would be contained in this. However, I couldn't
    // find anywhere in the prophet repos where this was actually used.
    // containers: Vec<ComponentType<'a>>,

    // classes: Vec<ClassComponent<'a>>,
    // interfaces: Vec<InterfaceComponent<'a>>,
    modules: Vec<ModuleComponent<'a>>,
    // methods: Vec<MethodComponent<'a>>,
}