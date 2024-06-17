use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields};

use crate::enum_str::get_snake_case_from_pascal_case;
use crate::enum_value::{get_nb_field, AttributeContent, FieldAttributes};
use crate::struct_new::StructNewAttrib;
use crate::utils::{
    all_equals, get_enum_data, get_generics_types_from_declared, get_struct_data, is_struct_tuple,
    is_type_generic,
};

mod enum_str;
mod enum_value;
mod struct_new;
mod utils;

#[proc_macro_derive(EnumValue, attributes(value))]
pub fn enum_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = get_enum_data(&input);
    let enum_name = &input.ident;
    let attributes_content = enum_data
        .variants
        .iter()
        .map(|v| FieldAttributes {
            variant_ident: v.ident.clone(),
            attribute_content: syn::parse2::<AttributeContent>(
                v.attrs
                    .first()
                    .unwrap_or_else(|| {
                        panic!("Illegal variant without value attribute : {}", v.ident)
                    })
                    .tokens
                    .clone(),
            )
            .expect("Invalid syntax in #[value(...)]"),
            nb_unnamed_fields: match &v.fields {
                Fields::Named(_) => panic!("Unsupported syntax"),
                Fields::Unnamed(fields) => fields.unnamed.len(),
                Fields::Unit => 0,
            },
        })
        .collect::<Vec<_>>();

    let nb_field = get_nb_field(&attributes_content);

    for field_index in 0..nb_field {
        let values = attributes_content
            .iter()
            .map(|f| f.attribute_content.values.iter().nth(field_index).unwrap())
            .collect::<Vec<_>>();
        let idents = values
            .iter()
            .map(|v| v.ident.to_string())
            .collect::<Vec<_>>();
        let types = values
            .iter()
            .map(|v| v.type_name.clone().into_token_stream().to_string())
            .collect::<Vec<_>>();

        if !all_equals(&idents) {
            panic!(
                "Field n°{} names are not all equals : {idents:#?}",
                field_index + 1
            );
        }

        if !all_equals(&types) {
            panic!(
                "Field n°{} types are not all equals : {types:#?}",
                field_index + 1
            );
        }
    }

    let mut functions = quote!();
    for field_index in 0..nb_field {
        let (field_name, type_name) = {
            let value = &attributes_content[field_index].attribute_content.values[field_index];
            (value.ident.clone(), value.type_name.clone())
        };

        let literals = attributes_content
            .iter()
            .map(|attr| {
                (
                    attr.variant_ident.clone(),
                    attr.attribute_content.values[field_index].neg_sign,
                    attr.attribute_content.values[field_index].value.clone(),
                    attr.nb_unnamed_fields,
                )
            })
            .collect::<Vec<_>>();

        let mut match_statement = quote!();

        fn build_field_matching(nb_field: usize) -> proc_macro2::TokenStream {
            let mut res = quote!();
            if nb_field != 0 {
                let mut matching = quote!();
                for _ in 0..nb_field {
                    matching.extend(quote!(_,))
                }
                res.extend(quote!((#matching)));
            }
            res
        }

        for (ident, optional_sign, literal, nb_field) in literals {
            let field_matching = build_field_matching(nb_field);
            match_statement.extend(quote! {
                #enum_name::#ident #field_matching => #optional_sign #literal,
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

#[proc_macro_derive(New, attributes(new_default))]
pub fn struct_new_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_data = get_struct_data(&input);

    if is_struct_tuple(struct_data) {
        panic!("Tuple structs are not supported");
    }

    let struct_name = &input.ident;

    let mut new_fn_params = quote!();
    let mut struct_creation_fields = quote!();
    let mut fn_new_generics = quote!();
    for (i, field) in struct_data.fields.iter().enumerate() {
        let ident = &field.ident.as_ref().unwrap();
        let type_name = &field.ty;

        let attribs = field
            .attrs
            .iter()
            .filter_map(|attr| StructNewAttrib::try_from_attrib_path(&attr.path))
            .collect::<Vec<_>>();

        let is_new_default_attrib_present = attribs
            .iter()
            .any(|attr| *attr == StructNewAttrib::NewDefault);

        if is_new_default_attrib_present {
            struct_creation_fields.extend(quote!(#ident: Default::default(),));
        } else if is_type_generic(&input.generics, &field.ty) {
            new_fn_params.extend(quote!(#ident: #type_name,));
            struct_creation_fields.extend(quote!(#ident,));
        } else {
            let gen_ident = syn::Ident::new(&format!("T{i}"), Span::call_site());
            new_fn_params.extend(quote!(#ident: #gen_ident,));
            struct_creation_fields.extend(quote!(#ident: #ident.into(),));
            fn_new_generics.extend(quote!(#gen_ident: Into<#type_name>,));
        }
    }

    let declared_generics = &input.generics;
    let generics_idents = get_generics_types_from_declared(declared_generics);

    let res = quote! {
        impl #declared_generics #struct_name #generics_idents {
            pub fn new<#fn_new_generics>(#new_fn_params) -> #struct_name #generics_idents {
                #struct_name {
                    #struct_creation_fields
                }
            }
        }
    };

    res.into()
}

#[proc_macro_derive(EnumStr)]
pub fn enum_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = get_enum_data(&input);
    let enum_name = &input.ident;

    let mut match_elems = quote!();
    let mut match_elems_snake_case = quote!();

    for variant in &enum_data.variants {
        let ident = &variant.ident;
        let ident_str = ident.to_string();
        match_elems.extend(quote!(#enum_name::#ident => #ident_str,));
        let snake_case = get_snake_case_from_pascal_case(&ident_str);
        match_elems_snake_case.extend(quote!(#enum_name::#ident => #snake_case,));
    }

    let res = quote! {
        impl #enum_name {
            fn as_str(&self) -> &'static str {
                match self {
                    #match_elems
                }
            }

            fn as_str_snakecase(&self) -> &'static str {
                match self {
                    #match_elems_snake_case
                }
            }
        }
    };

    res.into()
}
