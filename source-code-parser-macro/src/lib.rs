use std::str::FromStr;

use darling::ToTokens;
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, GenericArgument, Ident,
    Meta, PathArguments, PathSegment, Type,
};

#[proc_macro_derive(ChildFields)]
pub fn derive_child_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let item_name = input.ident;
    let fn_impl = match &input.data {
        Data::Struct(r#struct) => {
            let fields = get_indexable_fields(r#struct);
            quote! {}
        }
        Data::Enum(r#enum) => {
            quote! {}
        }
        _ => unimplemented!(),
    };
    let out = quote! {
        impl $crate::ressa::index::ChildFields for #item_name {
            fn get_fields(&self) -> std::vec::Vec<std::vec::Vec<&dyn $crate::ressa::index::Indexable>> {
                #fn_impl
            }
        }
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
Indexable -> vec![&self.#field]
Vec<Indexable> -> self.field.iter().collect::<std::vec::Vec<_>>()
Option<Indexable> -> self.field.map_or_else(|| std::vec::vec![], |x| std::vec::vec![x])
Vec<Option<Indexable>> -> self.field.iter().map(|x| #option_code).flat_map().collect()
Option<Vec<Indexable>> -> self.field.map_or_else(|| std::vec::vec![], |x| #vec_code)
Option<Vec<Option<Indexable>>> -> self.field.map_or_else(|| std::vec::vec![], |x| x.iter().map(|x| x.map_or_else(|| std::vec::vec![], |x| std::vec::vec![x])).flat_map().collect())
#option_code(#vec_code(#option_code(&self.#field)))
Node -> Option
Option -> Vec
Vec -> Option
I think I should reverse the iterator for the node types so we go inward to outward and then I can use the tuple window context correctly
Box<Indexable> -> &*self.#field
Option<Box<Indexable> -> #option_code(&*self.#field)
*/
fn index_field(ident: &Ident, ty: &Type) -> Option<TokenStream> {
    let type_path = match_or!(Type::Path(type_path), type_path, ty)?;
    let path_segment = match_or!(Some(seg), seg, type_path.path.segments.first())?;

    let ndx_types = get_indexable_field(path_segment)?;

    // This is going to be complicated.

    for (
        ndx, (first_ndx_type, second_ndx_type)
    ) in ndx_types.iter().tuple_windows::<(_, _)>().enumerate() {
        use IndexableType as IType;
        if ndx == 0 {
            match first_ndx_type {
                IType::Option => {},
                IType::Box => {},
                IType::Vec => {},
                IType::Node => {},
            }

            continue;
        }

        
        match (first_ndx_type, second_ndx_type) {
            // Option first
            (IType::Option, IType::Option) => indexing_code = quote! {
                
            },
            (IType::Option, IType::Box) => {},
            (IType::Option, IType::Vec) => {},
            (IType::Option, IType::Node) => {},
            // Box first
            (IType::Box, IType::Option) => {},
            (IType::Box, IType::Box) => {},
            (IType::Box, IType::Vec) => {},
            (IType::Box, IType::Node) => {},
            // Vec first
            (IType::Vec, IType::Option) => {},
            (IType::Vec, IType::Box) => {},
            (IType::Vec, IType::Vec) => {},
            (IType::Vec, IType::Node) => {},
            // This variant is unreachable because our nodes do not have
            // generic type parameters
            (IType::Node, _) => unreachable!()
            
            // IType::Vec => {
            //     indexing_code = quote! {
            //         #indexing_code
            //     };
            // }
            // IType::Option => {
            //     indexing_code = quote! {
            //         #indexing_code.map_or_else(|| vec![], |x| vec![x]);
            //     }
            // }
            // IType::Box => {
            //     indexing_code = quote! {
            //         &*#indexing_code
            //     }
            // }
            // IType::Owned => indexing_code = quote! { &#indexing_code },
        }
    }

    Some(quote! {})
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
