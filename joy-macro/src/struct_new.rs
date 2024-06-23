use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Generics, Ident};

use crate::utils::extract_generic_types;

pub fn derive(generics: Generics, ident: Ident, data: DataStruct) -> TokenStream {
    let field_names = data
        .fields
        .iter()
        .map(|f| f.ident.as_ref().expect("Doesn't work on tuple structs"))
        .collect::<Vec<_>>();
    let field_types = data.fields.iter().map(|f| &f.ty);
    let generic_types = extract_generic_types(&generics);
    let where_clause = generics.where_clause.as_ref();

    quote! {
        impl #generics #ident #generic_types #where_clause {
            pub fn new(#(#field_names: #field_types),*) -> #ident #generic_types {
                #ident {
                    #(
                        #field_names
                    ),*
                }
            }
        }
    }
}
