use itertools::{izip, Itertools};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, token::Paren, DataEnum, Expr, Ident,
    Token, Type,
};

#[derive(Debug)]
struct ValueAttributes {
    _value_ident: Ident,
    _par: Paren,
    values: Punctuated<ValueAttribute, Token![,]>,
}

impl Parse for ValueAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _value_ident: input.parse()?,
            _par: parenthesized!(content in input),
            values: Punctuated::parse_terminated(&content)?,
        })
    }
}

#[derive(Debug)]
struct ValueAttribute {
    ident: Ident,
    _colon: Token![:],
    ty: Type,
    _eq: Token![=],
    expr: Expr,
}

impl Parse for ValueAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _colon: input.parse()?,
            ty: input.parse()?,
            _eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}

fn transpose_2d_vec<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn derive(enum_ident: Ident, enum_data: DataEnum) -> TokenStream {
    if enum_data.variants.is_empty() {
        return quote! {};
    }

    let variants = enum_data.variants.iter().collect_vec();
    let variants_idents = variants.iter().map(|v| v.ident.clone()).collect_vec();

    let value_attributes = variants
        .iter()
        .map(|v| {
            v.attrs
                .iter()
                .find_map(|attribute| {
                    syn::parse2::<ValueAttributes>(attribute.meta.to_token_stream()).ok()
                })
                .expect("No valid value attribute")
        })
        .collect_vec();

    let value_idents = value_attributes[0]
        .values
        .iter()
        .map(|value| value.ident.clone())
        .collect_vec();
    let value_types = value_attributes[0]
        .values
        .iter()
        .map(|value| value.ty.clone())
        .collect_vec();

    let value_values = value_attributes
        .iter()
        .map(|values| {
            values
                .values
                .iter()
                .map(|value| value.expr.clone())
                .collect_vec()
        })
        .collect_vec();
    let value_values = transpose_2d_vec(value_values);

    let value_fns = izip!(value_idents, value_types, value_values)
        .map(|(ident, ty, expr)| {
            quote! {
                pub const fn #ident(&self) -> #ty {
                    match self {
                        #(
                            #enum_ident::#variants_idents => #expr
                        ),*
                    }
                }
            }
        })
        .collect_vec();

    quote! {
        impl #enum_ident {
            #(
                #value_fns
            )*
        }
    }
}
