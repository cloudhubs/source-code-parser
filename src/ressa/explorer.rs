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
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()>;
}

/// Create a no-operation implementation of exploring a node
#[macro_export]
macro_rules! ressa_dispatch_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
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
macro_rules! ressa_dispatch_single_dispatch_impl {
    ( $( $struct_name:ty: { $( $to_explore:ident ),+ } ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
                    use crate::ressa::explorer::choose_exit;

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

/// Create an implementation of exploring a node that visits all elements in a a set of collections in the node
#[macro_export]
macro_rules! ressa_dispatch_collection_impl {
    ( $( $struct_name:ty: { $( $to_explore:ident ),+ } ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
                    use crate::ressa::explorer::choose_exit;

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
        use crate::ressa::explorer::choose_exit;

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

/// Runs all provided statements, and returns the recommended exit.
/// All statements within the braces must evaluate to a boolean.
macro_rules! run_then_exit {
    ( $essential:expr => { $( $lines:expr );+; } ) => {{
        let mut found = false;
        $( if $lines { found = true; }; )*
        choose_exit($essential, found)
    }};
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

ressa_dispatch_single_dispatch_impl!(
    LambdaExpr: { body },
    CaseExpr: { body },
    ExprStmt: { expr },
    CatchStmt: { body },
    WhileStmt: { condition, body },
    DoWhileStmt: { condition, body },
    WithResourceStmt: { body, resources },
    BinaryExpr: { lhs, rhs },
    UnaryExpr: { expr },
    ParenExpr: { expr },
    DotExpr: { expr, selected }
);

ressa_dispatch_collection_impl!(
    ModuleComponent: { classes, interfaces },
    AssignExpr: { rhs, lhs },
    InitListExpr: { exprs },
    SwitchExpr: { cases },
    Block: { nodes }
);

// Information-Bearing Node Exploration
impl RessaNodeExplorer for Ident {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };
        })
    }
}

impl RessaNodeExplorer for Literal {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };
        })
    }
}

impl RessaNodeExplorer for ClassOrInterfaceComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            explore_all!(
                pattern,
                ctx,
                self.annotations,
                self.constructors,
                self.field_components,
                self.component.methods
            ).is_some();
        })
    }
}

impl RessaNodeExplorer for MethodComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            // Visit other nodes
            if let Some(block) = &mut self.body {
                block.explore(pattern, ctx).is_some()
            } else {
                false
            };

            explore_all!(
                pattern,
                ctx,
                self.annotations,
                self.parameters,
                self.sub_methods
            ).is_some();
        })
    }
}

impl RessaNodeExplorer for MethodParamComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            // Visit other nodes
            if let Some(annotations) = &mut self.annotation {
                explore_all!(pattern, ctx, annotations).is_some()
            } else {
                false
            };
        })
    }
}

impl RessaNodeExplorer for FieldComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            // Visit other nodes
            explore_all!(pattern, ctx, self.annotations).is_some();
        })
    }
}

impl RessaNodeExplorer for DeclStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            // Visit other nodes
            explore_all!(
                pattern,
                ctx,
                self.variables,
                self.expressions
                    .iter_mut()
                    .flat_map(|e| e)
                    .collect::<Vec<_>>()
            ).is_some();
        })
    }
}

impl RessaNodeExplorer for VarDecl {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            // Visit other nodes
            explore_all!(pattern, ctx, self.annotation).is_some();
        })
    }
}

impl RessaNodeExplorer for CallExpr {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            // Visit other nodes
            self.name.as_mut().explore(pattern, ctx).is_some();
            explore_all!(pattern, ctx, self.args).is_some();
        })
    }
}

impl RessaNodeExplorer for AnnotationComponent {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            // Check if this node needs parsed
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };

            // Visit other nodes
            explore_all!(pattern, ctx, self.key_value_pairs).is_some();
        })
    }
}

impl RessaNodeExplorer for AnnotationValuePair {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            if pattern.matches(self) {
                ressa_node_parse(pattern, self, ctx).is_some()
            } else {
                false
            };
        })
    }
}

// Nodes requiring custom visting because a general macro didn't cover their edge cases

impl RessaNodeExplorer for IfStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            self.cond.explore(pattern, ctx).is_some();
            self.body.explore(pattern, ctx).is_some();
            if let Some(else_body) = &mut self.else_body {
                else_body.explore(pattern, ctx).is_some()
            } else {
                false
            };
        })
    }
}

impl RessaNodeExplorer for ForStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            explore_all!(pattern, ctx, self.init, self.post).is_some();
            if let Some(condition) = &mut self.condition {
                condition.explore(pattern, ctx).is_some()
            } else {
                false
            };
            self.body.explore(pattern, ctx).is_some();
        })
    }
}
impl RessaNodeExplorer for ForRangeStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            self.init.as_mut().explore(pattern, ctx).is_some();
            self.body.explore(pattern, ctx).is_some();
            if let Some(iter) = &mut self.iterator {
                iter.explore(pattern, ctx).is_some()
            } else {
                false
            };
        })
    }
}

impl RessaNodeExplorer for TryCatchStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            self.try_body.explore(pattern, ctx).is_some();
            if let Some(finally_body) = &mut self.finally_body {
                finally_body.explore(pattern, ctx).is_some()
            } else {
                false
            };
            explore_all!(pattern, ctx, self.catch_bodies).is_some();
        })
    }
}

impl RessaNodeExplorer for ReturnStmt {
    fn explore(&mut self, pattern: &mut NodePattern, ctx: &mut ParserContext) -> Option<()> {
        run_then_exit!(pattern.essential => {
            if let Some(expr) = &mut self.expr {
                expr.explore(pattern, ctx).is_some()
            } else {
                false
            };
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ressa::NodeType;

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
            false,
        );
        tracing::warn!("hello?");
        c.explore(&mut np, &mut ParserContext::default());
    }
}
