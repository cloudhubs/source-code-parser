mod node_pattern;

pub use node_pattern::*;

mod explorer;
pub use explorer::*;

mod context;
pub use context::*;

mod pattern_parser;
pub use pattern_parser::*;

mod callback;
pub use callback::*;

pub mod index;
pub use index::*;

use crate::{ast::NodeLanguage, ModuleComponent};

/// Run the user-defined parsers, in the order they were defined, on our AST
pub fn run_ressa_parse(ast: &mut Vec<ModuleComponent>, ressas: Vec<NodePattern>) -> RessaResult {
    // Add all inferred languages (ReSSA languages which are not specified and assumed to be the same as the parent's)
    let ressas = ressas
        .into_iter()
        .map(|ressa| {
            let lang = ressa.get_language();
            ressa.cascade_language(&lang)
        })
        .collect();

    // Index the AST
    let project_index = compute_index_languages(
        &ressas,
        &ast.iter().map(|module| module as &dyn Indexable).collect(),
    );

    // Explore
    let mut ctx = ParserContext::default();
    for mut ressa in ressas.into_iter() {
        if let Some(roots) = project_index.get_roots(&ressa.get_language()) {
            for module in roots.iter() {
                println!("Root {:#?}", module.get_language());
                module.explore(&mut ressa, &mut ctx, &project_index);
            }
        }
    }

    // Clean and return context
    ctx.into()
}
