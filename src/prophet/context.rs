use super::*;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct JSSAContext<'a> {
    #[serde(flatten)]
    pub component: ComponentInfo,
    // module_package_map: ModulePackageMap<'a>,
    pub succeeded: bool,
    pub root_path: &'a str,

    // The following two fields could be done with a manual
    // serde::Serialization implementation rather than deriving it.
    // class_names: Vec<&'a str>,
    // interface_names: Vec<&'a str>,

    // Classes and interfaces would be contained in this. However, I couldn't
    // find anywhere in the prophet repos where this was actually used.
    // containers: Vec<ComponentType<'a>>,

    // classes: Vec<ClassComponent<'a>>,
    // interfaces: Vec<InterfaceComponent<'a>>,
    pub modules: Vec<ModuleComponent>,
    // methods: Vec<MethodComponent<'a>>,
}
