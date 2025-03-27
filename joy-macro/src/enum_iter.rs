use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

pub fn derive(enum_ident: Ident, enum_data: DataEnum) -> TokenStream {
    let variants = enum_data.variants.iter().map(|v| &v.ident).collect_vec();
    let variant_count = variants.len();
    let ordinals = 0..variant_count;
    quote! {
        impl #enum_ident {
            pub const COUNT: usize = #variant_count;
            pub const VARIANTS: [#enum_ident; #variant_count] = [#(#enum_ident::#variants),*];

            pub const fn ordinal(self) -> usize {
                match self {
                    #(
                        #enum_ident::#variants => #ordinals
                    ),*
                }
            }
        }
    }
}
