use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

pub fn derive(enum_ident: Ident, enum_data: DataEnum) -> TokenStream {
    let variants = enum_data.variants;
    let variant_iter = variants.iter().collect::<Vec<_>>();
    let variant_count = variants.len();
    let ordinals = 0..variant_count;
    quote! {
        impl #enum_ident {
            pub const SIZE: usize = #variant_count;
            pub const VARIANTS: [#enum_ident; #variant_count] = [#(#enum_ident::#variant_iter),*];

            pub const fn ordinal(self) -> usize {
                match self {
                    #(
                        #enum_ident::#variant_iter => #ordinals
                    ),*
                }
            }
        }
    }
}
