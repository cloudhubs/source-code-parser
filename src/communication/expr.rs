use super::CommunicationReplacer;
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Expr)]
pub trait CommunicationReplacerExpr {
    fn replace_communication_call_expr(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node>;
}

#[macro_export]
macro_rules! comm_repl_expr_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl CommunicationReplacerExpr for $struct_name {
                fn replace_communication_call_expr(
                    &mut self,
                    _module: &ModuleComponent,
                    _class: Option<&ClassOrInterfaceComponent>,
                    _method: &MethodComponent,
                ) -> Option<Node> {
                    None
                }
            }
        )*
    };
}

comm_repl_expr_impl!(
    BinaryExpr, UnaryExpr, ParenExpr, DotExpr, IncDecExpr, LogExpr, Ident, Literal, IndexExpr
);

impl CommunicationReplacerExpr for AssignExpr {
    fn replace_communication_call_expr(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.rhs.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call_expr(module, class, method)
            {
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacerExpr for CallExpr {
    fn replace_communication_call_expr(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        // TODO: convert call expressions that are REST or RPC calls.
        // I might need to pass more context information throughout these trait calls to know where
        // we might find the type information for a method call.
        None
    }
}

impl CommunicationReplacerExpr for InitListExpr {
    fn replace_communication_call_expr(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.exprs.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call_expr(module, class, method)
            {
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacerExpr for LambdaExpr {
    fn replace_communication_call_expr(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body.replace_communication_call(module, class, method)
    }
}
