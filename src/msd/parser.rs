use super::NodePattern;
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

// #[enum_dispatch(Node)]
// #[enum_dispatch(Expr)]
// #[enum_dispatch(Stmt)]
// #[enum_dispatch(ProphetNode)]
// pub trait MsdDispatch {
//     fn explore(&self, pattern: &NodePattern, ctx: ());
// }

// #[macro_export]
// macro_rules! msd_dispatch_default_impl {
//     ( $( $struct_name:ty ),+ ) => {
//         $(
//             impl MsdDispatch for $struct_name {
//                 fn explore(&self, pattern: &NodePattern, ctx: ()) {}
//             }
//         )*
//     };
// }

// msd_dispatch_default_impl!(
//     BinaryExpr,
//     UnaryExpr,
//     ParenExpr,
//     DotExpr,
//     IncDecExpr,
//     LogExpr,
//     Ident,
//     Literal,
//     IndexExpr,
//     EndpointCallExpr,
//     ImportStmt,
//     BreakStmt,
//     ContinueStmt,
//     ThrowStmt,
//     LabelStmt
// );
