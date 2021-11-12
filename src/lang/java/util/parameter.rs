use crate::java::method_body::log_unknown_tag;
use crate::java::modifier::parse_modifiers;
use crate::java::modifier::Modifier;
use crate::java::util::fold_vec;
use crate::parse::AST;
use crate::prophet::*;

use super::vartype::find_type;

/// Handles parsing parameters for lambdas, methods, etc.

/// Parse the AST of the parameters passed in to a method
pub(crate) fn parse_method_parameters(
    ast: &AST,
    component: &ComponentInfo,
) -> Vec<MethodParamComponent> {
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
    let mut modifier = Modifier::new();
    let param_type = find_type(ast);

    // Parse the definition
    for part_defn in ast.children.iter() {
        match &*part_defn.r#type {
            "variable_declarator" => match part_defn.find_child_by_type(&["identifier"]) {
                Some(ident) => name = ident.value.clone(),
                None => tracing::warn!("Variable declarator with no variable name!"),
            },
            "identifier" => name = part_defn.value.clone(),
            "modifiers" => {
                modifier = parse_modifiers(part_defn, &*component.path, &*component.package_name)
            }
            unknown => log_unknown_tag(unknown, "parameter list"),
        }
    }

    MethodParamComponent {
        component: ComponentInfo {
            path: component.path.clone(),
            package_name: component.package_name.clone(),
            instance_name: component.instance_name.clone(),
            instance_type: InstanceType::MethodParamComponent,
            language: Language::Java,
        },
        annotation: fold_vec(modifier.annotations),
        r#type: param_type,
        parameter_name: name,
    }
}
