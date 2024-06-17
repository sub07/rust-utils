use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataEnum, DataStruct, DeriveInput, GenericParam, Generics, Type, TypePath};

pub fn get_enum_data(input: &DeriveInput) -> &DataEnum {
    if let Data::Enum(data_enum) = &input.data {
        data_enum
    } else {
        panic!("Must be applied on enum")
    }
}

pub fn get_struct_data(input: &DeriveInput) -> &DataStruct {
    if let Data::Struct(data_struct) = &input.data {
        data_struct
    } else {
        panic!("Must be applied on struct")
    }
}

pub fn is_struct_tuple(struct_data: &DataStruct) -> bool {
    !struct_data.fields.is_empty() && struct_data.fields.iter().next().unwrap().ident.is_none()
}

pub fn all_equals<T: Eq>(idents: &[T]) -> bool {
    let first = idents.iter().next();
    idents
        .iter()
        .fold(first, |acc, ident| {
            acc.and_then(|acc| if acc == ident { Some(acc) } else { None })
        })
        .is_some()
}

pub fn is_type_generic(generics: &Generics, ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = &ty {
        generics
            .type_params()
            .map(|tp| &tp.ident)
            .any(|g| path.is_ident(g))
    } else {
        false
    }
}

pub fn get_generics_types_from_declared(declared_generics: &Generics) -> TokenStream {
    let generics_idents = declared_generics
        .params
        .iter()
        .map(|p| match p {
            GenericParam::Type(t) => t.ident.to_token_stream(),
            GenericParam::Lifetime(l) => l.to_token_stream(),
            GenericParam::Const(c) => c.ident.to_token_stream(),
        })
        .collect::<Vec<_>>();

    if generics_idents.is_empty() {
        quote!()
    } else {
        quote!(<#(#generics_idents),*>)
    }
}
