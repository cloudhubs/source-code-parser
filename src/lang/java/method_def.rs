use crate::java::body::parse_block;
use crate::java::modifier::parse_modifiers;
use crate::java::modifier::Modifier;
use crate::java::util::fold_vec;
use crate::java::vartype::find_type;
use crate::java::vartype::parse_type;
use crate::parse::AST;
use crate::prophet::*;

/// Parse the signature of a method. Parsing of the body is left to another file,
/// since that's a large task in and of itself.

/// Parse the AST for a specified method
pub(crate) fn parse_method(ast: &AST, component: &ComponentInfo) -> MethodComponent {
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
    let mut method_name = String::new();
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
            "formal_parameters" => parameters = parse_method_parameters(member, &component),
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

/// Parse the AST of the parameters passed in to a method
fn parse_method_parameters(ast: &AST, component: &ComponentInfo) -> Vec<MethodParamComponent> {
    let mut params = vec![];

    for parameter in ast.children.iter() {
        parse_method_parameters(parameter, component);
        match &*parameter.r#type {
            "formal_parameter" | "spread_parameter" => {
                params.push(parse_parameter(parameter, component))
            }
            _ => {}
        }
    }

    params
}

/// Parse the AST containing a single parameter to a method
fn parse_parameter(ast: &AST, component: &ComponentInfo) -> MethodParamComponent {
    let mut name = String::new();
    let mut param_type = String::new();
    let mut modifier = Modifier::new();
    let mut is_spread = false;

    // Parse the definition
    for part_defn in ast.children.iter() {
        match &*part_defn.r#type {
            "identifier" => name = part_defn.value.clone(),
            "modifiers" => {
                modifier = parse_modifiers(part_defn, &*component.path, &*component.package_name)
            }
            "..." => is_spread = true,
            _ => param_type = parse_type(part_defn),
        }
    }

    // If this is varargs, add varargs symbol
    if is_spread {
        name.push_str("...");
    }

    MethodParamComponent {
        component: ComponentInfo {
            path: component.path.clone(),
            package_name: component.package_name.clone(),
            instance_name: component.instance_name.clone(),
            instance_type: InstanceType::ParameterComponent,
        },
        annotation: fold_vec(modifier.annotations),
        r#type: param_type.into(),
        parameter_name: name.into(),
    }
}
