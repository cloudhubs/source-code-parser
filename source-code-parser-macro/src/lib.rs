use darling::ToTokens;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[macro_use]
mod util;
mod child_fields;
mod node_language;

#[proc_macro_derive(ChildFields)]
pub fn derive_child_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(child_fields::expand_derive, input)
}

#[proc_macro_derive(NodeLanguage)]
pub fn derive_node_language(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand(node_language::expand_derive, input)
}

fn expand<F>(expand_derive: F, input: proc_macro::TokenStream) -> proc_macro::TokenStream
where
    F: FnOnce(DeriveInput) -> TokenStream,
{
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive(input).to_token_stream().into()
}
