use super::Indexable;
/// WARNING: HERE THERE BE MACROS
use super::{ressa_node_parse, NodePattern, ParserContext};
use crate::ast::*;
use crate::prophet::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub enum ProphetNode {
    ClassOrInterfaceComponent(ClassOrInterfaceComponent),
    MethodComponent(MethodComponent),
    MethodParamComponent(MethodParamComponent),
    FieldComponent(FieldComponent),
    AnnotationComponent(AnnotationComponent),
    AnnotationValuePair(AnnotationValuePair),
}

/// Describes how to visit the nodes in the AST to find nodes of interest
#[enum_dispatch(Node)]
#[enum_dispatch(Expr)]
#[enum_dispatch(Stmt)]
#[enum_dispatch(ProphetNode)]
pub trait RessaNodeExplorer {
    fn explore(&self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()>;
}

/// Create a no-operation implementation of exploring a node
#[macro_export]
macro_rules! ressa_dispatch_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(&self, pattern: &mut NodePattern, _ctx: &mut ParserContext) -> Option<()> {
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

/// Generate a default explore implementation that delegates to the child fields (blanked implementation caused problems)
macro_rules! ressa_dispatch_delegate_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(&self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
                    crate::ressa::explorer::explore(self, pattern, ctx)
                }
            }
        )*
    };
}

/// Generate a default explore implementation that delegates to the child fields (blanked implementation caused problems)
macro_rules! ressa_dispatch_match_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(&self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
                    use crate::ressa::explorer::*;

                    // Record if this node matched
                    let found = if pattern.matches(self) {
                        ressa_node_parse(pattern, self, ctx).is_some()
                    } else {
                        false
                    };

                    // Explore subnodes, but also return if this node matched
                    choose_exit(pattern.essential, explore(self, pattern, ctx).is_some() || found)
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

ressa_dispatch_default_impl!(
    IncDecExpr,
    LogExpr,
    IndexExpr,
    EndpointCallExpr,
    ImportStmt,
    BreakStmt,
    ContinueStmt,
    ThrowStmt,
    LabelStmt
);
ressa_dispatch_delegate_impl!(
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

pub fn explore<T>(source: &T, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()>
where
    T: Indexable,
{
    let mut result = false;
    for child in source.get_children().iter() {
        if child.explore(pattern, ctx).is_some() {
            result = true;
        }
    }
    choose_exit(pattern.essential, result)
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
        c.explore(&mut np, &mut ParserContext::default());
    }
}
