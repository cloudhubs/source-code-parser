use super::CommunicationReplacer;
use crate::comm_repl_default_impl;
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};

comm_repl_default_impl!(ImportStmt, BreakStmt, ContinueStmt, ThrowStmt);

impl CommunicationReplacer for DeclStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.expressions.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call(modules, module, class, method)
            {
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacer for ExprStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.expr
            .replace_communication_call(modules, module, class, method)
    }
}

impl CommunicationReplacer for IfStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body
            .replace_communication_call(modules, module, class, method);
        if let Some(else_body) = self.else_body.as_mut() {
            else_body.replace_communication_call(modules, module, class, method);
        }
        None
    }
}

impl CommunicationReplacer for ForStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.init.iter_mut().for_each(|stmt| {
            stmt.replace_communication_call(modules, module, class, method);
        });
        self.condition
            .as_mut()?
            .replace_communication_call(modules, module, class, method);
        self.post
            .as_mut()?
            .replace_communication_call(modules, module, class, method);
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}
impl CommunicationReplacer for ForRangeStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.iterator
            .as_mut()?
            .replace_communication_call(modules, module, class, method);
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}

impl CommunicationReplacer for WhileStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.condition
            .replace_communication_call(modules, module, class, method);
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}
impl CommunicationReplacer for DoWhileStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.condition
            .replace_communication_call(modules, module, class, method);
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}

impl CommunicationReplacer for SwitchStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for case in self.cases.iter_mut() {
            case.replace_communication_call(modules, module, class, method);
        }
        None
    }
}

impl CommunicationReplacer for CaseStmt {
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

impl CommunicationReplacer for TryCatchStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.try_body
            .replace_communication_call(modules, module, class, method);
        self.finally_body
            .as_mut()?
            .replace_communication_call(modules, module, class, method)
    }
}

impl CommunicationReplacer for CatchStmt {
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

impl CommunicationReplacer for ReturnStmt {
    fn replace_communication_call(
        &mut self,
        modules: &Vec<ModuleComponent>,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.expr
            .as_mut()?
            .replace_communication_call(modules, module, class, method)
    }
}
