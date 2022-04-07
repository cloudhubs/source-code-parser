use source_code_parser::ressa::ExplorerContext;
use source_code_parser::ressa::LaastIndex;
use source_code_parser::ressa::NodePattern;
use source_code_parser::ressa::RessaNodeExplorer;
use source_code_parser::Language;
use source_code_parser_macro::ChildFields;
use source_code_parser_macro::NodeLanguage;

macro_rules! fake_ressa_explore_impl {
    ( $( $struct_name:ty ),+ ) => {
        $(
            impl RessaNodeExplorer for $struct_name {
                fn explore(&self, _pattern: &NodePattern, _ctx: &mut ExplorerContext, _index: &LaastIndex) -> Option<()> {
                    None
                }
            }
        )*
    };
}

fake_ressa_explore_impl!(CallExpr, IndexExpr, Ident, BinExpr, Expr, ExprStmt, Stmt, Node);

// Rust-analyzer likes to throw a "not yet implemented" compiler error here for some reason
// but the test passes since it compiles

#[derive(Debug, NodeLanguage, ChildFields)]
pub struct CallExpr {
    regular: IndexExpr,
    boxed: Box<Ident>,
    language: Language,
}

#[derive(Debug, NodeLanguage, ChildFields)]
pub struct IndexExpr {
    opt: Option<Ident>,
    vec_opt: Vec<Option<Ident>>,
    opt_vec: Option<Vec<ExprStmt>>,
    language_field: Language,
}
#[derive(Debug, NodeLanguage, ChildFields)]
pub struct Ident {
    opt_vec_opt: Option<Vec<Option<Node>>>,
    nested: BinExpr,

    language: Language,
}
#[derive(Debug, NodeLanguage, ChildFields)]
pub struct BinExpr {
    left: Box<Expr>,
    right: Box<Expr>,

    language: Language,
}

#[derive(Debug, NodeLanguage, ChildFields)]
pub struct EmptyExpr {
    language: Language,
}

#[derive(Debug, NodeLanguage, ChildFields)]
pub enum Expr {
    CallExpr(CallExpr),
    IndexExpr(IndexExpr),
    Ident(Ident),
    BinExpr(BinExpr),
}
#[derive(Debug, NodeLanguage, ChildFields)]
pub struct ExprStmt {
    vector: Vec<Expr>,

    language: Language,
}
#[derive(Debug, NodeLanguage, ChildFields)]
pub enum Stmt {
    ExprStmt(ExprStmt),
}

#[derive(Debug, NodeLanguage, ChildFields)]
pub enum Node {
    Expr(Expr),
    Stmt(Stmt),
}

fn main() {
    // Example:
    //
    // let x = vec![Some(Box::new(3))];
    // let y = vec![&x].into_iter() // initial step
    //     .flat_map(|x| x) // vec step
    //     .flat_map(|x: &Option<Box<i32>>| x.as_ref()) // option step
    //     .map(|x: &Box<i32>| &**x) // box step
    //     .map(|x| x as &dyn Display) // data step
    //     .collect::<Vec<&dyn Display>>() // wrap-up step;
}
