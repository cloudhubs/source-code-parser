use crate::py::method_body::parse_block;
use crate::py::modifier::parse_modifiers;
use crate::py::modifier::Modifier;
use crate::py::util::parameter::parse_method_parameters;
use crate::py::util::vartype::find_type;
use crate::parse::AST;
use crate::prophet::*;

/// Parse the signature of a method. Parsing of the body is left to another file,
/// since that's a large task in and of itself.

/// Parse the AST for a specified method
pub(crate) fn parse_function(ast: &AST, component: &ComponentInfo) -> MethodComponent {
    // Define new component info
    let component = ComponentInfo {
        path: component.path.clone(),
        package_name: component.package_name.clone(),
        instance_name: component.instance_name.clone(),
        instance_type: InstanceType::MethodComponent,
    };

    // Define fields
    let mut body = None;
    let mut modifier = Modifier::new();
    let mut function_name = String::new();
    let mut parameters = vec![];
    let return_type = find_type(ast);

    // Extract position
    let span = ast.span.expect("No span for a method! AST malformed!");
    let line_begin = span.0 as i32;
    let line_end = span.2 as i32;

    // Parse method
    for member in ast.children.iter() {
        match &*member.r#type {
            "identifier" | "static" => method_name = member.value.clone(),
            "modifiers" => {
                modifier = parse_modifiers(member, &*component.path, &*component.package_name)
            }
            "parameters" => parameters = parse_method_parameters(member, &component),
            "constructor_body" | "block" => {
                body = Some(parse_block(member, &component));
            }
            _ => {} // unknown => println!("{} unknown", unknown),
        }
    }

    // Define new component info
    let component = ComponentInfo {
        path: component.path.clone(),
        package_name: component.package_name.clone(),
        instance_name: method_name.clone(),
        instance_type: InstanceType::MethodComponent,
    };

    // Return the method component
    MethodComponent {
        component,
        accessor: modifier.accessor,
        method_name,
        return_type,
        parameters,
        is_static: modifier.is_static,
        is_abstract: modifier.is_abstract,
        is_final: modifier.is_final,
        sub_methods: vec![],
        annotations: modifier.annotations,
        line_count: line_end - line_begin,
        line_begin,
        line_end,
        body,
    }
}
