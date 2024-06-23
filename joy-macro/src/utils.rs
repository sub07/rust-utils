use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Generics, Ident};

pub fn get_enum_data(input: DeriveInput) -> (Generics, Ident, DataEnum) {
    if let Data::Enum(data) = input.data {
        (input.generics, input.ident, data)
    } else {
        panic!("Must be applied on an enum")
    }
}

pub fn get_struct_data(input: DeriveInput) -> (Generics, Ident, DataStruct) {
    if let Data::Struct(data) = input.data {
        (input.generics, input.ident, data)
    } else {
        panic!("Must be applied on a struct")
    }
}

pub fn extract_generic_types(generics: &Generics) -> TokenStream {
    if generics.params.is_empty() {
        TokenStream::new()
    } else {
        let generic_types = generics.params.iter().map(|p| match p {
            syn::GenericParam::Lifetime(lt) => lt.lifetime.to_token_stream(),
            syn::GenericParam::Type(ty) => ty.ident.to_token_stream(),
            syn::GenericParam::Const(c) => c.ident.to_token_stream(),
        });
        quote! {
            <#(#generic_types),*>
        }
    }
}
