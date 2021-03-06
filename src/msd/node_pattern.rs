use super::MsdNodeExplorer;
// use super::MsdDispatch;
use super::{pattern_parser::NodePatternParser, Executor};
use crate::ast::*;
use crate::msd::explorer::choose_exit;
use crate::prophet::*;
use derive_new::new;
use regex::Regex;
use serde::Deserialize;

use super::{ContextLocalVariableActions, ContextObjectActions, ParserContext};

/// A Node pattern describing a node of interest to the parser.
#[derive(Debug, Clone, Deserialize, new)]
pub struct NodePattern {
    /// The target AST node type
    pub identifier: NodeType,

    /// A regex-like pattern identifying the specific call.
    /// It's a modified Regex, but more importantly supports variables
    /// with a #varname syntax (with ## being a literal #).
    /// Variables indicate the information we are looking for, like URLs and entity names.
    #[serde(skip)]
    pub compiled_pattern: Option<CompiledPattern>,

    /// A pattern, like compiled_pattern, for checking the type of a variable for inforamtion
    #[serde(skip)]
    pub compiled_auxiliary_pattern: Option<CompiledPattern>,

    /// Sub-patterns for this node pattern to be matched in the AST.
    /// Some subpatterns may be specified as required.
    pub subpatterns: Vec<NodePattern>,

    /// A Rune script implementing the callback function interface
    pub callback: Option<String>,

    /// Indicates whether this pattern is essential for any higher order
    /// pattern to be matched successfully.
    pub essential: bool,

    /// Raw pattern defined by the user
    pub pattern: String,

    /// Raw pattern for checking the type defined by the user
    pub auxiliary_pattern: Option<String>,

    /// Transparently forward to child nodes
    #[serde(default = "bool::default")]
    pub transparent: bool,
}

impl NodePattern {
    pub fn matches(&self, node: &impl IntoMsdNode) -> bool {
        self.identifier == node.into_msd_node() || self.transparent
    }

    /// Lazy-compile the regexes on this NodePattern
    pub fn lazy_compile(&mut self) -> Option<()> {
        if self.compiled_pattern.is_none() {
            self.compiled_pattern = Some(compile_compiled_pattern(&*self.pattern)?);
        }
        if self.compiled_auxiliary_pattern.is_none() {
            if let Some(auxiliary_pattern) = &self.auxiliary_pattern {
                self.compiled_auxiliary_pattern =
                    Some(compile_compiled_pattern(&*auxiliary_pattern)?);
            }
        }
        Some(())
    }
}

pub fn compile_compiled_pattern(pattern: &str) -> Option<CompiledPattern> {
    let compiled_result = super::CompiledPattern::from_pattern(pattern);
    match compiled_result {
        Ok(compiled_result) => Some(compiled_result),
        Err(error) => {
            eprintln!("{:#?}", error);
            None
        }
    }
}

fn parse<N: NodePatternParser + MsdNodeExplorer>(
    pattern: &mut NodePattern,
    node: &mut N,
    ctx: &mut ParserContext,
) -> bool {
    if !pattern.transparent {
        node.parse(pattern, ctx).is_some()
    } else {
        let mut explore_all_found_essential = false;
        for subpattern in pattern.subpatterns.iter_mut() {
            if node.explore(subpattern, ctx).is_some() {
                explore_all_found_essential = true;
            }
        }
        choose_exit(pattern.essential, explore_all_found_essential).is_some()
    }
}

/// Parse an individual node with this NodePattern, lazily-initializing its CompiledPattern as needed
pub fn msd_node_parse<N: NodePatternParser + MsdNodeExplorer>(
    pattern: &mut NodePattern,
    node: &mut N,
    ctx: &mut ParserContext,
) -> Option<()> {
    // Lazily compile patterns
    pattern.lazy_compile()?;

    // Search
    ctx.frame_number += 1;

    let mut transaction = ctx.clone();
    let passed = if parse(pattern, node, &mut transaction) {
        if pattern.callback.is_some() {
            let tmp = transaction.clone();
            match Executor::get().execute(pattern, transaction) {
                Ok(new_ctx) => {
                    *ctx = new_ctx;
                    true
                }
                Err(err) => {
                    eprintln!(
                        "Failed to execute callback ({:#?}) for: {:?}\n{:#?}",
                        err, pattern.callback, tmp
                    );
                    false
                }
            }
        } else {
            *ctx = transaction;
            true
        }
    } else {
        false
    };

    ctx.frame_number -= 1;
    if ctx.frame_number == 0 {
        ctx.clear_variables();
    }

    // Resume the search where we left off, or pass on an error indicating a required subpattern
    // failed to match, so this pattern should be considered failed.
    if !passed && ctx.frame_number != 0 {
        None
    } else {
        Some(())
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum NodeType {
    // Prophet nodes
    ClassOrInterface,
    Method,
    MethodParam,
    Field,
    Annotation,
    AnnotationValuePair,

    // Body nodes
    CallExpr,
    VarDecl,
    DeclStmt,
    Ident,
    Literal,
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
    AnnotationValuePair: AnnotationValuePair,
    // Body node types
    CallExpr: CallExpr,
    VarDecl: VarDecl,
    DeclStmt: DeclStmt,
    Ident: Ident,
    Literal: Literal
);

#[derive(Debug, Clone, new)]
pub struct CompiledPattern {
    pub pattern: Regex,
    pub variables: Vec<String>,
    reference_vars: Vec<String>,
}

impl CompiledPattern {
    /// Create a compiled pattern from an MSD pattern and the context.
    pub fn from_pattern(pattern: &str) -> Result<CompiledPattern, regex::Error> {
        let tag_regex = Regex::new(r#"#(&)?\{([a-zA-Z_-]+)\}(\((.+)\))?"#)?;

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

            // Find a user-defined capture definition
            let mut regex_pattern = ".*";
            if let Some(group) = &captures.get(4) {
                regex_pattern = group.as_str();
            }

            // Replace the capture with the processed capture group
            pattern = pattern
                .as_str()
                .replace(s, &format!("(?P<{}>{})", name, regex_pattern));

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
            references,
        ))
    }

    pub fn matches(&self, match_str: &str, ctx: &ParserContext) -> bool {
        let tmp = self.pattern.clone();
        match tmp.captures(&*match_str) {
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
                        println!("Failed to find {}={:?}", reference, matches.name(reference));
                        return false;
                    }
                }
                true
            }
            None => false,
        }
    }

    pub fn match_and_insert(&self, match_str: &str, ctx: &mut ParserContext) -> bool {
        let tmp_pattern = self.pattern.clone();
        match tmp_pattern.captures(&*match_str) {
            Some(matches) => {
                // Verify all references
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

                // Extract variables to context
                for var_name in self.variables.iter() {
                    ctx.make_variable(
                        &var_name,
                        matches
                            .name(&var_name)
                            .expect("Failed to match a variable name")
                            .clone()
                            .as_str(),
                    );
                }
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bson() {
        assert_eq!(
            "BSON_APPEND_(?P<type>.*?)_(?P<another_one>.*?)",
            CompiledPattern::from_pattern("BSON_APPEND_#{type}_#&{another_one}")
                .expect("Failed to construct regex from pattern input")
                .pattern
                .as_str()
        );
    }
}
