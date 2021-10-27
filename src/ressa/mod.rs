mod node_pattern;

use std::collections::HashMap;

pub use node_pattern::*;

mod explorer;
pub use explorer::*;

mod context;
pub use context::*;

mod pattern_parser;
pub use pattern_parser::*;

mod callback;
pub use callback::*;

mod index;
pub use index::*;

use crate::{Language, ModuleComponent};

/// Run the user-defined parsers, in the order they were defined, on our AST
pub fn run_ressa_parse(ast: &mut Vec<ModuleComponent>, ressas: Vec<NodePattern>) -> RessaResult {
    let mut ctx = ParserContext::default();
    let mut project_index = compute_index_languages(&ressas);
    for node in ast.iter() {
        index(Language::Unknown, node, &project_index);
    }

    // Explore
    for mut ressa in ressas.into_iter() {
        for module in ast.iter_mut() {
            module.explore(&mut ressa, &mut ctx);
        }
    }

    // Clean and return context
    ctx.into()
}
