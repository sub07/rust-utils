use proc_macro::{Literal, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};

#[proc_macro_derive(EnumIter)]
pub fn enum_iter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Data::Enum(data_enum) = &input.data {
        let mut variants = quote! {};
        let enum_name = &input.ident;
        for variant in &data_enum.variants {
            let variant_ident = &variant.ident;
            variants.extend(quote! { #enum_name::#variant_ident, })
        }
        let array = quote! { [#variants] };
        let nb_variants = data_enum.variants.len();
        let impl_enum = quote! {
            impl #enum_name {
                pub const SIZE: usize = #enum_name::size();
                pub const VARIANTS: [#enum_name; #nb_variants] = #enum_name::variants();
                pub const fn size() -> usize { #nb_variants }
                pub const fn variants() -> [#enum_name; #nb_variants] {
                    #array
                }
                pub fn iter() -> std::slice::Iter<'static, #enum_name> {
                    #enum_name::VARIANTS.iter()
                }
            }
        };

        impl_enum.into()
    } else {
        panic!("EnumIter must be applied on enum only")
    }
}

#[proc_macro_derive(EnumValue, attributes(value))]
pub fn enum_variant_associate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let res = quote!();
    if let Data::Enum(data_enum) = &input.data {
        for variant in &data_enum.variants {
            let attr = variant.attrs.iter()
                .map(|a| &a.tokens).next()
                .expect(&format!("No value on variant {}", variant.ident))
                .clone();

        }
    }
    res.into()
}

struct Value {
    ident: Ident,
    equals: Token!(=),
    value: Literal,
    comma: Option<Token![,]>,
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Value{
            ident: input.parse()?,
            equals: input.parse()?,
            value: input.parse()?,
            comma: input.parse()?,
        })
    }
}