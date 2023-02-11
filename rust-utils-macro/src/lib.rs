use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::enum_str::get_snake_case_from_pascal_case;
use crate::enum_value::{AttributeContent, FieldAttributes, get_nb_field};
use crate::struct_getter::StructGetterAttrib;
use crate::struct_new::StructNewAttrib;
use crate::utils::{all_equals, get_enum_data, get_struct_data, is_struct_tuple, is_type_generic, TypeKind};

mod utils;
mod enum_value;
mod struct_getter;
mod enum_str;
mod struct_new;

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
pub fn enum_value_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_data = get_enum_data(&input);
    let enum_name = &input.ident;
    let attributes_content = enum_data
        .variants.iter()
        .map(|v|
            FieldAttributes {
                variant_ident: v.ident.clone(),
                attribute_content: syn::parse2::<AttributeContent>(
                    v.attrs.first()
                        .unwrap_or_else(|| panic!("Illegal variant without value attribute : {}", v.ident))
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

        let attribs = field.attrs.iter()
            .filter_map(|attr| StructNewAttrib::try_from_attrib_path(&attr.path))
            .collect::<Vec<_>>();

        let is_new_default_attrib_present = attribs.iter().any(|attr| *attr == StructNewAttrib::NewDefault);

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

    let generics = &input.generics;
    let res = quote! {
        impl #generics #struct_name #generics {
            pub fn new<#fn_new_generics>(#new_fn_params) -> #struct_name #generics {
                #struct_name {
                    #struct_creation_fields
                }
            }
        }
    };

    res.into()
}

#[proc_macro_derive(Getter, attributes(getter_force_copy, no_getter))]
pub fn struct_getter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_data = get_struct_data(&input);

    if is_struct_tuple(struct_data) {
        panic!("Tuple structs are not supported");
    }

    let struct_name = &input.ident;

    let mut getters_fn = quote!();

    for field in &struct_data.fields {
        let ident = &field.ident.as_ref().unwrap();
        let type_name = &field.ty;
        let attribs = field.attrs.iter()
            .filter_map(|attr| StructGetterAttrib::try_from_attrib_path(&attr.path))
            .collect::<Vec<_>>();

        if attribs.iter().any(|attr| *attr == StructGetterAttrib::NoGetter) { continue; }

        let force_copy = attribs.iter().any(|attr| *attr == StructGetterAttrib::GetterForceCopy);

        let (type_name, fn_body) = match TypeKind::from_type(type_name) {
            TypeKind::Primitive | TypeKind::PrimitiveTuple => (quote!(#type_name), quote!(self.#ident)),
            TypeKind::String => (quote!(&str), quote!(&self.#ident)),
            TypeKind::Other => (
                if force_copy { quote!(#type_name) } else { quote!(&#type_name) },
                if force_copy { quote!(self.#ident) } else { quote!(&self.#ident) },
            ),
        };
        getters_fn.extend(quote! {
            pub fn #ident(&self) -> #type_name {
                #fn_body
            }
        })
    }

    let res = quote! {
        impl #struct_name {
            #getters_fn
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
