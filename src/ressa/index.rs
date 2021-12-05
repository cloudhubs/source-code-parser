use crate::ast::NodeLanguage;
use crate::ressa::explorer::RessaNodeExplorer;
use crate::Language;
use bitmaps::Bitmap;
use derive_new::new;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::BitOrAssign;

use super::NodePattern;

/// Single indexable reference
pub type IndexableEntry<'a> = &'a dyn Indexable;

/// Maps references to IndexableEntry to a hash table key
#[derive(Hash, PartialEq, Eq)]
pub struct IndexableKey {
    wrapped: String,
}
impl IndexableKey {
    fn new(entry: IndexableEntry<'_>) -> IndexableKey {
        IndexableKey {
            wrapped: format!("{:p}", entry),
        }
    }
}

/// Structure used to index the LAAST
#[derive(new)]
pub struct LaastIndex<'a> {
    /// Index of language to root LAAST node (largest subtrees containing a given language)
    language_index: HashMap<Language, Vec<IndexableEntry<'a>>>,

    /// Reverse index from AST to language set, adding knowledge of subnode languages to a LAAST node
    ast_languages: HashMap<IndexableKey, LanguageSet>,
}
impl<'a> LaastIndex<'a> {
    /// Get all nodes indexed for a language
    pub fn get_roots(&self, language: Language) -> Option<&Vec<IndexableEntry<'a>>> {
        self.language_index.get(&language)
    }

    /// Retrieve whether a given language exists in the subtree of the provided node
    pub fn language_in_subtree(&self, language: Language, subtree: IndexableEntry) -> bool {
        // Edge case: any language requested
        if language == Language::Unknown {
            return true;
        }

        // Handle main case
        // println!("Check index...");
        match self.ast_languages.get(&IndexableKey::new(subtree)) {
            Some(descendent_languages) => descendent_languages.contains(language),
            None => panic!("Unknown subtree provided"),
        }
    }

    /// Indexes the given node, if its language is allowed to be indexed on
    fn add_if_valid(&mut self, node: IndexableEntry<'a>) {
        // Index under specific language
        if let Some(index) = self.language_index.get_mut(&node.get_language()) {
            // println!("{:#?}", node.get_language());
            index.push(node);
        }

        // Index under catchall
        if let Some(index) = self.language_index.get_mut(&Language::Unknown) {
            // println!("Catchall");
            index.push(node);
        }
    }

    /// Indexes the given node, if its language is indexed on
    fn record_subtree_langs(&mut self, node: IndexableEntry<'a>, langs: LanguageSet) {
        self.ast_languages.insert(IndexableKey::new(node), langs);
    }
}

/// Describe a specialization of the Bitmap type for indexing Languages
#[derive(Clone, Copy)]
pub struct LanguageSet(Bitmap<32>);
impl LanguageSet {
    fn new() -> LanguageSet {
        LanguageSet(Bitmap::new())
    }

    /// Retrieve whether a language is recorded
    fn contains(&self, lang: Language) -> bool {
        self.0.get(lang.ordinal() as usize)
    }

    /// Record a language to the set
    fn set(&mut self, lang: Language) -> bool {
        self.0.set(lang.ordinal() as usize, true)
    }
}
impl BitOrAssign for LanguageSet {
    /// Perform the |= operation, setting all bits set in the provided LanguageSet.
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}

/// Compute the languages to index over
pub fn compute_index_languages<'a>(
    patterns: &[NodePattern],
    nodes: Vec<IndexableEntry<'a>>,
) -> LaastIndex<'a> {
    // Compute all languages to index on
    let mut indices = HashMap::new();
    for pattern in patterns.iter() {
        let language = pattern.get_language();
        indices.entry(language).or_insert_with(Vec::new);
    }

    // Generate indices
    let mut indices = LaastIndex::new(indices, HashMap::new());
    for node in nodes {
        // println!("Node: {:#?}", node.get_language());
        index(Language::Unknown, node, LanguageSet::new(), &mut indices);
    }

    indices
}

/// Run the indexing procedure over the given node
fn index<'a>(
    mut current_lang: Language,
    current: IndexableEntry<'a>,
    mut curr_langs: LanguageSet,
    indices: &mut LaastIndex<'a>,
) -> LanguageSet {
    // If language changed, update data
    let new_lang = current.get_language();
    if new_lang != current_lang {
        // If it's a new language in this subtree, try to create an index entry
        if !curr_langs.contains(new_lang) {
            // println!("Index {:#?}", new_lang);
            indices.add_if_valid(current);
        }

        // Update current language data
        curr_langs.set(new_lang);
        current_lang = new_lang;
    }

    // Visit decendents and retrieve descendent languages
    let mut my_set = LanguageSet::new();
    my_set.set(current_lang);
    for node in current.get_children() {
        my_set |= index(current_lang, node, curr_langs, indices);
    }

    // Record and return node's languages
    indices.record_subtree_langs(current, my_set);
    my_set
}

pub trait Indexable: RessaNodeExplorer + NodeLanguage + ChildFields + std::fmt::Debug {
    fn get_children(&self) -> Vec<&dyn Indexable>;
}

pub trait ChildFields {
    fn get_fields(&self) -> Vec<Vec<&dyn Indexable>>;
}

impl<T> Indexable for T
where
    T: RessaNodeExplorer + ChildFields + NodeLanguage + std::fmt::Debug,
{
    fn get_children(&self) -> Vec<&dyn Indexable> {
        self.get_fields().into_iter().flatten().collect()
    }
}
