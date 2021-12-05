use crate::util;
use proc_macro2::TokenStream; //Span,
use quote::quote; //format_ident,
use syn::{DataStruct, DeriveInput, Ident, Type};

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

enum AccessorField {
    Language,
    ComponentInfo,
    ContainerComponent,
    Other,
}

impl ToString for AccessorField {
    fn to_string(&self) -> String {
        use AccessorField::*;
        match self {
            Language => "Language".to_string(),
            ComponentInfo => "ComponentInfo".to_string(),
            ContainerComponent => "ContainerComponent".to_string(),
            _ => "Other".to_string(),
        }
    }
}

impl From<&str> for AccessorField {
    fn from(s: &str) -> Self {
        use AccessorField::*;
        match s {
            "Language" => Language,
            "ComponentInfo" => ComponentInfo,
            "ContainerComponent" => ContainerComponent,
            _ => Other,
        }
    }
}

fn get_struct_impl(r#struct: &DataStruct, _struct_ident: &Ident) -> TokenStream {
    // let struct_ident = struct_ident.to_string();
    let fields = util::get_struct_fields(r#struct)
        .into_iter()
        .filter_map(|(field_ident, field_type)| {
            let field = can_access_language(field_type);
            field.map(|field| (field_ident, field))
        })
        .collect::<Vec<_>>();
    let (field_ident, field_variant) = fields.first().unwrap(); // TODO throw better compile errors
    use AccessorField::*;
    match field_variant {
        Language => quote! { self.#field_ident },
        ComponentInfo => quote! { self.#field_ident.language },
        ContainerComponent => quote! { self.#field_ident.get_language() },
        _ => unreachable!(),
    }
}

fn can_access_language(ty: &Type) -> Option<AccessorField> {
    let type_path = match_or!(Type::Path(type_path), type_path, ty)?;
    let path_segment = type_path.path.segments.first()?;
    use AccessorField::*;
    match AccessorField::from(&*path_segment.ident.to_string()) {
        Other => None,
        field => Some(field),
    }
}
