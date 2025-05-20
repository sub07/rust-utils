use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

pub fn derive(enum_ident: Ident, enum_data: DataEnum) -> TokenStream {
    let variants = enum_data.variants.iter().collect::<Vec<_>>();
    let variants_str = variants.iter().map(|v| v.ident.to_string()).collect_vec();
    let variants_ident = variants.iter().map(|v| &v.ident).collect_vec();
    let variants_destruct = variants
        .iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(_) => quote! { { .. } },
            syn::Fields::Unnamed(fields_unnamed) => {
                let fields = fields_unnamed
                    .unnamed
                    .iter()
                    .map(|_| quote! {_})
                    .collect_vec();
                quote! {
                    (#(#fields,)*)
                }
            }
            syn::Fields::Unit => TokenStream::new(),
        })
        .collect_vec();
    let const_token = if variants_destruct.iter().any(|t| !t.is_empty()) {
        quote! {const}
    } else {
        TokenStream::new()
    };
    let t = quote! {
        impl #enum_ident {
            pub #const_token fn as_str(&self) -> &'static str {
                match self {
                    #(
                        #enum_ident::#variants_ident #variants_destruct => #variants_str
                    ),*
                }
            }
        }
    };
    println!("{t}");
    t
}
