use crate::ast::Expr;
use crate::java::method_body::{log_unknown_tag, parse_assignment_pub};
use crate::java::method_def::parse_method;
use crate::java::modifier::{find_modifier, parse_modifiers, Modifier};
use crate::java::util::vartype::find_type;
use crate::parse::AST;
use crate::prophet::*;
use crate::Language::Java;

/// Handles class definition portions of the Java language, like fields,
/// orchestrating the class body, etc.

/// Parse a single class/interface/annotation/what have you's AST
pub(crate) fn parse_class(
    ast: &AST,
    package: &str,
    path: &str,
) -> Option<ClassOrInterfaceComponent> {
    // Get container info
    let instance_type = match ast.find_child_by_type(&["class", "interface", "enum", "annotation"])
    {
        Some(r#type) => match &*r#type.value {
            "interface" => InstanceType::InterfaceComponent,
            _ => InstanceType::ClassComponent,
        },
        None => InstanceType::ClassComponent,
    };
    let instance_name = match ast.find_child_by_type(&["identifier"]) {
        Some(identifier) => identifier.value.clone(),
        None => "".into(),
    };

    let declaration_type = match instance_type {
        InstanceType::InterfaceComponent => ContainerType::Interface,
        _ => ContainerType::Class,
    };
    let component = ComponentInfo {
        path: path.into(),
        package_name: package.into(),
        instance_name: format!(
            "{}::{}",
            instance_name,
            match instance_type {
                InstanceType::InterfaceComponent => "InterfaceComponent",
                _ => "ClassComponent",
            }
        ),
        instance_type,
        language: Java,
    };

    // Define default values
    let stereotype = ContainerStereotype::Entity; // TODO determine properly
    let mut fields = vec![];
    let mut constructors = vec![];
    let mut methods = vec![];
    let mut modifier = Modifier::new();

    // Find bounds
    let (start, end) = match ast.span {
        Some(span) => (span.0 as i32, span.1 as i32),
        None => (0, 0),
    };

    // Generate the implementation data
    for member in ast.children.iter() {
        match &*member.r#type {
            "modifiers" => {
                modifier = parse_modifiers(member, &*component.path, &*component.package_name)
            }
            "class_body" | "interface_body" | "enum_body" | "annotation_body" => {
                parse_class_body(
                    member,
                    &component,
                    &mut constructors,
                    &mut methods,
                    &mut fields,
                );
            }
            unknown_type => log_unknown_tag(unknown_type, "class"),
        };
    }

    // Assemble the return value from what was extracted from the tree
    Some(ClassOrInterfaceComponent {
        component: ContainerComponent {
            component,
            accessor: modifier.accessor,
            stereotype,
            methods,
            container_name: instance_name,
            line_count: end - start + 1,
        },
        declaration_type,
        annotations: modifier.annotations,
        constructors,
        field_components: fields,
    })
}

/// Parses the members of a class to find members, methods, blocks, etc.
fn parse_class_body(
    ast: &AST,
    component: &ComponentInfo,
    constructors: &mut Vec<MethodComponent>,
    methods: &mut Vec<MethodComponent>,
    fields: &mut Vec<FieldComponent>,
) {
    // Traverse body
    for member in ast.children.iter() {
        match &*member.r#type {
            "constructor_declaration" | "static_initializer" => {
                constructors.push(parse_method(member, component))
            }
            "method_declaration" => methods.push(parse_method(member, component)),
            "field_declaration" => fields.append(&mut parse_field(member, component)),
            "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "annotation_declaration" => { /* None, since these were extracted + handled elsewhere */
            }
            unknown => log_unknown_tag(unknown, "class body"),
        }
    }
}

/// Parses a single field in a class
fn parse_field(ast: &AST, component: &ComponentInfo) -> Vec<FieldComponent> {
    // let variables: Vec<String> = ast
    //     .find_all_children_by_type(&["variable_declarator"])
    //     .get_or_insert(vec![])
    //     .iter()
    //     .flat_map(|var_decl| var_decl.find_child_by_type(&["identifier"]))
    //     .map(|identifier| identifier.value.clone())
    //     .collect();
    let modifier = find_modifier(ast, &*component.path, &*component.package_name);
    let r#type = find_type(ast);
    let component = ComponentInfo {
        path: component.path.clone(),
        package_name: component.package_name.clone(),
        instance_name: component.instance_name.clone(),
        instance_type: InstanceType::FieldComponent,
        language: Java,
    };
    let fields: Vec<FieldComponent> = ast
        .find_all_children_by_type(&["variable_declarator"])
        .get_or_insert(vec![])
        .iter()
        .filter(|var_decl| var_decl.find_child_by_type(&["identifier"]).is_some())
        .map(|var_decl| {
            let field_name = var_decl
                .find_child_by_type(&["identifier"])
                .unwrap()
                .value
                .clone();
            let expr: Option<Expr> = parse_assignment_pub(var_decl, &component);
            FieldComponent {
                component: ComponentInfo {
                    path: component.path.clone(),
                    package_name: component.package_name.clone(),
                    instance_name: field_name.clone(),
                    instance_type: InstanceType::FieldComponent,
                    language: Java,
                },
                annotations: modifier.annotations.clone(),
                variables: vec![],
                field_name,
                accessor: modifier.accessor.clone(),
                is_static: modifier.is_static,
                is_final: modifier.is_final,
                default_value: String::new(),
                r#type: r#type.clone(),
                expression: expr,
            }
        })
        .collect();
    fields

    // TODO: How to handle field_name, default_value?
    // variables
    //     .into_iter()
    //     .map(|field_name| FieldComponent {
    //         component: ComponentInfo {
    //             path: component.path.clone(),
    //             package_name: component.package_name.clone(),
    //             instance_name: field_name.clone(),
    //             instance_type: InstanceType::FieldComponent,
    //         },
    //         annotations: modifier.annotations.clone(),
    //         variables: vec![],
    //         field_name,
    //         accessor: modifier.accessor.clone(),
    //         is_static: modifier.is_static.clone(),
    //         is_final: modifier.is_final.clone(),
    //         default_value: String::new(),
    //         r#type: r#type.clone(),
    //     })
    //     .collect()
}

// fn parse_expr(ast: &AST, component: &ComponentInfo) -> Option<Expr> {
//     match &*ast.r#type {
//         // Variables an initialization
//         "variable_declarator" | "assignment_expression" => parse_assignment(ast, component),
//         // Base case
//         unknown => {
//             log_unknown_tag(unknown, "expressions");
//             None
//         }
//     }
// }
