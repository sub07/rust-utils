use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use utils::get_enum_data;

mod enum_iter;
mod utils;

#[proc_macro_derive(EnumIter)]
pub fn enum_iter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (enum_ident, enum_data) = get_enum_data(input);
    enum_iter::derive(enum_ident, enum_data).into()
}
