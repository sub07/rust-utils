use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils::extract_generic_types;

pub fn derive(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = input;

    let generic_types = extract_generic_types(&generics);
    let where_clause = generics.where_clause.as_ref();

    quote! {
        impl #generics std::fmt::Display for #ident #generic_types #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    }
}
