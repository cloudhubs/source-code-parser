use super::{
    check_match, CachedCompiledPattern, Constraint, ExplorerContext, Indexable, LaastIndex,
    ParserContext, RessaNodeExplorer,
};
// use super::ressaDispatch;
use super::{pattern_parser::NodePatternParser, Executor};
use crate::ast::*;
use crate::prophet::*;
use crate::ressa::explorer::choose_exit;
use derive_new::new;
use serde::Deserialize;

use super::ContextLocalVariableActions;

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
    pub compiled_pattern: CachedCompiledPattern,

    /// A pattern, like compiled_pattern, for checking the type of a variable for inforamtion
    #[serde(skip)]
    #[serde(default)]
    pub compiled_auxiliary_pattern: CachedCompiledPattern,

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

    /// Any constraint for matching this node
    pub constraint: Option<Constraint>,

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

    pub fn match_regexes(
        &self,
        primary: &str,
        auxiliary: &str,
        ctx: &mut ParserContext,
    ) -> Option<()> {
        // Verify regexes compile
        let primary_ref = self.compiled_pattern.borrow();
        let primary_pattern = primary_ref.as_ref()?;
        let auxiliary_ref = self.compiled_auxiliary_pattern.borrow();
        let auxiliary_pattern = auxiliary_ref.as_ref();
        if self.auxiliary_pattern.is_some() && auxiliary_pattern.is_none() {
            return None;
        }

        // Check for matches (verify primary matches, write auxiliary escaping if fail, write primary escaping if fail)
        if !primary_pattern.matches(primary, ctx) {
            return None;
        }
        if let Some(pattern) = auxiliary_pattern {
            pattern.match_callback(auxiliary, ctx)?();
        }
        primary_pattern.match_callback(primary, ctx)?();
        Some(())
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
    pub fn lazy_compile(&self) -> bool {
        if self
            .compiled_pattern
            .get_or_compile(&self.pattern)
            .is_none()
        {
            return false;
        }
        if let Some(auxiliary_pattern) = self.auxiliary_pattern.as_ref() {
            if self
                .compiled_auxiliary_pattern
                .get_or_compile(auxiliary_pattern)
                .is_none()
            {
                return false;
            }
        }
        true
    }
}

impl NodeLanguage for NodePattern {
    fn get_language(&self) -> Language {
        self.language.unwrap_or_default()
    }
}

/// Parse the results
fn parse<N: Indexable + IntoRessaNode + NodePatternParser>(
    pattern: &NodePattern,
    node: &N,
    ctx: &mut ExplorerContext,
    index: &LaastIndex,
) -> bool {
    if !pattern.transparent {
        // Check constraint if present, or default to true
        let matches_constraint = pattern
            .constraint
            .as_ref()
            .map(|constraint| ctx.constraint_stack.check(constraint))
            .unwrap_or(true);
        if matches_constraint {
            node.parse(pattern, ctx, index).is_some()
        } else {
            false
        }
    } else {
        let mut explore_all_found_essential = false;
        for subpattern in pattern.subpatterns.iter() {
            if check_match(node, subpattern, ctx, index).unwrap_or(false) {
                explore_all_found_essential = true;
            }
        }
        choose_exit(pattern.essential, explore_all_found_essential).is_some()
    }
}

/// Run general ReSSA match acquired routine: compile patterns if needed, maintain context transactions, parse the node pattern, and
pub fn ressa_node_parse<N: Indexable + IntoRessaNode + NodePatternParser>(
    pattern: &NodePattern,
    node: &N,
    ctx: &mut ExplorerContext,
    index: &LaastIndex,
) -> Option<()> {
    // Lazily compile patterns
    if !pattern.lazy_compile() {
        return None;
    }

    // Search
    ctx.frame_number += 1;

    let mut transaction = ctx.clone();
    let passed = if parse(pattern, node, &mut transaction, index) {
        // for (k, v) in ctx.constraint_stack.constraints.iter() {
        //     println!("{:?}: {} entries", k, v.len());
        //     for constraint in v.iter().filter(|x| x.guaranteed) {
        //         println!("{}", constraint);
        //     }
        //     println!();
        // }
        // println!("End of context\n\n");
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
    if !passed && pattern.essential && ctx.frame_number != 0 {
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
    AssignExpr,
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
    BinaryExpr: BinaryExpr,
    AssignExpr: AssignExpr
);
