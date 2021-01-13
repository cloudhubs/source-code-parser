use crate::parse::AST;
use crate::prophet::*;

pub fn flatten_classes(ast: &mut AST) {}

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];

    for node in ast.children.iter() {
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
    let mut declaration_type = ContainerType::Class;
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
            "constructor_declaration" => constructors.push(parse_method(member, package, path)),
            "method_declaration" => methods.push(parse_method(member, package, path)),
            "class_body" | "interface_body" | "enum_body" | "annotation_body" => {
                parse_body(member, package, path);
            }

            _ => {}
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
        declaration_type,
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

fn parse_body(ast: &AST, package: &str, path: &str) {
    //
}

/// Convert a vector into an Option. If the vector is empty, swaps it out for None; otherwise is Some(vector)
fn fold_vec<T>(vector: Vec<T>) -> Option<Vec<T>> {
    if !vector.is_empty() {
        Some(vector)
    } else {
        None
    }
}
