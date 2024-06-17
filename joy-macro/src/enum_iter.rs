use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

pub fn derive(enum_ident: Ident, enum_data: DataEnum) -> TokenStream {
    let variants = enum_data.variants;
    let variant_iter = variants.iter();
    let variant_count = variants.len();
    quote! {
        impl #enum_ident {
            pub const SIZE: usize = #variant_count;
            pub const VARIANTS: [#enum_ident; #variant_count] = [#(#enum_ident::#variant_iter),*];
        }
    }
}
