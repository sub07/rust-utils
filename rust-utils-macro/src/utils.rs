use syn::{Data, DataEnum, DataStruct, DeriveInput, Generics, Type, TypePath, TypeTuple};
use syn::Type::Path;

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
    PrimitiveTuple,
    String,
    Other,
}

const PRIMITIVES: [&str; 16] = ["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "char", "bool"];


fn is_primitive_tuple(tuple_type: &TypeTuple) -> bool {
    tuple_type.elems.iter().all(|ty| {
        match ty {
            Type::Tuple(tuple) => is_primitive_tuple(tuple),
            Type::Path(ty) => PRIMITIVES.iter().any(|p| ty.path.segments.last().unwrap().ident == *p),
            _ => panic!("I forgot a kind of type"),
        }
    })
}

impl TypeKind {
    pub fn from_type(ty: &Type) -> TypeKind {
        match ty {
            Type::Path(ty) if ty.path.segments.last().unwrap().ident == "String" => TypeKind::String,
            Type::Path(ty) if PRIMITIVES.iter().any(|p| ty.path.segments.last().unwrap().ident == *p) => TypeKind::Primitive,
            Type::Tuple(tuple) if is_primitive_tuple(tuple) => TypeKind::PrimitiveTuple,
            _ => TypeKind::Other,
        }
    }
}

pub fn is_type_generic(generics: &Generics, ty: &Type) -> bool {
    if let Path(TypePath { path, .. }) = &ty {
        generics.type_params().map(|tp| &tp.ident).any(|g| path.is_ident(g))
    } else {
        false
    }
}
