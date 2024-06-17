use syn::{Data, DataEnum, DataStruct, DeriveInput, Generics, Ident};

pub fn get_enum_data(input: DeriveInput) -> (Generics, Ident, DataEnum) {
    if let Data::Enum(data) = input.data {
        (input.generics, input.ident, data)
    } else {
        panic!("Must be applied on an enum")
    }
}

pub fn get_struct_data(input: DeriveInput) -> (Generics, Ident, DataStruct) {
    if let Data::Struct(data) = input.data {
        (input.generics, input.ident, data)
    } else {
        panic!("Must be applied on a struct")
    }
}
