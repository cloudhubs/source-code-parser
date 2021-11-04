pub mod parse;
pub use parse::*;

pub mod prophet;
pub use prophet::*;

pub mod lang;
pub use lang::*;

pub mod ast;

pub mod communication;

pub mod ressa;

// Wow I hate this and it is terrible but it works for fixing
// our crate name resolution error with regards to using our own
// proc macros
extern crate self as source_code_parser;
