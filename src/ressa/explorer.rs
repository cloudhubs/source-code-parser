/// WARNING: HERE THERE BE MACROS
use super::{ressa_node_parse, ExplorerContext, IntoRessaNode, NodePattern, NodePatternParser};
use super::{Indexable, LaastIndex};
use crate::ast::*;
use crate::prophet::*;
use enum_dispatch::enum_dispatch;

/// Describes how to visit the nodes in the AST to find nodes of interest
#[enum_dispatch(Node)]
#[enum_dispatch(Expr)]
#[enum_dispatch(Stmt)]
pub trait RessaNodeExplorer {
    fn explore(
        &self,
        pattern: &NodePattern,
        ctx: &mut ExplorerContext,
        index: &LaastIndex,
    ) -> Option<()>;
}

/// Generate a default explore implementation that delegates to the child fields (blanket implementation caused problems)
macro_rules! ressa_dispatch_delegate_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(
                    &self,
                    pattern: &NodePattern,
                    ctx: &mut ExplorerContext,
                    index: &LaastIndex
                ) -> Option<()> {
                    crate::ressa::explorer::explore(self, pattern, ctx, index)
                }
            }
        )*
    };
}

/// Generate a default explore implementation that attempts to match, then delegates to the child fields
macro_rules! ressa_dispatch_match_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(
                    &self,
                    pattern: &NodePattern,
                    ctx: &mut ExplorerContext,
                    index: &LaastIndex
                ) -> Option<()> {
                    crate::ressa::explorer::explore_match(self, pattern, ctx, index)
                }
            }
        )*
    };
}

/// Determines the recommended exit based off of whether the Parser was essential, and whether it was matched
pub(crate) fn choose_exit(essential: bool, found: bool) -> Option<()> {
    // If an essential node had no matches, it's a failure; otherwise, we're good
    if essential && !found {
        None
    } else {
        Some(())
    }
}

ressa_dispatch_delegate_impl!(
    IncDecExpr,
    LogExpr,
    IndexExpr,
    EndpointCallExpr,
    ImportStmt,
    BreakStmt,
    ContinueStmt,
    ThrowStmt,
    LabelStmt,
    LambdaExpr,
    CaseExpr,
    ExprStmt,
    CatchStmt,
    WhileStmt,
    DoWhileStmt,
    WithResourceStmt,
    UnaryExpr,
    ParenExpr,
    DotExpr,
    ModuleComponent,
    AssignExpr,
    InitListExpr,
    SwitchExpr,
    Block,
    IfStmt,
    ForStmt,
    ForRangeStmt,
    TryCatchStmt,
    ReturnStmt,
    ContainerComponent
);
ressa_dispatch_match_impl!(
    Ident,
    Literal,
    ClassOrInterfaceComponent,
    MethodComponent,
    MethodParamComponent,
    FieldComponent,
    DeclStmt,
    VarDecl,
    CallExpr,
    AnnotationComponent,
    AnnotationValuePair,
    BinaryExpr
);

pub fn explore<T>(
    source: &T,
    pattern: &NodePattern,
    ctx: &mut ExplorerContext,
    index: &LaastIndex,
) -> Option<()>
where
    T: Indexable,
{
    let mut found_essential = false;
    for child in source.get_children().iter() {
        if child.explore(pattern, ctx, index).is_some() {
            found_essential = true;
        }
    }
    choose_exit(pattern.essential, found_essential)
}

pub fn explore_match<T>(
    source: &T,
    pattern: &NodePattern,
    ctx: &mut ExplorerContext,
    index: &LaastIndex,
) -> Option<()>
where
    T: Indexable + IntoRessaNode + NodePatternParser,
{
    // Check languages to validate remotely reasonable
    let lang_match = pattern.language_matches(source);
    if !lang_match && !index.language_in_subtree(pattern.get_language(), source) {
        return choose_exit(pattern.essential, false);
    }

    // Check if this node matches
    let found = if lang_match && pattern.matches(source) {
        ressa_node_parse(pattern, source, ctx, index).is_some()
    } else {
        false
    };

    // Explore subnodes, then return if this node matched
    let matched = explore(source, pattern, ctx, index).is_some();
    choose_exit(pattern.essential, matched || found)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use crate::ressa::NodeType;

    use super::*;

    #[test]
    fn does_this_call() {
        let c: Expr = CallExpr::new(
            Box::new(Ident::new("".into(), Language::Unknown).into()),
            vec![],
            Language::Unknown,
        )
        .into();
        let np = NodePattern::new(
            NodeType::CallExpr,
            RefCell::new(None),
            RefCell::new(None),
            vec![],
            None,
            true,
            "".into(),
            None,
            false,
            Some(Language::default()),
        );
        let index = LaastIndex::new(HashMap::new(), HashMap::new());
        tracing::warn!("hello?");
        c.explore(&np, &mut ExplorerContext::default(), &index);
    }
}
