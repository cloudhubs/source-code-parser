use super::{msd_node_parse, NodePattern, ParserContext};
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
pub trait MsdNodeExplorer {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()>;
}

/// Create a no-operation implementation of exploring a node
#[macro_export]
macro_rules! msd_dispatch_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl MsdNodeExplorer for $struct_name {
                fn explore(&mut self, pattern: &mut NodePattern, _ctx: &mut ParserContext) -> Option<()> {
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

/// Create an implementation of exploring a node that visits a set of fields in the node directly
#[macro_export]
macro_rules! msd_dispatch_single_dispatch_impl {
    ( $( $struct_name:ty: { $( $to_explore:ident ),+ } ),+ ) => {
        $(
            impl MsdNodeExplorer for $struct_name {
                fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
                    use crate::msd::explorer::choose_exit;

                    let mut found_essential = false;
                    $(
                        if self.$to_explore.explore(pattern, ctx).is_some() {
                            found_essential = true;
                        }
                    )*
                    choose_exit(pattern.essential, found_essential)
                }
            }
        )*
    };
}

/// Create an implementation of exploring a node that visits ll elements in a a set of collections in the node
#[macro_export]
macro_rules! msd_dispatch_collection_impl {
    ( $( $struct_name:ty: { $( $to_explore:ident ),+ } ),+ ) => {
        $(
            impl MsdNodeExplorer for $struct_name {
                fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
                    use crate::msd::explorer::choose_exit;

                    let mut found_essential = false;
                    $(
                        if explore_all!(pattern, ctx, self.$to_explore).is_some() {
                            found_essential = true;
                        }
                    )*
                    choose_exit(pattern.essential, found_essential)
                }
            }
        )*
    };
}

/// Explore all elements in the provided collections
#[macro_export]
macro_rules! explore_all {
    ( $pattern:expr, $ctx:expr, $( $explorable:expr ),+ ) => {{
        use crate::msd::explorer::choose_exit;

        let mut explore_all_found_essential = false;
        $(
            for x in $explorable.iter_mut() {
                if x.explore($pattern, $ctx).is_some() {
                    explore_all_found_essential = true;
                }
            }
        )*
        choose_exit($pattern.essential, explore_all_found_essential)
    }};
}

pub(crate) fn choose_exit(essential: bool, found: bool) -> Option<()> {
    // If an essential node had no matches, it's a failure; otherwise, we're good
    if essential && !found {
        None
    } else {
        Some(())
    }
}

msd_dispatch_default_impl!(
    BinaryExpr,
    UnaryExpr,
    ParenExpr,
    DotExpr,
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

msd_dispatch_single_dispatch_impl!(
    LambdaExpr: { body },
    CaseExpr: { body },
    ExprStmt: { expr },
    CatchStmt: { body },
    WhileStmt: { condition, body },
    DoWhileStmt: { condition, body },
    WithResourceStmt: { body, resources }
);

msd_dispatch_collection_impl!(
    ModuleComponent: { classes, interfaces },
    AssignExpr: { rhs, lhs },
    InitListExpr: { exprs },
    SwitchExpr: { cases },
    Block: { nodes }
);

// Information-Bearing Node Exploration
impl MsdNodeExplorer for Ident {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)
        } else if pattern.essential {
            None
        } else {
            Some(())
        }
    }
}

impl MsdNodeExplorer for Literal {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)
        } else if pattern.essential {
            None
        } else {
            Some(())
        }
    }
}

impl MsdNodeExplorer for ClassOrInterfaceComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        explore_all!(
            pattern,
            ctx,
            self.annotations,
            self.constructors,
            self.field_components,
            self.component.methods
        )
    }
}

impl MsdNodeExplorer for MethodComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        if let Some(block) = &mut self.body {
            block.explore(pattern, ctx)?;
        }

        explore_all!(
            pattern,
            ctx,
            self.annotations,
            self.parameters,
            self.sub_methods
        )
    }
}

impl MsdNodeExplorer for MethodParamComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        if let Some(annotations) = &mut self.annotation {
            explore_all!(pattern, ctx, annotations)
        } else {
            Some(())
        }
    }
}

impl MsdNodeExplorer for FieldComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        explore_all!(pattern, ctx, self.annotations)
    }
}

impl MsdNodeExplorer for DeclStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        explore_all!(
            pattern,
            ctx,
            self.variables,
            self.expressions
                .iter_mut()
                .flat_map(|e| e)
                .collect::<Vec<_>>()
        )
    }
}

impl MsdNodeExplorer for VarDecl {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        explore_all!(pattern, ctx, self.annotation)
    }
}

impl MsdNodeExplorer for CallExpr {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        let name_result = self.name.as_mut().explore(pattern, ctx);
        let explore_all_result = explore_all!(pattern, ctx, self.args);
        choose_exit(
            pattern.essential,
            name_result.is_some() || explore_all_result.is_some(),
        )
    }
}

impl MsdNodeExplorer for AnnotationComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)?;
        }

        // Visit other nodes
        explore_all!(pattern, ctx, self.key_value_pairs)
    }
}

impl MsdNodeExplorer for AnnotationValuePair {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx)
        } else if pattern.essential {
            None
        } else {
            Some(())
        }
    }
}

// Nodes requiring custom visting because a general macro didn't cover their edge cases

impl MsdNodeExplorer for IfStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        let body_result = self.body.explore(pattern, ctx);
        if let Some(else_body) = &mut self.else_body {
            let else_result = else_body.explore(pattern, ctx);
            choose_exit(
                pattern.essential,
                body_result.is_some() || else_result.is_some(),
            )
        } else {
            body_result
        }
    }
}

impl MsdNodeExplorer for ForStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        let mut found = explore_all!(pattern, ctx, self.init, self.post).is_some();
        if let Some(condition) = &mut self.condition {
            found = found || condition.explore(pattern, ctx).is_some();
        }
        found = found || self.body.explore(pattern, ctx).is_some();
        choose_exit(pattern.essential, found)
    }
}
impl MsdNodeExplorer for ForRangeStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        let mut found = self.body.explore(pattern, ctx).is_some();
        if let Some(iter) = &mut self.iterator {
            found = found || iter.explore(pattern, ctx).is_some();
        }
        choose_exit(pattern.essential, found)
    }
}

impl MsdNodeExplorer for TryCatchStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        let mut found = self.try_body.explore(pattern, ctx).is_some();
        if let Some(finally_body) = &mut self.finally_body {
            found = found || finally_body.explore(pattern, ctx).is_some();
        }
        choose_exit(pattern.essential, found)
    }
}

impl MsdNodeExplorer for ReturnStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        if let Some(expr) = &mut self.expr {
            expr.explore(pattern, ctx)
        } else if pattern.essential {
            None
        } else {
            Some(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::msd::NodeType;

    use super::*;

    #[test]
    fn does_this_call() {
        let mut c: Expr = CallExpr::new(Box::new(Ident::new("".into()).into()), vec![]).into();
        let mut np = NodePattern::new(
            NodeType::CallExpr,
            None,
            None,
            vec![],
            None,
            true,
            "".into(),
            None,
        );
        eprintln!("hello?");
        c.explore(&mut np, &mut ParserContext::default());
    }
}
