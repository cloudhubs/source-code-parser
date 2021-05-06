use super::{NodePattern, ParserContext};
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

#[enum_dispatch(Node)]
#[enum_dispatch(Expr)]
#[enum_dispatch(Stmt)]
#[enum_dispatch(ProphetNode)]
pub trait MsdDispatch {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext);
}

#[macro_export]
macro_rules! msd_dispatch_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl MsdDispatch for $struct_name {
                fn explore(&self, _pattern: &NodePattern, _ctx: &mut ParserContext) {

                }
            }
        )*
    };
}

#[macro_export]
macro_rules! msd_dispatch_single_dispatch_impl {
    ( $( $struct_name:ty: { $( $to_explore:ident ),+ } ),+ ) => {
        $(
            impl MsdDispatch for $struct_name {
                fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
                    $(
                        self.$to_explore.explore(pattern, ctx);
                    )*
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! msd_dispatch_collection_impl {
    ( $( $struct_name:ty: { $( $to_explore:ident ),+ } ),+ ) => {
        $(
            impl MsdDispatch for $struct_name {
                fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
                    $(
                        explore_all!(pattern, ctx, self.$to_explore);
                    )*
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! explore_all {
    ( $pattern:expr, $ctx:expr, $( $explorable:expr ),+ ) => {
        $(
            for x in $explorable.iter() {
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
    AssignExpr: { rhs, lhs },
    InitListExpr: { exprs },
    SwitchExpr: { cases },
    Block: { nodes }
);

// Prophet Types
impl MsdDispatch for ClassOrInterfaceComponent {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
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

impl MsdDispatch for MethodComponent {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
        }

        // Visit other nodes
        if let Some(block) = &self.body {
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

impl MsdDispatch for MethodParamComponent {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
        }

        // Visit other nodes
        if let Some(annotations) = &self.annotation {
            explore_all!(pattern, ctx, annotations);
        }
    }
}

impl MsdDispatch for FieldComponent {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
        }

        // Visit other nodes
        explore_all!(pattern, ctx, self.annotations);
    }
}

impl MsdDispatch for DeclStmt {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
        }

        // Visit other nodes
        explore_all!(
            pattern,
            ctx,
            self.variables,
            self.expressions.iter().flat_map(|e| e).collect::<Vec<_>>()
        );
    }
}

impl MsdDispatch for VarDecl {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
        }

        // Visit other nodes
        explore_all!(pattern, ctx, self.annotation);
    }
}

impl MsdDispatch for CallExpr {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
        }

        // Visit other nodes
        self.name.as_ref().explore(pattern, ctx);
        explore_all!(pattern, ctx, self.args);
    }
}

impl MsdDispatch for AnnotationComponent {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        // Check if this node needs parsed
        if pattern.matches(self) {
            // self.parse(pattern, ctx);
        }

        // Visit other nodes
        // TODO visit key_value_pairs?
        // explore_all!(pattern, ctx, self.key_value_pairs);
    }
}

// AST Types

impl MsdDispatch for IfStmt {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        self.body.explore(pattern, ctx);
        if let Some(else_body) = &self.else_body {
            else_body.explore(pattern, ctx);
        }
    }
}

impl MsdDispatch for ForStmt {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        explore_all!(pattern, ctx, self.init);
        if let Some(condition) = &self.condition {
            condition.explore(pattern, ctx);
        }
        explore_all!(pattern, ctx, self.post);
        self.body.explore(pattern, ctx)
    }
}
impl MsdDispatch for ForRangeStmt {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        if let Some(iter) = &self.iterator {
            iter.explore(pattern, ctx);
        }
        self.body.explore(pattern, ctx)
    }
}

impl MsdDispatch for TryCatchStmt {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        self.try_body.explore(pattern, ctx);
        if let Some(finally_body) = &self.finally_body {
            finally_body.explore(pattern, ctx);
        }
    }
}

impl MsdDispatch for ReturnStmt {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext) {
        if let Some(expr) = &self.expr {
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
        let c: Expr = CallExpr::new(Box::new(Ident::new("".into()).into()), vec![]).into();
        eprintln!("hello?");
        c.explore(
            &NodePattern::new(NodeType::CallExpr, "".into(), vec![], "".into(), true),
            &mut ParserContext::default(),
        )
    }
}
