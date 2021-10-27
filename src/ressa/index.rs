use crate::ast::{Expr, Node, NodeLanguage, Stmt};
use crate::ressa::explorer::RessaNodeExplorer;
use crate::{Language, ModuleComponent};
use bitmaps::Bitmap;
use std::collections::HashMap;
use std::ops::BitOrAssign;

use super::NodePattern;

pub type SingleLanguageIndex<'a> = HashMap<Language, Vec<&'a dyn Indexable>>;
pub type LaastIndex<'a> = HashMap<LanguageSet, SingleLanguageIndex<'a>>;
pub type LanguageSet = Bitmap<512>;

/// Compute the languages to index over
pub fn compute_index_languages<'a>(patterns: &Vec<NodePattern>) -> LaastIndex<'a> {
    let mut indices = HashMap::new();
    for pattern in patterns.iter() {
        recursively_compute_lang(pattern, LanguageSet::new(), &mut indices);
    }
    indices
}

fn recursively_compute_lang<'a>(
    node: &dyn Indexable,
    curr_langs: LanguageSet,
    indices: &mut LaastIndex<'a>,
) -> LanguageSet {
    // If this node's language hasn't been found yet, record it and prepare to create an index entry
    let lang = node.get_language();
    let ndx = Language::get_index(&lang);
    let add_entry = if node.get_language() != Language::Unknown && !curr_langs.get(ndx) {
        curr_langs.set(ndx, true);
        true
    } else {
        false
    };

    // Compute languages of all child nodes
    for child in node.get_children().iter() {
        curr_langs.bitor_assign(recursively_compute_lang(node, curr_langs, indices));
    }

    // If a new index is needed, record before returning
    if add_entry {
        if let Some(map) = indices.get(&curr_langs) {
            if !map.contains_key(&lang) {
                map.insert(lang, vec![]);
            }
        } else {
            indices.insert(curr_langs, HashMap::new());
        }
    }
    curr_langs
}

/// Run the indexing procedure over the given
pub fn index<'a>(
    mut current_lang: Language,
    current: &'a dyn Indexable, // Please don't touch this
    indices: &mut LaastIndex<'a>,
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

// ========== Module component implementations (so indexable)

impl NodeLanguage for NodePattern {
    fn get_language(&self) -> Language {}
}

impl NodeLanguage for ModuleComponent {
    fn get_language(&self) -> Language {
        self.get_language()
    }
}

/// TODO delete when Jacob's macro is done
impl ChildFields for ModuleComponent {
    fn get_fields(&self) -> Vec<Vec<&dyn Indexable>> {
        todo!("Merge with Jacob's macro")
    }
}

// ========== Old, currently unused implementations ==========

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
