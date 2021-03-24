use crate::ast::*;
use crate::prophet::ModuleComponent;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Node)]
pub trait CommunicationReplacer {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>);
}

#[enum_dispatch(Expr)]
pub trait CommunicationReplacerExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>);
}

#[enum_dispatch(Stmt)]
pub trait CommunicationReplacerStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>);
}

impl CommunicationReplacer for Expr {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>) {
        self.replace_communication_call_expr(modules);
    }
}

impl CommunicationReplacer for Stmt {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>) {
        self.replace_communication_call_stmt(modules);
    }
}
impl CommunicationReplacer for Block {
    fn replace_communication_call(&mut self, modules: &Vec<ModuleComponent>) {
        for node in self.nodes.iter_mut() {
            node.replace_communication_call(modules);
        }
    }
}

impl CommunicationReplacerStmt for DeclStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}

impl CommunicationReplacerStmt for ExprStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}

impl CommunicationReplacerStmt for IfStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}

impl CommunicationReplacerStmt for ForStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for ForRangeStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}

impl CommunicationReplacerStmt for WhileStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for DoWhileStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for ReturnStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for SwitchStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for CaseStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for ImportStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for BreakStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for ContinueStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for ThrowStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for TryCatchStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerStmt for CatchStmt {
    fn replace_communication_call_stmt(&mut self, modules: &Vec<ModuleComponent>) {}
}

impl CommunicationReplacerExpr for AssignExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for BinaryExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for UnaryExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for CallExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for IndexExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for ParenExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for DotExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for IncDecExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for InitListExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for LogExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for LambdaExpr {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for Ident {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
impl CommunicationReplacerExpr for Literal {
    fn replace_communication_call_expr(&mut self, modules: &Vec<ModuleComponent>) {}
}
