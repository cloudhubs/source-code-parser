use crate::parse::AST;
use crate::prophet::*;

pub(crate) fn parse_types(
    ast: &AST,
    package: &str,
    path: &str
) -> Vec<ComponentType>{
    let mut vec = vec![];

    for node in ast.find_all_children_by_type(&["type_spec"]).get_or_insert(vec![]).iter() {
        match &*node.r#type {
            "type_spec" => match parse_struct(node, &package, path) {
                Some(struct_type) => vec.push(ComponentType::ClassOrInterfaceComponent(struct_type)),
                None => {}
            },
            tag => todo!("Cannot identify provided tag {:#?}", tag),
        };
    }

    vec
}

pub(crate) fn parse_struct(
    ast: &AST,
    package: &str,
    path: &str
) -> Option<ClassOrInterfaceComponent> {

    let instance_type = match ast.find_child_by_type(&["struct"]) {
        Some(r#type) => match &*r#type.value {
            "interface" => InstanceType::InterfaceComponent,
            _ => InstanceType::ClassComponent,
        },
        None => InstanceType::ClassComponent,
    };

    let instance_name = match ast.find_child_by_type(&["type_identifier"]) {
        Some(identifier) => identifier.value.clone(),
        None => "Fail".into(),
    };

    //let instance_name = ast.find_child_by_type(&["type_identifier"]).map_or_else(|| "fail".into(), |t| t.value.clone());
    //println!("{}", instance_name.to_string());


    let component = ComponentInfo {
        path: path.into(),
        package_name: package.into(),
        instance_name: format!("{}", instance_name),
        instance_type: instance_type,
    };

    Some(ClassOrInterfaceComponent {
        component: ContainerComponent {
            component,
            accessor: AccessorType::Private,
            stereotype: ContainerStereotype::Fabricated,
            methods: vec![],
            container_name: instance_name,
            line_count: 0
        },
        declaration_type: ContainerType::Class,
        annotations: vec![],
        constructors: vec![],
        field_components: vec![]
    })


}