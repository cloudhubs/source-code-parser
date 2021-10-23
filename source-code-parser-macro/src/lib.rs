use std::str::FromStr;

use darling::ToTokens;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, GenericArgument, Ident,
    Meta, PathArguments, PathSegment, Type,
};

#[proc_macro_derive(Indexable)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let item_name = input.ident;
    let out = match &input.data {
        Data::Struct(r#struct) => {
            quote! {
                impl $crate::ressa::index::Indexable for #item_name {

                }
            }
        }
        Data::Enum(r#enum) => {
            quote! {}
        }
        _ => unimplemented!(),
    };
    out.to_token_stream().into()
}

fn get_indexable_fields(r#struct: &DataStruct) -> Vec<(&Ident, &Type)> {
    match &r#struct.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().expect("You're named for a reason.");
                let ty = &field.ty;
                (ident, ty)
            })
            // .filter(|(_, ty)| {})
            .collect(),
        _ => unimplemented!(),
    }
}

macro_rules! match_or {
    ($pattern:pat, $name:ident, $thing:expr) => {
        if let $pattern = $thing {
            Some($name)
        } else {
            None
        }
    };
}

/*
Indexable
Vec<Indexable>
Option<Indexable>
Vec<Option<Indexable>>
Box<Indexable>
Option<Vec<Indexable>>
*/
fn index_field(ident: &Ident, ty: &Type) -> Option<TokenStream> {
    let type_path = match_or!(Type::Path(type_path), type_path, ty)?;
    let path_segment = match_or!(Some(seg), seg, type_path.path.segments.first())?;

    let ndx_types = get_indexable_field(path_segment)?;

    // This is going to be complicated.

    // let mut indexing_code = quote! { #ident };
    // for ndx_type in ndx_types {
    //     match ndx_type {
    //         IndexableType::Vec => {
    //             indexing_code = quote! {
    //                 #indexing_code
    //             };
    //         }
    //         IndexableType::Option => {
    //             indexing_code = quote! {
    //                 #indexing_code.map_or_else(|| vec![], |x| vec![x]);
    //             }
    //         }
    //     }
    // }

    Some(quote! {})
}

enum IndexableType {
    Option,
    Vec,
}

fn get_indexable_field(path_segment: &PathSegment) -> Option<Vec<IndexableType>> {
    match &*path_segment.ident.to_string() {
        "Vec" => select_wrapper(path_segment, Some(IndexableType::Vec)),
        "Option" => select_wrapper(path_segment, Some(IndexableType::Option)),
        "Box" => select_wrapper(path_segment, None),
        "Language" | "bool" | "String" | "LogLevel" => None,
        // "Node" | "Expr" | "Stmt" | "Block" => {}
        _ => Some(Default::default()),
    }
}

fn select_wrapper(
    path_segment: &PathSegment,
    ndx_type: Option<IndexableType>,
) -> Option<Vec<IndexableType>> {
    let inner_path_segment = get_inner_path_segment(path_segment)?;
    let mut fields = get_indexable_field(&inner_path_segment)?;
    if let Some(ndx_type) = ndx_type {
        fields.insert(0, ndx_type);
    }
    Some(fields)
}

fn get_inner_path_segment(path_segment: &PathSegment) -> Option<PathSegment> {
    let args = match_or!(
        PathArguments::AngleBracketed(ref args),
        args,
        path_segment.arguments
    )?;
    let generic_arg = args.args.first()?;
    let type_path = match_or!(GenericArgument::Type(Type::Path(path)), path, generic_arg)?;

    Some(type_path.path.segments.first()?.clone())
}
