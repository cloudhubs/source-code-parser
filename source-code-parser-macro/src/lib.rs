// use std::{ops::Index, str::FromStr};

use darling::ToTokens;
// use itertools::Itertools;
use proc_macro2::TokenStream; //Span,
use quote::quote; //format_ident,
use syn::{
    parse_macro_input,
    Data,
    DataStruct,
    DeriveInput,
    Fields,
    GenericArgument,
    Ident,
    PathArguments,
    PathSegment,
    Type, //Attribute, Meta
};

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
