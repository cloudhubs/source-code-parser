use crate::go::util::identifier::parse_identifier;
use crate::go::util::vartype::find_type;
use crate::parse::AST;
use crate::prophet::*;

pub(crate) fn parse_type(
    ast: &AST,
    package: &str,
    path: &str,
) -> Option<ClassOrInterfaceComponent> {
    let node = match ast.find_child_by_type(&["type_spec"]) {
        Some(type_node) => type_node,
        None => ast,
    };

    parse_type_internal(node, &package, path)
}

pub(crate) fn parse_type_internal(
    ast: &AST,
    package: &str,
    path: &str,
) -> Option<ClassOrInterfaceComponent> {
    
    //determine the type of the instance
    let instance_type = match ast.find_child_by_type(&["struct_type", "interface_type"]) {
        Some(node) => match &*node.r#type {
            "interface_type" => InstanceType::InterfaceComponent,
            _ => InstanceType::ClassComponent,
        },
        None => InstanceType::ClassComponent,
    };
    
    //find the name of the type
    let instance_name = match ast.find_child_by_type(&["type_identifier"]) {
        Some(identifier) => identifier.value.clone(),
        None => "".into(),
    };

    //determine what type the instance is
    let declaration_type = match instance_type {
        InstanceType::InterfaceComponent => ContainerType::Interface,
        _ => ContainerType::Class,
    };

    //get the component information using the path, package, instance name, and instance type
    let component = ComponentInfo {
        language: Language::Go,
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
        instance_type: instance_type,
    };

    // Find bounds
    let (start, end) = match ast.span {
        Some(span) => (span.0 as i32, span.2 as i32),
        None => (0, 0),
    };

    // Define default values
    let stereotype = ContainerStereotype::Entity; // TODO determine properly
    let mut fields = vec![];
    let constructors = vec![];
    let mut methods = vec![];
    //let mut modifier = Modifier::new();

    for child in ast.children.iter() {
        match &*child.r#type {
            "struct_type" => {
                parse_struct_body(child, &component, &mut methods, &mut fields);
            }
            _ => {}
        }
    }

    Some(ClassOrInterfaceComponent {
        component: ContainerComponent {
            component: component,
            accessor: AccessorType::Public,
            stereotype: stereotype,
            methods: methods,
            container_name: instance_name,
            line_count: (end - start) + 1,
        },
        declaration_type: declaration_type,
        annotations: vec![],
        constructors: constructors,
        field_components: fields,
    })
}

fn parse_struct_body(
    ast: &AST,
    component: &ComponentInfo,
    _methods: &mut Vec<MethodComponent>,
    fields: &mut Vec<FieldComponent>,
) {
    for node in ast.children.iter() {
        match &*node.r#type {
            "field_declaration_list" => fields.append(&mut parse_fields(node, component)),
            _ => {}
        }
    }
}

fn parse_fields(ast: &AST, component: &ComponentInfo) -> Vec<FieldComponent> {
    let mut fields = vec![];
    for node in ast.children.iter() {
        match &*node.r#type {
            "field_declaration" => {
                let field_identifier = parse_identifier(node);
                let type_identifier = find_type(node);

                fields.push(FieldComponent {
                    component: ComponentInfo {
                        language: Language::Go,
                        path: component.path.clone(),
                        package_name: component.package_name.clone(),
                        instance_name: field_identifier.clone(),
                        instance_type: InstanceType::FieldComponent,
                    },
                    annotations: vec![],
                    variables: vec![],
                    field_name: field_identifier,
                    accessor: AccessorType::Public,
                    is_static: false,
                    is_final: false,
                    default_value: String::new(),
                    r#type: type_identifier,
                    expression: None,
                })
            }
            _ => {}
        }
    }
    fields
}
