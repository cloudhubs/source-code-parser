use crate::ast::NodeLanguage;
use crate::ressa::explorer::RessaNodeExplorer;
use crate::{Language, ModuleComponent};
use bitmaps::Bitmap;
use by_address::ByAddress;
use derive_new::new;
use std::collections::HashMap;
use std::ops::BitOrAssign;

use super::NodePattern;

/// Single indexable reference
pub type IndexableEntry<'a> = &'a dyn Indexable;

/// Structure used to index the LAAST
#[derive(new)]
pub struct LaastIndex<'a> {
    /// Index of language to root LAAST node (largest subtrees containing a given language)
    language_index: HashMap<Language, Vec<IndexableEntry<'a>>>,

    /// Reverse index from AST to language set, adding knowledge of subnode languages to a LAAST node
    ast_languages: HashMap<ByAddress<IndexableEntry<'a>>, LanguageSet>,
}
impl<'a> LaastIndex<'a> {
    /// Get indexed node for a language
    pub fn get_roots(&self, language: &Language) -> Option<&Vec<IndexableEntry<'a>>> {
        self.language_index.get(language)
    }

    /// Retrieve the language of a subtree
    pub fn language_in_subtree(&self, language: &Language, subtree: &IndexableEntry<'a>) -> bool {
        match self.ast_languages.get(&ByAddress(*subtree)) {
            Some(descendent_languages) => langset_contains(descendent_languages, language),
            None => panic!("Unknown subtree provided"),
        }
    }

    /// Indexes the given node, if its language is indexed on
    fn add_if_valid(&mut self, node: IndexableEntry<'a>) {
        if let Some(index) = self.language_index.get_mut(&node.get_language()) {
            index.push(node);
        }
    }
}

/// Descriptive alias. Also, centralizes size; may need to expand in the future.
pub type LanguageSet = Bitmap<32>;

/// Retrieve whether a language is recorded
pub fn langset_contains(langset: &LanguageSet, lang: &Language) -> bool {
    langset.get(Language::get_index(&lang))
}

/// Record a language to the set
pub fn langset_set(langset: &mut LanguageSet, lang: &Language) -> bool {
    langset.set(Language::get_index(lang), true)
}

/// Compute the languages to index over
pub fn compute_index_languages<'a>(
    patterns: &Vec<NodePattern>,
    nodes: &Vec<IndexableEntry<'a>>,
) -> LaastIndex<'a> {
    // Compute all languages to index on
    let mut indices = HashMap::new();
    for pattern in patterns.iter() {
        if let Some(lang) = pattern.language {
            if !indices.contains_key(&lang) {
                indices.insert(lang, vec![]);
            }
        }
    }

    // Generate indices
    let mut indices = LaastIndex::new(indices, HashMap::new());
    for node in nodes.iter() {
        index(Language::Unknown, *node, LanguageSet::new(), &mut indices);
    }

    indices
}

/// Run the indexing procedure over the given node
pub fn index<'a>(
    mut current_lang: Language,
    current: IndexableEntry<'a>,
    mut curr_langs: LanguageSet,
    indices: &mut LaastIndex<'a>,
) -> LanguageSet {
    // If language changed, update data
    let new_lang = current.get_language();
    if new_lang != current_lang {
        // If it's a new language in this subtree, try to create an index entry
        if !langset_contains(&curr_langs, &new_lang) {
            indices.add_if_valid(current);
        }

        // Update current language data
        current_lang = new_lang;
        langset_set(&mut curr_langs, &current_lang);
    }

    // Visit decendents and retrieve descendent languages
    for node in current.get_children() {
        curr_langs.bitor_assign(index(current_lang, node, curr_langs.clone(), indices));
    }
    curr_langs
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
