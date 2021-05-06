// use super::MsdDispatch;
use crate::ast::*;
use crate::prophet::*;
use derive_new::new;
use regex::Regex;
use serde::Deserialize;

/// A Node pattern describing a node of interest to the parser.
#[derive(Debug, Clone, Deserialize, new)]
pub struct NodePattern {
    /// The target AST node type
    pub identifier: NodeType,

    /// A regex-like pattern identifying the specific call.
    /// It's a modified Regex, but more importantly supports variables
    /// with a #varname syntax (with ## being a literal #).
    /// Variables indicate the information we are looking for, like URLs and entity names.
    pub pattern: String,

    /// Sub-patterns for this node pattern to be matched in the AST.
    /// Some subpatterns may be specified as required.
    pub subpatterns: Vec<NodePattern>,

    /// A Rune script implementing the callback function interface
    pub callback: String,

    /// Indicates whether this pattern is essential for any higher order
    /// pattern to be matched successfully.
    pub essential: bool,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum NodeType {
    // Prophet nodes
    ClassOrInterface,
    Method,
    MethodParam,
    Field,
    Annotation,

    // Body nodes
    CallExpr,
    VarDecl,
}

pub trait IntoMsdNode {
    fn into_msd_node(&self) -> NodeType;
}

#[macro_export]
macro_rules! into_msd_node {
    ( $( $struct_name:ty: $name:ident ),+ ) => {
        $(
            impl IntoMsdNode for $struct_name {
                fn into_msd_node(&self) -> NodeType {
                    NodeType::$name
                }
            }
        )*
    };
}

into_msd_node!(
    // Prophet types
    ClassOrInterfaceComponent: ClassOrInterface,
    MethodComponent: Method,
    MethodParamComponent: MethodParam,
    FieldComponent: Field,
    AnnotationComponent: Annotation,
    // Body node types
    CallExpr: CallExpr,
    VarDecl: VarDecl
);

#[derive(Debug, Clone, new)]
pub struct CompiledPattern {
    pub pattern: Regex,
    pub variables: Vec<String>,
}

impl CompiledPattern {
    /// Create a compiled pattern from an MSD pattern and the context.
    pub fn from_pattern(pattern: &str, ctx: ()) -> Result<CompiledPattern, regex::Error> {
        let tag_regex = Regex::new(r#"#(&)?\{([a-zA-Z_-]+)\}"#)?;

        let mut variables = vec![];

        let matches = tag_regex.captures_iter(pattern);
        let mut pattern: String = pattern.into();
        for captures in matches.into_iter() {
            let is_ref = captures.get(1).is_some();
            let name = captures.get(2).expect("Variable must have a name").as_str();
            let s = captures
                .get(0)
                .expect("Entire pattern should have matched")
                .as_str();

            // Replace the capture with the processed capture group
            let capture_name = if is_ref {
                // TODO: replace with reference name from ctx if is_ref name
                name.into()
            } else {
                format!("(?P<{}>.*?)", name)
            };
            pattern = pattern.as_str().replace(s, &capture_name);

            // Insert non-reference variables
            if !is_ref {
                variables.push(name.into());
            }
        }
        let transformed_pattern = Regex::new(pattern.as_str())?;
        Ok(CompiledPattern::new(transformed_pattern, variables))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bson() {
        assert_eq!(
            "BSON_APPEND_(?P<type>.*?)_another_one",
            CompiledPattern::from_pattern("BSON_APPEND_#{type}_#&{another_one}", ())
                .expect("Failed to construct regex from pattern input")
                .pattern
                .as_str()
        );
    }
}
