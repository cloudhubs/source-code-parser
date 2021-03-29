use super::CommunicationReplacer;
use crate::comm_repl_default_impl;
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};

comm_repl_default_impl!(
    BinaryExpr, UnaryExpr, ParenExpr, DotExpr, IncDecExpr, LogExpr, Ident, Literal, IndexExpr
);

impl CommunicationReplacer for AssignExpr {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.rhs.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call(modules, module, class, method)
            {
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacer for CallExpr {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        // TODO: convert call expressions that are REST or RPC calls.

        // Get name -- could be Ident, or DotExpr where the left hand is the identifier.
        // For REST and RPC calls, it should be a DotExpr most likely since it will be using
        // some kind of Client object to actually make the calls.

        // Use the identifier from the DotExpr and look for a declaration with that same name.
        // This could be in the module itself (unlikely???), in the class definition's fields,
        // or as a local variable defined in your current function.

        // Once we have the declaration we can try to use the type information to guess if it
        // is an RPC or REST call or a normal call expression.

        None
    }
}

impl CommunicationReplacer for InitListExpr {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.exprs.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call(modules, module, class, method)
            {
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacer for LambdaExpr {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}
