use crate::ast::{Block, Expr, Node, NodeLanguage, Stmt};
use crate::ressa::explorer::RessaNodeExplorer;
use crate::Language;
use std::collections::HashMap;

pub fn index<'a>(
    mut current_lang: Language,
    current: &'a dyn Indexable, // Please don't touch this
    indices: &mut HashMap<Language, Vec<&'a dyn Indexable>>,
) {
    let new_lang = current.get_language();
    if new_lang != current_lang {
        match indices.get_mut(&new_lang) {
            Some(nodes) => nodes.push(current),
            None => {
                indices.insert(current_lang, vec![current]);
            }
        }
        current_lang = new_lang;
    }
    current
        .get_children()
        .iter()
        .for_each(|node| index(current_lang, *node as &dyn Indexable, indices));
}

pub trait Indexable: RessaNodeExplorer + NodeLanguage + ChildFields {
    fn get_children(&self) -> Vec<&dyn Indexable>;
}

pub trait ChildFields {
    fn get_fields(&self) -> Vec<Vec<&dyn Indexable>>;
}

// impl ChildFields for Node {
//     fn get_fields(&self) -> Vec<Vec<&dyn Indexable>> {
//         vec![vec![&*self.child_one], self.child_two.iter().collect()]
//     }
// }

impl<T> Indexable for T
where
    T: RessaNodeExplorer + ChildFields + NodeLanguage,
{
    fn get_children(&self) -> Vec<&dyn Indexable> {
        self.get_fields()
            .into_iter()
            .flat_map(|field_children| field_children)
            .collect()
    }
}

// impl Indexable for Node {
//     fn get_children(&self) -> Vec<&dyn Indexable> {
//         match self {
//             Node::Block(block) => block.nodes.iter().collect(),
//             Node::Stmt(stmt) => stmt.get_children(),
//             Node::Expr(expr) => expr.get_children(),
//         }
//     }
// }

// impl Indexable for Stmt {
//     fn get_children(&self) -> Vec<&dyn Indexable> {
//         use Stmt::*;
//         match self {
//             DeclStmt(decl) => (&decl.variables)
//                 .iter()
//                 .map(|decl| decl as &dyn Indexable)
//                 .chain(decl.expressions.iter().flat_map(|expr| expr))
//                 .into(),
//             ExprStmt(expr) => expr.expr.get_children(),
//             IfStmt(if_stmt) => todo!(),
//             ForStmt(for_stmt) => todo!(),
//             ForRangeStmt(for_range_stmt) => todo!(),
//             WhileStmt(while_stmt) => todo!(),
//             DoWhileStmt(do_while_stmt) => todo!(),
//             ReturnStmt(return_stmt) => todo!(),
//             ImportStmt(import_stmt) => todo!(),
//             BreakStmt(break_stmt) => todo!(),
//             ContinueStmt(continue_stmt) => todo!(),
//             ThrowStmt(throw_stmt) => todo!(),
//             TryCatchStmt(try_catch_stmt) => todo!(),
//             CatchStmt(catch_stmt) => todo!(),
//             WithResourceStmt(with_rss_stmt) => todo!(),
//             LabelStmt(label_stmt) => todo!(),
//         }
//     }
// }

// impl Indexable for Expr {
//     fn get_children(&self) -> Vec<&dyn Indexable> {
//         use Expr::*;
//         match self {
//             AssignExpr(_) => todo!(),
//             BinaryExpr(_) => todo!(),
//             UnaryExpr(_) => todo!(),
//             CallExpr(_) => todo!(),
//             EndpointCallExpr(_) => todo!(),
//             IndexExpr(_) => todo!(),
//             ParenExpr(_) => todo!(),
//             DotExpr(_) => todo!(),
//             IncDecExpr(_) => todo!(),
//             InitListExpr(_) => todo!(),
//             LogExpr(_) => todo!(),
//             LambdaExpr(_) => todo!(),
//             Ident(_) => todo!(),
//             Literal(_) => todo!(),
//             SwitchExpr(_) => todo!(),
//         }
//     }
// }
