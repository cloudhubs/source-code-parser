use proc_macro2::TokenStream; //Span,
use quote::quote; //format_ident,
use syn::{
    Data, DataEnum, DataStruct, Fields, GenericArgument, Ident, PathArguments, PathSegment, Type,
};

macro_rules! match_or {
    ($pattern:pat, $name:ident, $thing:expr) => {
        if let $pattern = $thing {
            Some($name)
        } else {
            None
        }
    };
}

pub fn get_inner_path_segment(path_segment: &PathSegment) -> Option<PathSegment> {
    let args = match_or!(
        PathArguments::AngleBracketed(ref args),
        args,
        path_segment.arguments
    )?;
    let generic_arg = args.args.first()?;
    let type_path = match_or!(GenericArgument::Type(Type::Path(path)), path, generic_arg)?;

    Some(type_path.path.segments.first()?.clone())
}

pub fn get_struct_fields(r#struct: &DataStruct) -> Vec<(&Ident, &Type)> {
    match &r#struct.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().expect("You're named for a reason.");
                let ty = &field.ty;
                (ident, ty)
            })
            .collect(),
        _ => unimplemented!(),
    }
}

pub fn get_enum_variants(r#enum: &DataEnum) -> Vec<(&Ident, &Type)> {
    r#enum
        .variants
        .iter()
        .map(|variant| {
            let ty = match &variant.fields {
                Fields::Unnamed(fields) => fields.unnamed.first().map(|field| &field.ty),
                _ => unimplemented!(), // assuming tuple variants
            }
            .expect("There should be at least one unnamed field");
            (&variant.ident, ty)
        })
        .collect()
}

pub fn get_impl<S, E>(
    item_ident: &Ident,
    data: &Data,
    get_struct_impl: S,
    enum_action_impl: E,
) -> TokenStream
where
    S: Fn(&DataStruct, &Ident) -> TokenStream,
    E: Fn(&Ident) -> TokenStream,
{
    match data {
        Data::Struct(r#struct) => get_struct_impl(r#struct, item_ident),
        Data::Enum(r#enum) => get_enum_impl(item_ident, r#enum, enum_action_impl),
        _ => unimplemented!(),
    }
}

fn get_enum_impl<F>(ident: &Ident, r#enum: &DataEnum, action: F) -> TokenStream
where
    F: Fn(&Ident) -> TokenStream,
{
    let action_ident = Ident::new("x", ident.span());
    let variants = get_enum_variants(r#enum)
        .into_iter()
        .map(|(variant_ident, _)| {
            let action = action(&action_ident);
            quote! {
                #ident ::#variant_ident(#action_ident) => #action,
            }
        })
        .reduce(|variant_one, variant_two| {
            quote! {
                #variant_one
                #variant_two
            }
        })
        .unwrap(); // TODO better compile error
    quote! {
        match self {
            #variants
        }
    }
}
