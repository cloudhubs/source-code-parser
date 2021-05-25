use crate::parse::AST;
use crate::prophet::*;
/// Handles parsing of modifiers in the Java AST. Modifiers contains the visibility,
/// final, and static nature of the element the modifier is attached to.

/// Struct to hold return data from parse_modifiers/find_modifiers
#[derive(Debug)]
pub(crate) struct Modifier {
    pub accessor: AccessorType,
    pub is_const: bool,
    pub is_static: bool,
}
impl Modifier {
    pub(crate) fn new() -> Modifier {
        Modifier {
            accessor: AccessorType::Default,
            is_const: false,
            is_static: false,
        }
    }
}

// finds and parses modifiers attached to a given AST node
// note: JS does NOT have a "modifiers" node in the AST, so this function seeks out known modifiers directly off of the root node to examine
// e.g. it may take a "lexical_declaration" and search for crap on that
pub(crate) fn parse_modifiers(ast: &AST, path: &str, package: &str) -> Modifier { 
    // TODO: everything
    Modifier {
        accessor: AccessorType::Default,
        is_const: false,
        is_static: false
    }
}