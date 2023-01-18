extern crate core;

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Ident, parenthesized, parse_macro_input, Token, token, TypePath};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

fn get_enum_data(input: &DeriveInput) -> &DataEnum {
    if let Data::Enum(data_enum) = &input.data {
        data_enum
    } else {
        panic!("Must be applied on enum")
    }
}

#[proc_macro_derive(EnumIter)]
pub fn enum_iter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = get_enum_data(&input);
    let mut variants = quote! {};
    let enum_name = &input.ident;
    for variant in &enum_data.variants {
        let variant_ident = &variant.ident;
        variants.extend(quote! { #enum_name::#variant_ident, })
    }
    let array = quote! { [#variants] };
    let nb_variants = enum_data.variants.len();
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
}

#[proc_macro_derive(EnumValue, attributes(value))]
pub fn enum_variant_associate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = get_enum_data(&input);
    let enum_name = &input.ident;
    let attributes_content =
        enum_data
            .variants.iter()
            .map(|v|
                FieldAttributes {
                    variant_ident: v.ident.clone(),
                    attribute_content: syn::parse2::<AttributeContent>(
                        v.attrs.iter().next()
                            .expect(&format!("Illegal variant without value attribute : {}", v.ident))
                            .tokens.clone()
                    ).expect("Invalid syntax in #[value(...)] : "),
                }
            ).collect::<Vec<_>>();

    let nb_field = get_nb_field(&attributes_content);

    for field_index in 0..nb_field {
        let values = attributes_content.iter()
            .map(|f|
                f.attribute_content
                    .values.iter()
                    .nth(field_index).unwrap()
            )
            .collect::<Vec<_>>();
        let idents = values.iter().map(|v| v.ident.to_string()).collect::<Vec<_>>();
        let types = values.iter()
            .map(|v|
                v.type_name.path
                    .get_ident().expect("Only single segments paths type are supported")
                    .to_string()
            )
            .collect::<Vec<_>>();

        if !all_equals(&idents) {
            panic!("Field n°{} names are not all equals : {idents:#?}", field_index + 1);
        }

        if !all_equals(&types) {
            panic!("Field n°{} types are not all equals : {types:#?}", field_index + 1);
        }
    }

    let mut functions = quote!();
    for field_index in 0..nb_field {
        let (field_name, type_name) = {
            let value = &attributes_content[field_index].attribute_content.values[field_index];
            (value.ident.clone(), value.type_name.clone())
        };

        let literals =
            attributes_content.iter().map(|attr| (attr.variant_ident.clone(), attr.attribute_content.values[field_index].value.clone())).collect::<Vec<_>>();

        let mut match_statement = quote!();

        for (ident, literal) in literals {
            match_statement.extend(quote! {
                #enum_name::#ident => #literal,
            });
        }

        functions.extend(quote! {
            pub const fn #field_name(&self) -> #type_name {
                match self {
                    #match_statement
                }
            }
        })
    }

    let impl_block = quote! {
        impl #enum_name {
            #functions
        }
    };

    impl_block.into()
}

fn all_equals<T: Eq>(idents: &[T]) -> bool {
    let first = idents.iter().next();
    idents.iter().fold(first, |acc, ident| {
        acc.and_then(|acc| if acc == ident { Some(acc) } else { None })
    }).is_some()
}

fn get_nb_field(attributes_content: &[FieldAttributes]) -> usize {
    let values_length = attributes_content.iter().map(|attr| attr.attribute_content.values.len()).collect::<Vec<_>>();

    if values_length.is_empty() {
        panic!("No field in value");
    }

    if values_length.iter().max() != values_length.iter().min() { // All values are equals
        panic!("Must have the same number of fields in all values");
    }

    let nb_field = *values_length.iter().next().unwrap();
    nb_field
}

struct FieldAttributes {
    variant_ident: Ident,
    attribute_content: AttributeContent,
}

#[derive(Debug)]
struct AttributeContent {
    _paren_token: token::Paren,
    values: Punctuated<Value, Token![,]>,
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
struct Value {
    ident: Ident,
    _colon: Token!(:),
    type_name: TypePath,
    _equals: Token!(=),
    value: Literal,
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Value {
            ident: input.parse()?,
            _colon: input.parse()?,
            type_name: input.parse()?,
            _equals: input.parse()?,
            value: input.parse()?,
        })
    }
}
