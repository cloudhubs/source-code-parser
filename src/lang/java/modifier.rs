use crate::parse::AST;
use crate::prophet::*;
use crate::Language::Java;
/// Handles parsing of modifiers in the Java AST. Modifiers contains the visibility,
/// final, and static nature of the element the modifier is attached to.

/// Struct to hold return data from parse_modifiers/find_modifiers
#[derive(Debug)]
pub(crate) struct Modifier {
    pub accessor: AccessorType,
    pub annotations: Vec<AnnotationComponent>,
    pub is_abstract: bool,
    pub is_final: bool,
    pub is_static: bool,
}
impl Modifier {
    pub(crate) fn new() -> Modifier {
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
pub(crate) fn find_modifier(ast: &AST, path: &str, package: &str) -> Modifier {
    match ast.find_child_by_type(&["modifiers"]) {
        Some(modifier) => parse_modifiers(modifier, path, package),
        None => Modifier::new(),
    }
}

/// Parse the modifiers field of the AST; this includes access level and annotations
pub(crate) fn parse_modifiers(ast: &AST, path: &str, package: &str) -> Modifier {
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
pub(crate) fn parse_annotations(
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

        // Generate and store annotation
        if let Some(params) = params {
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
                        language: Java,
                    });
                }

                // Create annotation
                annotations.push(AnnotationComponent::create_normal(
                    name,
                    key_value_pairs,
                    path,
                    package,
                    Java,
                ));
            } else {
                // Annotation with 1 unnamed parameter -> SingleAnnotation
                let val = params.children.get(1).expect("AST for a SingleAnnotation does not follow the pattern '(, value, )' in annotation_argument_list! AST malformed!");
                annotations.push(AnnotationComponent::create_single(
                    name,
                    &*parse_annotation_value(val),
                    path,
                    package,
                    Java,
                ));
            }
        } else {
            // Annotation with no parameters -> MarkerAnnotation
            annotations.push(AnnotationComponent::create_marker(
                name, path, package, Java,
            ))
        }
    }
    Some(())
}

/// Parse one of the values stored within an annotation
pub(crate) fn parse_annotation_value(ast: &AST) -> String {
    if !ast.children.is_empty() {
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
