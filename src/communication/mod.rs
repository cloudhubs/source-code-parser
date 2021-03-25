use crate::ast::*;
use crate::prophet::ModuleComponent;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Node)]
pub trait CommunicationReplacer {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node>;
}

#[enum_dispatch(Expr)]
pub trait CommunicationReplacerExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node>;
}

#[enum_dispatch(Stmt)]
pub trait CommunicationReplacerStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node>;
}

impl CommunicationReplacer for Expr {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.replace_communication_call_expr(modules)
    }
}

impl CommunicationReplacer for Stmt {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.replace_communication_call_stmt(modules)
    }
}
impl CommunicationReplacer for Block {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        for node in self.nodes.iter_mut() {
            if let Some(replacement) = node.replace_communication_call(modules) {
                *node = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacerStmt for DeclStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        for expr in self.expressions.iter_mut() {
            if let Some(Node::Expr(replacement)) = expr.replace_communication_call_expr(modules) {
                // TODO: make sure for other replacements that if it's just an Expr it becomes an ExprStmt
                // Maybe make the different traits return Option of their own type.
                *expr = replacement;
            }
        }
        None
    }
}

impl CommunicationReplacerStmt for ExprStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.expr.replace_communication_call_expr(modules)
    }
}

impl CommunicationReplacerStmt for IfStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.body.replace_communication_call(modules);
        if let Some(else_body) = self.else_body.as_mut() {
            else_body.replace_communication_call(modules);
        }
        None
    }
}

impl CommunicationReplacerStmt for ForStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.body.replace_communication_call(modules)
    }
}
impl CommunicationReplacerStmt for ForRangeStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.body.replace_communication_call(modules)
    }
}

impl CommunicationReplacerStmt for WhileStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.body.replace_communication_call(modules)
    }
}
impl CommunicationReplacerStmt for DoWhileStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.body.replace_communication_call(modules)
    }
}
impl CommunicationReplacerStmt for ReturnStmt {
    fn replace_communication_call_stmt(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for SwitchStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        for case in self.cases.iter_mut() {
            case.replace_communication_call_stmt(modules);
        }
        None
    }
}
impl CommunicationReplacerStmt for CaseStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.body.replace_communication_call(modules)
    }
}
impl CommunicationReplacerStmt for ImportStmt {
    fn replace_communication_call_stmt(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for BreakStmt {
    fn replace_communication_call_stmt(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for ContinueStmt {
    fn replace_communication_call_stmt(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for ThrowStmt {
    fn replace_communication_call_stmt(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerStmt for TryCatchStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.try_body.replace_communication_call(modules)
    }
}
impl CommunicationReplacerStmt for CatchStmt {
    fn replace_communication_call_stmt(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}

impl CommunicationReplacerExpr for AssignExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        for expr in self.rhs.iter_mut() {
            if let Some(Node::Expr(replacement)) = expr.replace_communication_call(modules) {
                *expr = replacement;
            }
        }
        None
    }
}
impl CommunicationReplacerExpr for BinaryExpr {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for UnaryExpr {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for CallExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        // TODO: convert call expressions that are REST or RPC calls.
        // I might need to pass more context information throughout these trait calls to know where
        // we might find the type information for a method call.
        None
    }
}
impl CommunicationReplacerExpr for IndexExpr {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for ParenExpr {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for DotExpr {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for IncDecExpr {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for InitListExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        for expr in self.exprs.iter_mut() {
            if let Some(Node::Expr(replacement)) = expr.replace_communication_call_expr(modules) {
                *expr = replacement;
            }
        }
        None
    }
}
impl CommunicationReplacerExpr for LogExpr {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for LambdaExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) -> Option<Node> {
        self.body.replace_communication_call(modules)
    }
}
impl CommunicationReplacerExpr for Ident {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
impl CommunicationReplacerExpr for Literal {
    fn replace_communication_call_expr(&mut self, _modules: &Vec<ModuleComponent>) -> Option<Node> {
        None
    }
}
