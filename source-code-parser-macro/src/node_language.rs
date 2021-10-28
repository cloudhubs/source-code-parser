use darling::ToTokens;
// use itertools::Itertools;
use crate::util;
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

pub fn expand_derive(input: DeriveInput) -> TokenStream {
    todo!()
}
