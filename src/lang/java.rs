use crate::parse::AST;
use crate::prophet::*;

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];

    for node in ast
        .find_all_children_by_type(&[
            "package_declaration",
            "class_declaration",
            "interface_declaration",
            "enum_declaration",
            "annotation_declaration",
        ])
        .unwrap()
        .iter()
    {
        match &*node.r#type {
            "package_declaration" => {
                package = parse_package(&node)
                    .expect(&*format!("Malformed package declaration {:#?}!", node));
            }
            "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "annotation_declaration" => match parse_class(node, &*package, path) {
                Some(class) => components.push(ComponentType::ClassOrInterfaceComponent(class)),
                None => {}
            },
            tag => {
                todo!("Cannot identify provided tag {:#?}", tag);
            }
        };
    }

    components
}

/// Take in the AST node containing the package declaration, and--if it is not malformed--return a string representing the package
fn parse_package(ast: &AST) -> Option<String> {
    let result = ast.find_child_by_type(&["scoped_identifier"])?;
    let mut package = String::new();
    for member in result.children.iter() {
        package.push_str(&*member.value);
    }
    Some(package)
}

/// Parse a single class/interface/annotation/what have you's AST
fn parse_class(ast: &AST, package: &str, path: &str) -> Option<ClassOrInterfaceComponent> {
    // Define default values
    let mut instance_name = String::new();
    let mut accessor = AccessorType::Default;
    let mut stereotype = ContainerStereotype::Entity;
    let mut instance_type = InstanceType::ClassComponent;
    let mut fields = vec![];
    let mut constructors = vec![];
    let mut methods = vec![];
    let mut annotations = vec![];

    // Generate the implementation data
    for member in ast.children.iter() {
        match &*member.r#type {
            "modifiers" => {
                let (_accessor, _annotations) = parse_modifiers(member);
                accessor = _accessor;
                annotations = _annotations;
            }
            "identifier" => instance_name = member.value.clone(),
            "class_body" | "enum_body" => {
                instance_type = InstanceType::ClassComponent;
                parse_body(
                    member,
                    package,
                    path,
                    &mut constructors,
                    &mut methods,
                    &mut fields,
                );
            }
            "interface_body" => {
                instance_type = InstanceType::InterfaceComponent;
                parse_body(
                    member,
                    package,
                    path,
                    &mut constructors,
                    &mut methods,
                    &mut fields,
                );
            }
            "annotation_body" => {
                instance_type = InstanceType::AnnotationComponent;
                parse_body(
                    member,
                    package,
                    path,
                    &mut constructors,
                    &mut methods,
                    &mut fields,
                );
            }

            unknown_type => {
                println!("{} unhandled", unknown_type);
            }
        };
    }

    // Assemble the return value from what was extracted from the tree
    Some(ClassOrInterfaceComponent {
        component: ContainerComponent {
            component: ComponentInfo {
                path: path.into(),
                package_name: package.into(),
                instance_name: instance_name.clone(),
                instance_type,
            },
            accessor,
            stereotype: stereotype,
            methods,
            container_name: instance_name,
            line_count: 0,
        },
        declaration_type: ContainerType::Class,
        annotations,
        stereotype: ContainerStereotype::Entity,
        constructors: fold_vec(constructors),
        field_components: fold_vec(fields),
    })
}

/// Parse the modifiers field of the AST; this includes access level and annotations
fn parse_modifiers(ast: &AST) -> (AccessorType, Vec<AnnotationComponent>) {
    // Parse access level
    let access_level = match ast.find_child_by_type(&["public", "protected", "private"]) {
        Some(ast) => match &*ast.r#type {
            "public" => AccessorType::Public,
            "protected" => AccessorType::Protected,
            "private" => AccessorType::Private,
            _ => AccessorType::Default,
        },
        None => AccessorType::Default,
    };

    // Parse annotations
    let mut annotations = vec![];
    parse_annotations(ast, &mut annotations);

    (access_level, annotations)
}

/// Recursively parse the annotations on a field
fn parse_annotations(ast: &AST, annotations: &mut Vec<AnnotationComponent>) {
    for item in ast.children.iter() {
        match &*item.r#type {
            "identifier" => annotations.push(AnnotationComponent {
                component: ComponentInfo {
                    path: String::new(),
                    package_name: String::new(),
                    instance_name: String::new(),
                    instance_type: InstanceType::AnnotationComponent,
                },
                name: format!("@{}", item.r#value),
                annotation_meta_model: String::new(),
                meta_model_field_name: String::new(),
                key_value_pairs: vec![],
                value: String::new(),
            }),
            _ => parse_annotations(item, annotations),
        }
    }
}

/// Parse the AST for a specified method
fn parse_method(ast: &AST, package: &str, path: &str) -> MethodComponent {
    eprintln!("Attempting to parse a method, not currently supported. TODO implement fully!");
    MethodComponent {
        component: ComponentInfo {
            path: path.clone().into(),
            package_name: package.clone().into(),
            instance_name: String::from("foo"),
            instance_type: InstanceType::MethodComponent,
        },
        accessor: AccessorType::Private,
        method_name: String::from("foo"),
        return_type: String::from("goo"),
        parameters: vec![],
        is_static: false,
        is_abstract: false,
        sub_methods: vec![],
        annotations: vec![],
        line_count: 0,
        line_begin: 0,
        line_end: 0,
    }
}

// parse_field(ast: &AST, package: &str, path: &str) -> FieldComponent {
//     todo!("No field parsing yet");
// }

fn parse_body(
    ast: &AST,
    package: &str,
    path: &str,
    constructors: &mut Vec<MethodComponent>,
    methods: &mut Vec<MethodComponent>,
    fields: &mut Vec<FieldComponent>,
) {
    // Traverse body
    for member in ast.children.iter() {
        match &*member.r#type {
            "constructor_declaration" => constructors.push(parse_method(member, package, path)),
            "method_declaration" => methods.push(parse_method(member, package, path)),
            // "field_declaration" => {
            //     //fields.push(parse_field(member, package, path)),
            //     println!("Can't parse {} yet", member.r#value);
            // }
            unknown => {
                eprintln!(
                    "Attempting to parse {}, not currently supported. TODO implement fully!",
                    unknown
                );
            }
        }
    }
}

/// Convert a vector into an Option. If the vector is empty, swaps it out for None; otherwise is Some(vector)
fn fold_vec<T>(vector: Vec<T>) -> Option<Vec<T>> {
    if !vector.is_empty() {
        Some(vector)
    } else {
        None
    }
}
