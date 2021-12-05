use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};
use enum_dispatch::enum_dispatch;

mod expr;
mod stmt;

#[enum_dispatch(Node)]
#[enum_dispatch(Expr)]
#[enum_dispatch(Stmt)]
pub trait CommunicationReplacer {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node>;
}

#[macro_export]
macro_rules! comm_repl_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl CommunicationReplacer for $struct_name {
                fn replace_communication_call(
                    &mut self,
                    _modules: &[ModuleComponent],
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

impl CommunicationReplacer for Block {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for node in self.nodes.iter_mut() {
            if let Some(replacement) =
                node.replace_communication_call(modules, module, class, method)
            {
                *node = replacement;
            }
        }
        None
    }
}
