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

use crate::ModuleComponent;

/// Run the user-defined parsers, in the order they were defined, on our AST
pub fn run_msd_parse(ast: &mut Vec<ModuleComponent>, msds: Vec<NodePattern>) -> ContextData {
    let mut ctx = ParserContext::default();

    // Explore
    for mut msd in msds.into_iter() {
        for module in ast.iter_mut() {
            module.explore(&mut msd, &mut ctx);
        }
    }

    // Clean and return context
    ctx.into()
}
