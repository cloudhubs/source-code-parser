/// WARNING: HERE THERE BE MACROS
use super::{ressa_node_parse, NodePattern, ParserContext};
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
        pattern: &mut NodePattern,
        ctx: &mut ParserContext,
        index: &LaastIndex,
    ) -> Option<()>;
}

/// Create a no-operation implementation of exploring a node
#[macro_export]
macro_rules! ressa_dispatch_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(
                    &self,
                    pattern: &mut NodePattern,
                    _ctx: &mut ParserContext,
                    _index: &LaastIndex
                ) -> Option<()> {
                    if pattern.essential {
                        None
                    } else {
                        Some(())
                    }
                }
            }
        )*
    };
}

/// Generate a default explore implementation that delegates to the child fields (blanket implementation caused problems)
macro_rules! ressa_dispatch_delegate_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(
                    &self,
                    pattern: &mut NodePattern,
                    ctx: &mut ParserContext,
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
                    pattern: &mut NodePattern,
                    ctx: &mut ParserContext,
                    index: &LaastIndex
                ) -> Option<()> {
                    use crate::ressa::explorer::*;

                    // Record if this node matched
                    let found = if pattern.matches(self) {
                        ressa_node_parse(pattern, self, ctx, index).is_some()
                    } else {
                        false
                    };

                    // Explore subnodes, then return if this node matched
                    let matched = explore(self, pattern, ctx, index).is_some();
                    choose_exit(pattern.essential, matched || found)
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
    BinaryExpr,
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
    ReturnStmt
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
    AnnotationValuePair
);

pub fn explore<T>(
    source: &T,
    pattern: &mut NodePattern,
    ctx: &mut ParserContext,
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

#[cfg(test)]
mod tests {
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
        let mut np = NodePattern::new(
            NodeType::CallExpr,
            None,
            None,
            vec![],
            None,
            true,
            "".into(),
            None,
            false,
            None,
        );
        tracing::warn!("hello?");
        // c.explore(&mut np, &mut ParserContext::default());
    }
}
