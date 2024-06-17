use syn::{Data, DataEnum, DeriveInput, Ident};

pub fn get_enum_data(input: DeriveInput) -> (Ident, DataEnum) {
    if let Data::Enum(data_enum) = input.data {
        (input.ident, data_enum)
    } else {
        panic!("Must be applied on enum")
    }
}
