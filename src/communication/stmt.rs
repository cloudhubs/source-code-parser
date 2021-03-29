use super::CommunicationReplacer;
use crate::comm_repl_default_impl;
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};

comm_repl_default_impl!(
    ReturnStmt,
    ImportStmt,
    BreakStmt,
    ContinueStmt,
    ThrowStmt,
    CatchStmt
);

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
                // TODO: make sure for other replacements that if it's just an Expr it becomes an ExprStmt
                // Maybe make the different traits return Option of their own type.
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
            .replace_communication_call(modules, module, class, method)
    }
}
