use super::{NodePattern, ParserContext};
use crate::ast::*;
use crate::prophet::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub enum ProphetNode {
    ClassOrInterfaceComponent(ClassOrInterfaceComponent),
    MethodComponent(MethodComponent),
    MethodParamComponent(MethodParamComponent),
    FieldComponent(FieldComponent),
    AnnotationComponent(AnnotationComponent),
}

#[enum_dispatch(Node)]
#[enum_dispatch(Expr)]
#[enum_dispatch(Stmt)]
#[enum_dispatch(ProphetNode)]
pub trait MsdDispatch {
    fn explore(&self, pattern: &NodePattern, ctx: &mut ParserContext);
}

#[macro_export]
macro_rules! msd_dispatch_default_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl MsdDispatch for $struct_name {
                fn explore(&self, _pattern: &NodePattern, _ctx: &mut ParserContext) {
                    eprintln!("{:#?}", self);
                }
            }
        )*
    };
}

msd_dispatch_default_impl!(
    BinaryExpr,
    UnaryExpr,
    ParenExpr,
    DotExpr,
    IncDecExpr,
    LogExpr,
    Ident,
    Literal,
    IndexExpr,
    EndpointCallExpr,
    ImportStmt,
    BreakStmt,
    ContinueStmt,
    ThrowStmt,
    LabelStmt,
    // shouldn't be default
    ClassOrInterfaceComponent,
    MethodComponent,
    MethodParamComponent,
    FieldComponent,
    AnnotationComponent,
    AssignExpr,
    InitListExpr,
    LambdaExpr,
    SwitchExpr,
    CaseExpr,
    DeclStmt,
    ExprStmt,
    IfStmt,
    ForStmt,
    ForRangeStmt,
    WhileStmt,
    DoWhileStmt,
    TryCatchStmt,
    CatchStmt,
    ReturnStmt,
    WithResourceStmt,
    Block,
    CallExpr
);

#[cfg(test)]
mod tests {
    use crate::msd::NodeType;

    use super::*;

    #[test]
    fn does_this_call() {
        let c: Expr = CallExpr::new(Box::new(Ident::new("".into()).into()), vec![]).into();
        eprintln!("hello?");
        c.explore(
            &NodePattern::new(NodeType::CallExpr, "".into(), vec![], "".into(), true),
            &mut ParserContext::default(),
        )
    }
}
