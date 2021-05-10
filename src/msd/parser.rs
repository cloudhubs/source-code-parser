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
}

/// Describes how to visit the nodes in the AST to find nodes of interest
#[enum_dispatch(Node)]
#[enum_dispatch(Expr)]
#[enum_dispatch(Stmt)]
#[enum_dispatch(ProphetNode)]
pub trait MsdNodeExplorer {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext);
}

/// Create a no-operation implementation of exploring a node
#[macro_export]
macro_rules! msd_dispatch_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl MsdNodeExplorer for $struct_name {
                fn explore(&mut self, _pattern: &mut NodePattern<'_>, _ctx: &mut ParserContext)  {}
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
                fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext)  {
                    $(
                        self.$to_explore.explore(pattern, ctx);
                    )*
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
                fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext)  {
                    $(
                        explore_all!(pattern, ctx, self.$to_explore);
                    )*
                }
            }
        )*
    };
}

/// Explore all elements in the provided collections
#[macro_export]
macro_rules! explore_all {
    ( $pattern:expr, $ctx:expr, $( $explorable:expr ),+ ) => {
        $(
            for x in $explorable.iter_mut() {
                x.explore($pattern, $ctx);
            }
        )*
    };
}

msd_dispatch_default_impl!(
    BinaryExpr,
    UnaryExpr,
    ParenExpr,
    DotExpr,
    IncDecExpr,
    LogExpr,
    Ident,
    Literal,
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
impl MsdNodeExplorer for ClassOrInterfaceComponent {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
        }

        // Visit other nodes
        explore_all!(
            pattern,
            ctx,
            self.annotations,
            self.constructors,
            self.field_components,
            self.component.methods
        );
    }
}

impl MsdNodeExplorer for MethodComponent {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
        }

        // Visit other nodes
        if let Some(block) = &mut self.body {
            block.explore(pattern, ctx);
        }
        explore_all!(
            pattern,
            ctx,
            self.annotations,
            self.parameters,
            self.sub_methods
        );
    }
}

impl MsdNodeExplorer for MethodParamComponent {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
        }

        // Visit other nodes
        if let Some(annotations) = &mut self.annotation {
            explore_all!(pattern, ctx, annotations);
        }
    }
}

impl MsdNodeExplorer for FieldComponent {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
        }

        // Visit other nodes
        explore_all!(pattern, ctx, self.annotations);
    }
}

impl MsdNodeExplorer for DeclStmt {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
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
        );
    }
}

impl MsdNodeExplorer for VarDecl {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
        }

        // Visit other nodes
        explore_all!(pattern, ctx, self.annotation);
    }
}

impl MsdNodeExplorer for CallExpr {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
        }

        // Visit other nodes
        self.name.as_mut().explore(pattern, ctx);
        explore_all!(pattern, ctx, self.args);
    }
}

impl MsdNodeExplorer for AnnotationComponent {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            msd_node_parse(pattern, self, ctx);
        }

        // Visit other nodes
        // TODO visit key_value_pairs?
        // explore_all!(pattern, ctx, self.key_value_pairs);
    }
}

// Nodes requiring custom visting because a general macro didn't cover their edge cases

impl MsdNodeExplorer for IfStmt {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        self.body.explore(pattern, ctx);
        if let Some(else_body) = &mut self.else_body {
            else_body.explore(pattern, ctx);
        }
    }
}

impl MsdNodeExplorer for ForStmt {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        explore_all!(pattern, ctx, self.init);
        if let Some(condition) = &mut self.condition {
            condition.explore(pattern, ctx);
        }
        explore_all!(pattern, ctx, self.post);
        self.body.explore(pattern, ctx)
    }
}
impl MsdNodeExplorer for ForRangeStmt {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        if let Some(iter) = &mut self.iterator {
            iter.explore(pattern, ctx);
        }
        self.body.explore(pattern, ctx)
    }
}

impl MsdNodeExplorer for TryCatchStmt {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        self.try_body.explore(pattern, ctx);
        if let Some(finally_body) = &mut self.finally_body {
            finally_body.explore(pattern, ctx);
        }
    }
}

impl MsdNodeExplorer for ReturnStmt {
    fn explore(&mut self, pattern: &mut NodePattern<'_>, ctx: &mut ParserContext) {
        if let Some(expr) = &mut self.expr {
            expr.explore(pattern, ctx);
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
        c.explore(&mut np, &mut ParserContext::default())
    }
}
