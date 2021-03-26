use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};
use enum_dispatch::enum_dispatch;

mod expr;
use expr::*;

mod stmt;
use stmt::*;

#[enum_dispatch(Node)]
pub trait CommunicationReplacer {
    fn replace_communication_call(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node>;
}

impl CommunicationReplacer for Expr {
    fn replace_communication_call(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.replace_communication_call_expr(module, class, method)
    }
}

impl CommunicationReplacer for Stmt {
    fn replace_communication_call(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.replace_communication_call_stmt(module, class, method)
    }
}
impl CommunicationReplacer for Block {
    fn replace_communication_call(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for node in self.nodes.iter_mut() {
            if let Some(replacement) = node.replace_communication_call(module, class, method) {
                *node = replacement;
            }
        }
        None
    }
}
