use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use utils::{get_enum_data, get_struct_data};

mod enum_iter;
mod struct_new;
mod utils;

#[proc_macro_derive(EnumIter)]
pub fn enum_iter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (_, ident, data) = get_enum_data(input);
    enum_iter::derive(ident,data).into()
}

#[proc_macro_derive(New)]
pub fn struct_new_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (generics, ident, data) = get_struct_data(input);
    struct_new::derive(generics, ident, data).into()
}
