use crate::parse::AST;
use crate::prophet::*;
use crate::js::modifier::{parse_modifiers, Modifier};

// Takes an AST starting with a class_declaration node and parses it as a class component
pub(crate) fn parse_class(
    ast: &AST,
    package: &str,
    path: &str,
) -> Option<ClassOrInterfaceComponent> {
    // **
    // handle metadata stuff
    // **

    // may need to handle other instance types, but IDK
    let instance_type = InstanceType::ClassComponent;
    // class name is first "identifier" node we find in children
    let instance_name = match ast.find_child_by_type(&["identifier"]) {
        Some(identifier) => identifier.value.clone(),
        None => "".into(),
    };
    // again, may need other types
    let declaration_type = ContainerType::Class;
    let component = ComponentInfo {
        path: path.into(),
        package_name: package.into(),
        instance_name: format!(
            "{}::{}",
            instance_name,
            "ClassComponent" // blah blah other types blah
        ),
        instance_type: instance_type,
    };

    // **
    // filling out the class contents
    // **

    let stereotype = ContainerStereotype::Entity; // yeah no idea what this is
    let mut fields = vec![];
    let mut constructors = vec![];
    let mut methods = vec![];
    let mut modifier = Modifier::new();

    // start/end lines in code
    let (start, end) = match ast.span {
        Some(span) => (span.0 as i32, span.1 as i32),
        None => (0, 0),
    };
}