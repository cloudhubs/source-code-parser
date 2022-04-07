use super::{ExplorerContext, LaastIndex, RessaNodeExplorer};
// use super::ressaDispatch;
use super::{pattern_parser::NodePatternParser, Executor};
use crate::ast::*;
use crate::prophet::*;
use crate::ressa::explorer::choose_exit;
use derive_new::new;
use regex::Regex;
use serde::Deserialize;
use std::cell::RefCell;

use super::{ContextLocalVariableActions, ContextObjectActions, ParserContext};

// There really isn't any way around the too many arguments in the new method here
// aside from not using a `new` method
#[allow(clippy::too_many_arguments)]
/// A ReSSA Parser describing a node of interest.
#[derive(Debug, Clone, Deserialize, new)]
pub struct NodePattern {
    /// The target AST node type
    pub identifier: NodeType,

    /// A regex-like pattern identifying the specific call.
    /// It's a modified Regex, but more importantly supports variables
    /// with a #varname syntax (with ## being a literal #).
    /// Variables indicate the information we are looking for, like URLs and entity names.
    #[serde(skip)]
    #[serde(default)]
    pub compiled_pattern: RefCell<Option<CompiledPattern>>,

    /// A pattern, like compiled_pattern, for checking the type of a variable for inforamtion
    #[serde(skip)]
    #[serde(default)]
    pub compiled_auxiliary_pattern: RefCell<Option<CompiledPattern>>,

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

    /// Language this node applies to
    #[serde(default = "Option::default")]
    pub language: Option<Language>,
}

impl NodePattern {
    /// Determine whether this node matches the language of the provided node
    /// If no language initialized, assume failure.
    pub fn language_matches(&self, node: &impl NodeLanguage) -> bool {
        match self.language {
            Some(lang) => lang == Language::Unknown || lang == node.get_language(),
            None => false,
        }
    }

    /// Determine whether this node's fields match the provided node. Does not verify language.
    pub fn matches(&self, node: &impl IntoRessaNode) -> bool {
        self.identifier == node.into_ressa_node() || self.transparent
    }

    // Consume self to update language references. Unsure if this is the best way.
    pub fn cascade_language(mut self, parent_lang: Language) -> NodePattern {
        // Update own language if unknown
        let parent_lang = match self.language {
            Some(lang) => lang,
            None => {
                self.language = Some(parent_lang);
                parent_lang
            }
        };

        // Cascade to children
        self.subpatterns = self
            .subpatterns
            .into_iter()
            .map(|child| child.cascade_language(parent_lang))
            .collect();
        self
    }

    /// Lazy-compile the regexes on this NodePattern
    pub fn lazy_compile(&self) -> Option<()> {
        let mut compiled_pattern = self.compiled_pattern.borrow_mut();
        if compiled_pattern.is_none() {
            *compiled_pattern = Some(compile_compiled_pattern(&*self.pattern)?);
        }
        let mut compiled_auxiliary_pattern = self.compiled_auxiliary_pattern.borrow_mut();
        if compiled_auxiliary_pattern.is_none() {
            if let Some(auxiliary_pattern) = &self.auxiliary_pattern {
                *compiled_auxiliary_pattern = Some(compile_compiled_pattern(&*auxiliary_pattern)?);
            }
        }
        Some(())
    }
}

impl NodeLanguage for NodePattern {
    fn get_language(&self) -> Language {
        self.language.unwrap_or_default()
    }
}

/// Optionally compile a provided pattern, if possible
pub fn compile_compiled_pattern(pattern: &str) -> Option<CompiledPattern> {
    let compiled_result = super::CompiledPattern::from_pattern(pattern);
    match compiled_result {
        Ok(compiled_result) => Some(compiled_result),
        Err(error) => {
            tracing::warn!("Error compiling pattern: {:#?}", error);
            None
        }
    }
}

/// Parse the results
fn parse<N: NodePatternParser + RessaNodeExplorer>(
    pattern: &NodePattern,
    node: &N,
    ctx: &mut ExplorerContext,
    index: &LaastIndex,
) -> bool {
    if !pattern.transparent {
        node.parse(pattern, ctx, index).is_some()
    } else {
        let mut explore_all_found_essential = false;
        for subpattern in pattern.subpatterns.iter() {
            if node.explore(subpattern, ctx, index).is_some() {
                explore_all_found_essential = true;
            }
        }
        choose_exit(pattern.essential, explore_all_found_essential).is_some()
    }
}

/// Run general ReSSA match acquired routine: compile patterns if needed, maintain context transactions, parse the node pattern, and
pub fn ressa_node_parse<N: NodePatternParser + RessaNodeExplorer>(
    pattern: &NodePattern,
    node: &N,
    ctx: &mut ExplorerContext,
    index: &LaastIndex,
) -> Option<()> {
    // Lazily compile patterns
    pattern.lazy_compile()?;

    // Search
    ctx.frame_number += 1;

    let mut transaction = ctx.clone();
    let passed = if parse(pattern, node, &mut transaction, index) {
        if pattern.callback.is_some() {
            // let tmp = transaction.clone();
            match Executor::get().execute(pattern, transaction.parser) {
                Ok(new_ctx) => {
                    ctx.parser = new_ctx;
                    true
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to execute callback ({}) for: {:?}",
                        err,
                        pattern.callback
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
        ctx.parser.clear_variables();
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
    BinaryExpr,
}

pub trait IntoRessaNode {
    // The normal convention is to pass an owned self for into_* methods, we maybe
    // could rename this to follow conventions more closely but I don't think it's
    // a big deal here
    #[allow(clippy::wrong_self_convention)]
    fn into_ressa_node(&self) -> NodeType;
}

#[macro_export]
macro_rules! into_ressa_node {
    ( $( $struct_name:ty: $name:ident ),+ ) => {
        $(
            impl IntoRessaNode for $struct_name {
                fn into_ressa_node(&self) -> NodeType {
                    NodeType::$name
                }
            }
        )*
    };
}

into_ressa_node!(
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
    Literal: Literal,
    BinaryExpr: BinaryExpr
);

#[derive(Debug, Clone, new)]
pub struct CompiledPattern {
    pub pattern: Regex,
    pub variables: Vec<String>,
    reference_vars: Vec<String>,
}

impl CompiledPattern {
    /// Create a compiled pattern from a ReSSA pattern and the context.
    pub fn from_pattern(pattern: &str) -> Result<CompiledPattern, regex::Error> {
        /*
         * Expanding and explaining the following Regex for posterity:
         *
         * #(&)?              - Variable declared with #, maybe as a reference (&)
         * \{                 1 Followed by literal curly braces
         *   ([a-zA-Z_-]+)    - Wrapping a capture of the variable's name
         * \}                 1
         * (                  1 Optionally capture
         *   \(               2 Something wrapped in parenthesis literals
         *     (              3 Containing the desired RegEx pattern consisting of
         *       (            4 Any number of
         *         [^(]       - Characters other than opening parentheses
         *         |\\\(      - Or escaped opening parenthesis
         *         |\(        5 Or a clause wrapped in literal open/close parenthesis
         *           (        6 Containing any number of
         *             [^)]   - characters other than closing parentheses
         *             |\\\)  - Or escaped closing parenthesis
         *           )*       6
         *         \)         5
         *       )*           4
         *     )              3
         *   \)               2
         * )?                 1
         */
        let tag_regex =
            Regex::new(r#"#(&)?\{([a-zA-Z_-]+)\}(\((([^(]|\\\(|\(([^)]|\\\))*\))*)\))?"#)?;

        let mut variables = vec![];
        let mut references = vec![];

        let matches = tag_regex.captures_iter(pattern);
        let mut pattern: String = pattern.into();
        for captures in matches {
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
                references.push(name.into());
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
                        .get(
                            matches
                                .name(reference)
                                .expect("Reference variable name not found in regex!")
                                .as_str(),
                        )
                        .is_none()
                    {
                        tracing::info!(
                            "Failed to find {}={:?}",
                            reference,
                            matches.name(reference)
                        );
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
                        .get(
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
                        var_name,
                        matches
                            .name(var_name)
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
            "BSON_APPEND_(?P<type>.*)_(?P<another_one>.*)",
            CompiledPattern::from_pattern("BSON_APPEND_#{type}_#&{another_one}")
                .expect("Failed to construct regex from pattern input")
                .pattern
                .as_str()
        );
    }

    #[test]
    fn test_tt() {
        assert_eq!(
            "(https?://)(?P<host>[^/]*)/(?P<path_var_val>.+)/?\"",
            CompiledPattern::from_pattern("(https?://)#{host}([^/]*)/#{path_var_val}(.+)/?\"")
                .expect("Failed to construct regex from pattern input")
                .pattern
                .as_str()
        );
    }
}
