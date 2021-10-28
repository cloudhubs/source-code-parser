use darling::ToTokens;
// use itertools::Itertools;
use crate::util;
use proc_macro2::TokenStream; //Span,
use quote::quote; //format_ident,
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericArgument, Ident,
    PathArguments, PathSegment, Type,
};

pub fn expand_derive(input: DeriveInput) -> TokenStream {
    let item_name = input.ident;
    let fn_impl = get_impl(&input.data);
    quote! {
        impl source_code_parser::ressa::index::ChildFields for #item_name {
            fn get_fields(&self) -> std::vec::Vec<std::vec::Vec<&dyn source_code_parser::ressa::index::Indexable>> {
                #fn_impl
            }
        }
    }
}

fn get_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(r#struct) => get_struct_impl(r#struct),
        Data::Enum(r#enum) => get_enum_impl(r#enum),
        _ => unimplemented!(),
    }
}

fn get_enum_impl(r#enum: &DataEnum) -> TokenStream {
    let variants = util::get_enum_variants(r#enum).into_iter();
    // .map(|(variant_ident, variant_type| {
    //     // How do we know whether this inner type is a struct or another enum??? Can we??
    // });
    quote! {
        //match self {
            //
        //}
    }
}

fn get_struct_impl(r#struct: &DataStruct) -> TokenStream {
    let fields = util::get_struct_fields(r#struct)
        .into_iter()
        .map(|(field_ident, field_type)| index_field(field_ident, field_type))
        .flat_map(|field_code| field_code)
        .reduce(|left_field_code, right_field_code| quote! { #left_field_code, #right_field_code })
        .unwrap(); // TODO throw better compile errors
    quote! { std::vec![#fields] }
}

fn index_field(ident: &Ident, ty: &Type) -> Option<TokenStream> {
    let type_path = match_or!(Type::Path(type_path), type_path, ty)?;
    let path_segment = match_or!(Some(seg), seg, type_path.path.segments.first())?;

    let ndx_types = get_indexable_field(path_segment)?;

    match ndx_types.last() {
        Some(IndexableType::Node) => {}
        _ => return None,
    };

    let mut indexing_code = None;
    for ndx_type in ndx_types.into_iter().rev() {
        indexing_code = Some(index_field_inner(indexing_code, ident, ndx_type));
    }
    match indexing_code {
        Some(indexing_code) => Some(quote! { #indexing_code.collect::<std::vec::Vec<_>>() }),
        _ => None,
    }
}

fn index_field_inner(
    indexing_code: Option<TokenStream>,
    ident: &Ident,
    ty: IndexableType,
) -> TokenStream {
    let indexing_code = match indexing_code {
        Some(indexing_code) => indexing_code,
        None => quote! { self.#ident },
    };

    match ty {
        IndexableType::Option => quote! {
            #indexing_code.iter()
                .map(|#ident| #ident.as_ref().map_or_else(|| std::vec![], |#ident| std::vec![#ident]))
                .flat_map(|#ident| #ident)
        },
        IndexableType::Box => quote! {
            #indexing_code.iter()
                .map(|#ident| &***#ident)
        },
        IndexableType::Vec => quote! {
            #indexing_code.iter()
        },
        IndexableType::Node => {
            quote! { std::vec![&#indexing_code as &dyn source_code_parser::ressa::Indexable].into_iter() }
        }
    }
}

enum IndexableType {
    Option,
    Box,
    Vec,
    Node,
}

fn get_indexable_field(path_segment: &PathSegment) -> Option<Vec<IndexableType>> {
    match &*path_segment.ident.to_string() {
        "Vec" => select_wrapper(path_segment, Some(IndexableType::Vec)),
        "Option" => select_wrapper(path_segment, Some(IndexableType::Option)),
        "Box" => select_wrapper(path_segment, Some(IndexableType::Box)),
        "Language" | "bool" | "String" | "LogLevel" => None,
        // "Node" | "Expr" | "Stmt" | "Block" => {}
        _ => Some(vec![IndexableType::Node]),
    }
}

fn select_wrapper(
    path_segment: &PathSegment,
    ndx_type: Option<IndexableType>,
) -> Option<Vec<IndexableType>> {
    let inner_path_segment = util::get_inner_path_segment(path_segment)?;
    let mut fields = get_indexable_field(&inner_path_segment)?;
    if let Some(ndx_type) = ndx_type {
        fields.insert(0, ndx_type);
    }
    Some(fields)
}
