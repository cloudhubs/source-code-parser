use super::CommunicationReplacer;
use crate::comm_repl_default_impl;
use crate::{ast::*, ClassOrInterfaceComponent};
use crate::{prophet::ModuleComponent, MethodComponent};

comm_repl_default_impl!(ImportStmt, BreakStmt, ContinueStmt, ThrowStmt, LabelStmt);

impl CommunicationReplacer for DeclStmt {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        for expr in self.expressions.iter_mut().flatten() {
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
        modules: &[ModuleComponent],
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
        modules: &[ModuleComponent],
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
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.init.iter_mut().for_each(|stmt| {
            stmt.replace_communication_call(modules, module, class, method);
        });
        if let Some(Node::Expr(replacement)) = self
            .condition
            .as_mut()?
            .replace_communication_call(modules, module, class, method)
        {
            *self.condition.as_mut()? = replacement;
        }
        self.post.iter_mut().for_each(|stmt| {
            stmt.replace_communication_call(modules, module, class, method);
        });
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}
impl CommunicationReplacer for ForRangeStmt {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        if let Some(Node::Expr(replacement)) = self
            .iterator
            .as_mut()?
            .replace_communication_call(modules, module, class, method)
        {
            *self.iterator.as_mut()? = replacement;
        }
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}

impl CommunicationReplacer for WhileStmt {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        if let Some(Node::Expr(replacement)) =
            (&mut self.condition).replace_communication_call(modules, module, class, method)
        {
            self.condition = replacement;
        };
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}
impl CommunicationReplacer for DoWhileStmt {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        if let Some(Node::Expr(replacement)) =
            (&mut self.condition).replace_communication_call(modules, module, class, method)
        {
            self.condition = replacement;
        }
        self.body
            .replace_communication_call(modules, module, class, method)
    }
}

impl CommunicationReplacer for TryCatchStmt {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
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
        modules: &[ModuleComponent],
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
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        if let Some(Node::Expr(replacement)) = self
            .expr
            .as_mut()?
            .replace_communication_call(modules, module, class, method)
        {
            *self.expr.as_mut()? = replacement;
        }
        None
    }
}

impl CommunicationReplacer for WithResourceStmt {
    fn replace_communication_call(
        &mut self,
        modules: &[ModuleComponent],
        module: &ModuleComponent,
        class: Option<&ClassOrInterfaceComponent>,
        method: &MethodComponent,
    ) -> Option<Node> {
        self.body
            .replace_communication_call(modules, module, class, method);
        self.resources
            .replace_communication_call(modules, module, class, method);
        None
    }
}
