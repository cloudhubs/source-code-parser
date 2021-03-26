use super::{expr::CommunicationReplacerExpr, CommunicationReplacer};
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Stmt)]
pub trait CommunicationReplacerStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node>;
}

impl CommunicationReplacerStmt for DeclStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.expressions.iter_mut() {
            if let Some(Node::Expr(replacement)) =
                expr.replace_communication_call(module, class, method)
            {
                // TODO: make sure for other replacements that if it's just an Expr it becomes an ExprStmt
                // Maybe make the different traits return Option of their own type.
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacerStmt for ExprStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.expr.replace_communication_call(module, class, method)
    }
}

impl CommunicationReplacerStmt for IfStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body.replace_communication_call(module, class, method);
        if let Some(else_body) = self.else_body.as_mut() {
            else_body.replace_communication_call(module, class, method);
        }
        None
    }
}

impl CommunicationReplacerStmt for ForStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body.replace_communication_call(module, class, method)
    }
}
impl CommunicationReplacerStmt for ForRangeStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body.replace_communication_call(module, class, method)
    }
}

impl CommunicationReplacerStmt for WhileStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body.replace_communication_call(module, class, method)
    }
}
impl CommunicationReplacerStmt for DoWhileStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body.replace_communication_call(module, class, method)
    }
}
impl CommunicationReplacerStmt for ReturnStmt {
    fn replace_communication_call_stmt(
        &mut self,
        _module: &ModuleComponent,
        _class: Option<&ClassOrInterfaceComponent>,
        _method: &MethodComponent,
    ) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for SwitchStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for case in self.cases.iter_mut() {
            case.replace_communication_call_stmt(module, class, method);
        }
        None
    }
}
impl CommunicationReplacerStmt for CaseStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body.replace_communication_call(module, class, method)
    }
}
impl CommunicationReplacerStmt for ImportStmt {
    fn replace_communication_call_stmt(
        &mut self,
        _module: &ModuleComponent,
        _class: Option<&ClassOrInterfaceComponent>,
        _method: &MethodComponent,
    ) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for BreakStmt {
    fn replace_communication_call_stmt(
        &mut self,
        _module: &ModuleComponent,
        _class: Option<&ClassOrInterfaceComponent>,
        _method: &MethodComponent,
    ) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for ContinueStmt {
    fn replace_communication_call_stmt(
        &mut self,
        _module: &ModuleComponent,
        _class: Option<&ClassOrInterfaceComponent>,
        _method: &MethodComponent,
    ) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for ThrowStmt {
    fn replace_communication_call_stmt(
        &mut self,
        _module: &ModuleComponent,
        _class: Option<&ClassOrInterfaceComponent>,
        _method: &MethodComponent,
    ) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for TryCatchStmt {
    fn replace_communication_call_stmt(
        &mut self,
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.try_body
            .replace_communication_call(module, class, method)
    }
}
impl CommunicationReplacerStmt for CatchStmt {
    fn replace_communication_call_stmt(
        &mut self,
        _module: &ModuleComponent,
        _class: Option<&ClassOrInterfaceComponent>,
        _method: &MethodComponent,
    ) -> Option<Node> {
        None
    }
}
