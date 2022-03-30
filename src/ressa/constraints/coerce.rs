use crate::{ast::*, prophet::*, ressa::Indexable};

pub trait IntoExpr<R> {
    fn get_expr<T>(&self) -> Option<&R>
    where
        T: Indexable;
}

macro_rules! impl_not_expr {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl IntoExpr<$struct_name> for $struct_name {
                fn get_expr<T>(&self) -> Option<&$struct_name> where T: Indexable {
                    None
                }
            }
        )*
    };
}

macro_rules! impl_expr {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl IntoExpr<$struct_name> for $struct_name {
                fn get_expr<T>(&self) -> Option<&$struct_name> where T: Indexable {
                    Some(self)
                }
            }
        )*
    };
}

impl_not_expr!(
    ImportStmt,
    BreakStmt,
    ContinueStmt,
    ThrowStmt,
    LabelStmt,
    ExprStmt,
    CatchStmt,
    WhileStmt,
    DoWhileStmt,
    WithResourceStmt,
    ModuleComponent,
    Block,
    IfStmt,
    ForStmt,
    ForRangeStmt,
    TryCatchStmt,
    ReturnStmt,
    ContainerComponent,
    Ident,
    Literal,
    ClassOrInterfaceComponent,
    MethodComponent,
    MethodParamComponent,
    FieldComponent,
    DeclStmt,
    VarDecl,
    AnnotationComponent,
    AnnotationValuePair
);

impl_expr!(
    IncDecExpr,
    LogExpr,
    IndexExpr,
    EndpointCallExpr,
    LambdaExpr,
    CaseExpr,
    BinaryExpr,
    UnaryExpr,
    ParenExpr,
    DotExpr,
    AssignExpr,
    InitListExpr,
    SwitchExpr,
    CallExpr
);
