use proc_macro2::Literal;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use syn::{parenthesized, token, Ident, Token, Type};

pub fn get_nb_field(attributes_content: &[FieldAttributes]) -> usize {
    let values_length = attributes_content
        .iter()
        .map(|attr| attr.attribute_content.values.len())
        .collect::<Vec<_>>();

    if values_length.is_empty() {
        panic!("No field in value");
    }

    if values_length.iter().max() != values_length.iter().min() {
        // All values are equals
        panic!("Must have the same number of fields in all values");
    }

    let nb_field = *values_length.first().unwrap();
    nb_field
}

pub struct FieldAttributes {
    pub(crate) variant_ident: Ident,
    pub(crate) attribute_content: AttributeContent,
    pub(crate) nb_unnamed_fields: usize,
}

#[derive(Debug)]
pub struct AttributeContent {
    _paren_token: token::Paren,
    pub(crate) values: Punctuated<Value, Token![,]>,
}

impl Parse for AttributeContent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(AttributeContent {
            _paren_token: parenthesized!(content in input),
            values: content.parse_terminated(Value::parse)?,
        })
    }
}

#[derive(Debug)]
pub struct Value {
    pub(crate) ident: Ident,
    _colon: Token!(:),
    pub(crate) type_name: Type,
    _equals: Token!(=),
    pub(crate) neg_sign: Option<Token!(-)>,
    pub(crate) value: Literal,
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Value {
            ident: input.parse()?,
            _colon: input.parse()?,
            type_name: input.parse()?,
            _equals: input.parse()?,
            neg_sign: input.parse()?,
            value: input.parse()?,
        })
    }
}
