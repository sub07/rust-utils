use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

pub fn derive(enum_ident: Ident, enum_data: DataEnum) -> TokenStream {
    let variants = enum_data.variants.iter().collect::<Vec<_>>();
    let variants_str = variants
        .iter()
        .map(|v| v.ident.to_string())
        .collect::<Vec<_>>();
    quote! {
        impl #enum_ident {
            pub const fn as_str(self) -> &'static str {
                match self {
                    #(
                        #enum_ident::#variants => #variants_str
                    ),*
                }
            }
        }
    }
}
