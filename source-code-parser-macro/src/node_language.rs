use crate::util;
use proc_macro2::TokenStream; //Span,
use quote::quote; //format_ident,
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument, Ident,
    PathArguments, PathSegment, Type,
};

pub fn expand_derive(input: DeriveInput) -> TokenStream {
    let item_name = input.ident;
    let fn_impl = util::get_impl(
        &item_name,
        &input.data,
        get_struct_impl,
        |ident| quote! { #ident.get_language() },
    );
    quote! {
        impl source_code_parser::ast::NodeLanguage for #item_name {
            fn get_language(&self) -> source_code_parser::Language {
                #fn_impl
            }
        }
    }
}

fn get_struct_impl(r#struct: &DataStruct) -> TokenStream {
    let (field_ident, _) = util::get_struct_fields(r#struct)
        .into_iter()
        .find(|(_, field_type)| is_language_field(field_type).is_some())
        .unwrap(); // TODO throw better compile errors
    quote! { self.#field_ident }
}

fn is_language_field(ty: &Type) -> Option<()> {
    let type_path = match_or!(Type::Path(type_path), type_path, ty)?;
    let path_segment = match_or!(Some(seg), seg, type_path.path.segments.first())?;
    if path_segment.ident.to_string() == "Language" {
        Some(())
    } else {
        None
    }
}
