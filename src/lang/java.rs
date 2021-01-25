use crate::prophet::*;
use crate::{ast::*, parse::AST};

pub fn merge_modules(modules: Vec<ModuleComponent>) -> Vec<ModuleComponent> {
    // TODO implement correctly
    modules
}

pub fn find_components(ast: AST, path: &str) -> Vec<ComponentType> {
    find_components_internal(ast, String::new(), path)
}

fn find_components_internal(ast: AST, mut package: String, path: &str) -> Vec<ComponentType> {
    let mut components = vec![];
    for node in ast
        .find_all_children_by_type(&[
            "import_declaration",
            "package_declaration",
             "class_declaration",
            "interface_declaration",
             "enum_declaration",
             "annotation_declaration",
        ])
        .expect("Provided an invalid Java file, no class, interface, annotation, enum, packages, or imports found!")
         .iter()
    {
        match &*node.r#type {
            "import_declaration" => println!("{}", parse_import(&node)),
            "package_declaration" => {
                package = parse_package(&node)
                    .expect(&*format!("Malformed package declaration {:#?}!", node));
            }
            "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "annotation_declaration" => match parse_class(node, &*package, path) {
                Some(class) => {
                    // Save the methods
                    for method in class.component.methods.iter() {
                        components.push(ComponentType::MethodComponent(method.clone()));
                    }

                    // Save the class itself
                    components.push(ComponentType::ClassOrInterfaceComponent(class));
                }
                None => {}
            },
            tag => todo!("Cannot identify provided tag {:#?}", tag),
        };
    }

    components
}

/// Take in the AST node containing the package declaration, and--if it is not malformed--return a string representing the package
fn parse_package(ast: &AST) -> Option<String> {
    let result = ast.find_child_by_type(&["scoped_identifier"])?;
    let mut buffer = String::new();
    for member in result.children.iter() {
        if member.r#type == "scoped_identifier" {
            buffer = format!("{}{}", parse_package(result)?, buffer);
        } else {
            buffer.push_str(&*member.value);
        }
    }
    Some(buffer)
}

/// Take the AST node containing an import statement, and return back the String describing the package imported
fn parse_import(ast: &AST) -> String {
    let mut buffer = String::new();
    for child in ast.children.iter() {
        match &*child.r#type {
            "identifier" | "." | "*" => buffer.push_str(&*child.value),
            _ => buffer.push_str(&*parse_import(child)),
        };
    }
    buffer
}

/// Parse a single class/interface/annotation/what have you's AST
fn parse_class(ast: &AST, package: &str, path: &str) -> Option<ClassOrInterfaceComponent> {
    // Get container info
    let instance_type = match ast.find_child_by_type(&["class", "interface", "enum", "annotation"])
    {
        Some(r#type) => match &*r#type.value {
            "class" | "enum" => InstanceType::ClassComponent,
            "interface" => InstanceType::InterfaceComponent,
            "annotation" => InstanceType::AnnotationComponent,
            _ => InstanceType::ClassComponent,
        },
        None => InstanceType::ClassComponent,
    };
    let instance_name = match ast.find_child_by_type(&["identifier"]) {
        Some(identifier) => identifier.value.clone(),
        None => "".into(),
    };
    let component = ComponentInfo {
        path: path.into(),
        package_name: package.into(),
        instance_name: instance_name.clone(),
        instance_type: instance_type,
    };

    // Define default values
    let mut stereotype = ContainerStereotype::Entity;
    let mut fields = vec![];
    let mut constructors = vec![];
    let mut methods = vec![];
    let mut modifier = Modifier::new();
    let mut start = 0;
    let mut end = 0;

    // Generate the implementation data
    for member in ast.children.iter() {
        match &*member.r#type {
            "modifiers" => {
                modifier = parse_modifiers(member, &*component.path, &*component.package_name)
            }
            "class_body" | "interface_body" | "enum_body" | "annotation_body" => {
                let (_start, _end) = parse_class_body(
                    member,
                    &component,
                    &mut constructors,
                    &mut methods,
                    &mut fields,
                );
                start = _start as i32;
                end = _end as i32;
            }
            unknown_type => {
                eprintln!(
                    "{} tag unhandled. This may or may not be an issue.",
                    unknown_type
                );
            }
        };
    }

    // Assemble the return value from what was extracted from the tree
    Some(ClassOrInterfaceComponent {
        component: ContainerComponent {
            component,
            accessor: modifier.accessor,
            stereotype: stereotype,
            methods,
            container_name: instance_name,
            line_count: end - start,
        },
        declaration_type: ContainerType::Class,
        annotations: modifier.annotations,
        constructors: fold_vec(constructors),
        field_components: fold_vec(fields),
    })
}

/// Parse the AST for a specified method
fn parse_method(ast: &AST, component: &ComponentInfo) -> MethodComponent {
    // Define fields
    let mut body = None;
    let mut modifier = Modifier::new();
    let mut method_name = String::new();
    let mut parameters = vec![];
    let return_type = find_type(ast);
    let line_begin = ast.span.expect("No span for a method! AST malformed!").0 as i32;
    let line_end = ast.span.expect("No span for a method! AST malformed!").2 as i32;

    // Parse method
    for member in ast.children.iter() {
        match &*member.r#type {
            "identifier" | "static" => method_name = member.value.clone(),
            "modifiers" => {
                modifier = parse_modifiers(member, &*component.path, &*component.package_name)
            }
            "formal_parameters" => parameters = parse_method_parameters(member, component),
            "constructor_body" | "block" => {
                body = Some(parse_block(member, component));
            }
            unknown => println!("{} unknown", unknown),
        }
    }

    // Return the method component
    MethodComponent {
        component: component.clone(),
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

/// Struct to hold return data from parse_modifiers/find_modifiers
struct Modifier {
    accessor: AccessorType,
    annotations: Vec<AnnotationComponent>,
    is_abstract: bool,
    is_final: bool,
    is_static: bool,
}
impl Modifier {
    fn new() -> Modifier {
        Modifier {
            accessor: AccessorType::Default,
            annotations: vec![],
            is_abstract: false,
            is_final: false,
            is_static: false,
        }
    }
}

/// Find the modifier child of a tree, and return it to the caller
fn find_modifier(ast: &AST, path: &str, package: &str) -> Modifier {
    match ast.find_child_by_type(&["modifiers"]) {
        Some(modifier) => parse_modifiers(modifier, path, package),
        None => Modifier::new(),
    }
}

/// Parse the modifiers field of the AST; this includes access level and annotations
fn parse_modifiers(ast: &AST, path: &str, package: &str) -> Modifier {
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

    // Parse final, static, abstract keywords
    let mut is_final = false;
    let mut is_static = false;
    let mut is_abstract = false;
    for attribute in ast.children.iter() {
        match &*attribute.r#type {
            "final" => is_final = true,
            "static" => is_static = true,
            "abstract" => is_abstract = true,
            _ => {}
        }
    }

    // Parse annotations
    let mut annotations = vec![];
    parse_annotations(ast, path, package, &mut annotations);

    Modifier {
        accessor: access_level,
        annotations,
        is_final,
        is_static,
        is_abstract,
    }
}

/// Recursively parse the annotations on a field
fn parse_annotations(
    ast: &AST,
    path: &str,
    package: &str,
    annotations: &mut Vec<AnnotationComponent>,
) -> Option<()> {
    for item in ast
        .find_all_children_by_type(&["annotation", "marker_annotation"])?
        .iter()
    {
        let name = &*format!(
            "@{}",
            &*item
                .find_child_by_type(&["identifier"])
                .expect("Annotation found with no name (identifier)! AST malformed!")
                .value
        );

        // Parse exact type of annotation
        let params = item.find_child_by_type(&["annotation_argument_list"]);
        println!("{}", name);
        if name == "@Data" {
            println!("{:#?}", params);
        }

        // Generate and store annotation
        if params.is_some() {
            let params = params.unwrap();
            if let Some(params) = params.find_all_children_by_type(&["element_value_pair"]) {
                // Annotation with named parameters -> NormalAnnotation
                let mut key_value_pairs = vec![];

                // Generate element values
                for child in params.iter() {
                    let key = &*child
                        .find_child_by_type(&["identifier"])
                        .expect(&*format!(
                            "Annotation with multiple elements contains unnamed element! {:#?}",
                            child
                        ))
                        .value;
                    let value = parse_annotation_value(
                        child
                            .children
                            .get(2)
                            .expect("No value provided for an annotation element! AST malformed!"),
                    );
                    key_value_pairs.push(AnnotationValuePair {
                        key: String::from(key),
                        value,
                    });
                }

                // Create annotation
                annotations.push(AnnotationComponent::create_normal(
                    name,
                    key_value_pairs,
                    path,
                    package,
                ));
            } else {
                // Annotation with 1 unnamed parameter -> SingleAnnotation
                let val = params.children.get(1).expect("AST for a SingleAnnotation does not follow the pattern '(, value, )' in annotation_argument_list! AST malformed!");
                annotations.push(AnnotationComponent::create_single(
                    name,
                    &*parse_annotation_value(val),
                    path,
                    package,
                ));
            }
        } else {
            // Annotation with no parameters -> MarkerAnnotation
            annotations.push(AnnotationComponent::create_marker(name, path, package))
        }
    }
    Some(())
}

/// Parse one of the values stored within an annotation
fn parse_annotation_value(ast: &AST) -> String {
    if ast.children.len() > 0 {
        // Filter the tokens to ensure the RHS parses correctly
        let tokens = ast.children.iter().map(|node| {
            if node.value == "new" {
                "new "
            } else {
                &*node.value
            }
        });
        let mut buffer = String::new();
        for term in tokens {
            buffer.push_str(term);
        }
        buffer
    } else {
        ast.value.clone()
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
        component: component.clone(),
        annotation: fold_vec(modifier.annotations),
        parameter_type: param_type.into(),
        parameter_name: name.into(),
    }
}

/// Provided an AST representing the type of a parameter/field, interprets it into a valid String representation
fn parse_type(ast: &AST) -> String {
    match &*ast.r#type {
        "type_identifier" => ast.value.clone(),
        "array_type" => {
            let mut result_type = String::new();
            for child in ast.children.iter() {
                result_type.push_str(&*parse_type(child));
            }
            result_type
        }
        "integral_type" | "floating_point_type" => ast
            .children
            .get(0)
            .expect("Cannot detect the type of a numeric primitive! The AST appears malformed!")
            .r#type
            .clone(),
        "boolean_type" => ast.value.clone(),
        "dimensions" => stringify_tree_children(ast),
        unknown => String::from(unknown),
    }
}
fn find_type(ast: &AST) -> String {
    find_type_or_none(ast).get_or_insert("void".into()).clone()
}
fn find_type_or_none(ast: &AST) -> Option<String> {
    let child = ast.find_child_by_type(&[
        "type_identifier",
        "array_type",
        "integral_type",
        "floating_point_type",
        "boolean_type",
        "dimensions",
    ])?;

    Some(parse_type(child))
}

/// Parse the body of a method, static block, constructor, etc.
fn parse_block(ast: &AST, component: &ComponentInfo) -> Block {
    Block::new(parse_child_nodes(ast, component))
}

fn parse_child_nodes(ast: &AST, component: &ComponentInfo) -> Vec<Node> {
    ast.children
        .iter()
        .map(|member| parse_node(member, component))
        .filter(|option| option.is_some())
        .flat_map(|some| some)
        .collect()
}

fn parse_node(ast: &AST, component: &ComponentInfo) -> Option<Node> {
    match &*ast.r#type {
        "local_variable_declaration" | "field_declaration" => {
            // Extract informtion about the variable
            let r#type = find_type_or_none(ast);
            let modifier = find_modifier(ast, &*component.path, &*component.package_name);

            // Determine the value it was set to
            let rhs = parse_child_nodes(ast, component)
                .into_iter()
                .map(|node| match node {
                    Node::Expr(expr) => Some(expr),
                    _ => None,
                })
                .filter(|expr| expr.is_some())
                .flat_map(|expr| expr)
                .collect();

            // TODO: Use name
            let mut decl = DeclStmt::new(r#type, rhs);
            decl.is_static = Some(modifier.is_static);
            decl.is_final = Some(modifier.is_final);
            let decl: Stmt = decl.into();
            Some(decl.into())
        }

        "variable_declarator" | "assignment_expression" => {
            // Define attributes
            let mut name = "";
            let mut rhs = None;

            // Find values
            for node in ast.children.iter() {
                match &*node.r#type {
                    "identifier" => name = &*node.value,
                    "=" => {}
                    unknown => {
                        if let Some(Node::Expr(parsed_rhs)) = parse_node(node, component) {
                            rhs = Some(parsed_rhs);
                        } else {
                            eprintln!(
                                "Encountered unknown tag {} while parsing variable assignment",
                                unknown
                            );
                            rhs = None;
                        }
                    }
                }
            }

            // Assemble
            if let Some(rhs) = rhs {
                let bin: Expr = BinaryExpr::new(
                    Box::new(Ident::new(name.into()).into()),
                    "=".into(),
                    Box::new(rhs),
                )
                .into();
                Some(bin.into())
            } else {
                let expr: Expr = Ident::new(name.into()).into();
                Some(expr.into())
            }
        }

        "identifier" => {
            let ident: Expr = Ident::new(ast.value.clone()).into();
            Some(ident.into())
        }

        "decimal_integer_literal"
        | "decimal_floating_point_literal"
        | "string_literal"
        | "false"
        | "true" => Some(Node::Expr(Expr::Literal(ast.value.clone()))),

        "object_creation_expression" => {
            let mut name = String::new();
            let mut arg_list = vec![];
            for child in ast.children.iter() {
                match &*child.r#type {
                    "type_identifier" => name = child.value.clone(),
                    "argument_list" => {
                        arg_list = parse_child_nodes(child, component)
                            .into_iter()
                            .map(|node| match node {
                                Node::Expr(expr) => Some(expr),
                                _ => None,
                            })
                            .flat_map(|expr| expr)
                            .collect()
                    }
                    _ => {}
                }
            }

            // Create ident
            let ident: Expr = CallExpr::new(Box::new(Ident::new(name).into()), arg_list).into();
            Some(ident.into())
        }

        "array_creation_expression" => {
            let ident: Expr = Ident::new("AN_ARRAY_HANDLE".into()).into();
            Some(ident.into())
        }

        unknown => {
            eprintln!("{} unknown tag in parsing method body!", unknown);
            None
        }
    }
}

fn parse_field(ast: &AST, component: &ComponentInfo) -> FieldComponent {
    let variables = ast
        .find_all_children_by_type(&["variable_declarator"])
        .get_or_insert(vec![])
        .iter()
        .map(|var_decl| var_decl.find_child_by_type(&["identifier"]))
        .filter(|var_ident| var_ident.is_some())
        .map(|identifier| identifier.unwrap().value.clone())
        .collect();
    let modifier = find_modifier(ast, &*component.path, &*component.package_name);
    let r#type = find_type(ast);

    // TODO: How to handle field_name, default_value?
    FieldComponent {
        component: component.clone(),
        annotations: modifier.annotations,
        variables,
        field_name: String::new(),
        accessor: modifier.accessor,
        is_static: modifier.is_static,
        is_final: modifier.is_final,
        default_value: String::new(),
        r#type,
    }
}

fn parse_class_body(
    ast: &AST,
    component: &ComponentInfo,
    constructors: &mut Vec<MethodComponent>,
    methods: &mut Vec<MethodComponent>,
    fields: &mut Vec<FieldComponent>,
) -> (usize, usize) {
    let mut start = 0;
    let mut end = 0;

    // Traverse body
    for member in ast.children.iter() {
        match &*member.r#type {
            "constructor_declaration" | "static_initializer" => {
                constructors.push(parse_method(member, component))
            }
            "method_declaration" => methods.push(parse_method(member, component)),
            "{" => match member.span {
                Some((line, _, _, _)) => start = line,
                None => eprintln!("No span for open parenthesis! Cannot detect line number."),
            },
            "}" => match member.span {
                Some((line, _, _, _)) => end = line,
                None => eprintln!("No span for close parenthesis! Cannot detect line number."),
            },
            "field_declaration" => fields.push(parse_field(member, component)),
            "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "annotation_declaration" => { /* None, since these were extracted + handled elsewhere */
            }
            unknown => eprintln!(
                "Attempting to parse {}, not currently supported. TODO implement fully!",
                unknown
            ),
        }
    }

    (start, end)
}

/// Convert a vector into an Option. If the vector is empty, swaps it out for None; otherwise is Some(vector)
fn fold_vec<T>(vector: Vec<T>) -> Option<Vec<T>> {
    if !vector.is_empty() {
        Some(vector)
    } else {
        None
    }
}

/// Convert the children of a provided tree into a single, consecutive string
fn stringify_tree_children(ast: &AST) -> String {
    let mut buffer = String::new();
    for member in ast.children.iter() {
        buffer.push_str(&*member.value);
    }
    buffer
}
