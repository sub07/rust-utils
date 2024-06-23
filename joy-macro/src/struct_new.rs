use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, Generics, Ident};

fn extract_generic_types(generics: &Generics) -> TokenStream {
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
