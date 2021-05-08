// use super::MsdDispatch;
use super::interpreter::NodePatternParser;
use crate::ast::*;
use crate::prophet::*;
use derive_new::new;
use regex::Regex;
use serde::Deserialize;

use super::{ContextLocalVariableActions, ContextObjectActions, ParserContext};

/// A Node pattern describing a node of interest to the parser.
#[derive(Debug, Deserialize, new)]
pub struct NodePattern<'a> {
    /// The target AST node type
    pub identifier: NodeType,

    /// A regex-like pattern identifying the specific call.
    /// It's a modified Regex, but more importantly supports variables
    /// with a #varname syntax (with ## being a literal #).
    /// Variables indicate the information we are looking for, like URLs and entity names.
    #[serde(skip)]
    pub compiled_pattern: Option<CompiledPattern<'a>>,

    /// Sub-patterns for this node pattern to be matched in the AST.
    /// Some subpatterns may be specified as required.
    pub subpatterns: Vec<NodePattern<'a>>,

    /// A Rune script implementing the callback function interface
    pub callback: Option<String>,

    /// Indicates whether this pattern is essential for any higher order
    /// pattern to be matched successfully.
    pub essential: bool,

    /// Raw pattern defined by the user
    pub pattern: &'a str,
}

impl<'a> NodePattern<'a> {
    pub fn matches(&self, node: &impl IntoMsdNode) -> bool {
        self.identifier == node.into_msd_node()
    }
}

/// Parse an individual node with this NodePattern, lazily-initializing its CompiledPattern as needed
#[macro_export]
macro_rules! msd_node_parse {
    ( $pattern:ident, $node:ident, $ctx:ident ) => {
        use crate::msd::callback::Executor;
        use crate::msd::context::ContextLocalVariableActions;
        use crate::msd::interpreter::NodePatternParser;

        // Lazily compile pattern
        if $pattern.compiled_pattern.is_none() {
            let compiled_result = super::CompiledPattern::from_pattern(&*$pattern.pattern);
            match compiled_result {
                Ok(compiled_result) => {
                    $pattern.compiled_pattern = Some(compiled_result);
                }
                Err(error) => {
                    eprintln!("{:#?}", error);
                    return;
                }
            };
        }

        // Search
        $ctx.frame_number += 1;
        $node.parse($pattern, $ctx);

        if $pattern.callback.is_some() {
            let ctx_clone = $ctx.clone();
            match Executor::get().execute($pattern, ctx_clone) {
                Ok(new_ctx) => *$ctx = new_ctx,
                Err(err) => {
                    eprintln!("Failed to execute callback: {:#?}", err);
                }
            }
        }

        $ctx.frame_number -= 1;
        if $ctx.frame_number == 0 {
            $ctx.clear_variables();
        }
    };
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
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
    DeclStmt,
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
    VarDecl: VarDecl,
    DeclStmt: DeclStmt
);

#[derive(Debug, new)]
pub struct CompiledPattern<'a> {
    pub pattern: Regex,
    pub variables: Vec<String>,

    match_result: Option<regex::Captures<'a>>,
    reference_vars: Vec<String>,
}

impl<'a> CompiledPattern<'a> {
    /// Create a compiled pattern from an MSD pattern and the context.
    pub fn from_pattern(pattern: &str) -> Result<CompiledPattern, regex::Error> {
        let tag_regex = Regex::new(r#"#(&)?\{([a-zA-Z_-]+)\}"#)?;

        let mut variables = vec![];
        let mut references = vec![];

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
            pattern = pattern.as_str().replace(s, &format!("(?P<{}>.*?)", name));

            // Register variables and references
            if is_ref {
                references.push(name.clone().into());
            }
            variables.push(name.into());
        }
        let transformed_pattern = Regex::new(pattern.as_str())?;
        Ok(CompiledPattern::new(
            transformed_pattern,
            variables,
            None,
            references,
        ))
    }

    pub fn matches(&mut self, match_str: &'a str, ctx: &ParserContext) -> bool {
        match self.pattern.captures(match_str) {
            Some(matches) => {
                for reference in self.reference_vars.iter() {
                    if ctx
                        .get_object(
                            matches
                                .name(reference)
                                .expect("Reference variable name not found in regex!")
                                .as_str(),
                        )
                        .is_none()
                    {
                        return false;
                    }
                }
                self.match_result = Some(matches);
                true
            }
            None => false,
        }
    }

    pub fn insert_match_result(&self, ctx: &mut ParserContext) {
        for var_name in self.variables.iter() {
            ctx.make_variable(
                &var_name,
                &self
                    .match_result
                    .as_ref()
                    .expect("Matches unset when saving results")
                    .name(&var_name)
                    .expect("Failed to match a variable name")
                    .as_str(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bson() {
        assert_eq!(
            "BSON_APPEND_(?P<type>.*?)_another_one",
            CompiledPattern::from_pattern("BSON_APPEND_#{type}_#&{another_one}")
                .expect("Failed to construct regex from pattern input")
                .pattern
                .as_str()
        );
    }
}
