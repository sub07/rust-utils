use syn::{Data, DataEnum, DataStruct, DeriveInput, Type};

pub fn get_enum_data(input: &DeriveInput) -> &DataEnum {
    if let Data::Enum(data_enum) = &input.data {
        data_enum
    } else {
        panic!("Must be applied on enum")
    }
}

pub fn get_struct_data(input: &DeriveInput) -> &DataStruct {
    if let Data::Struct(data_struct) = &input.data {
        data_struct
    } else {
        panic!("Must be applied on struct")
    }
}

pub fn is_struct_tuple(struct_data: &DataStruct) -> bool {
    !struct_data.fields.is_empty() && struct_data.fields.iter().next().unwrap().ident.is_none()
}

pub fn all_equals<T: Eq>(idents: &[T]) -> bool {
    let first = idents.iter().next();
    idents.iter().fold(first, |acc, ident| {
        acc.and_then(|acc| if acc == ident { Some(acc) } else { None })
    }).is_some()
}

pub enum TypeKind {
    Primitive,
    String,
    Other,
}

impl TypeKind {
    pub fn from_type(ty: &Type) -> TypeKind {
        if let Type::Path(type_path) = ty {
            let primitives = ["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "char", "bool"];
            if type_path.path.segments.last().unwrap().ident == "String" {
                TypeKind::String
            } else if primitives.iter().any(|p| type_path.path.segments.last().unwrap().ident == *p) {
                TypeKind::Primitive
            } else {
                TypeKind::Other
            }
        } else {
            TypeKind::Other
        }
    }
}