use syn::{Path};

#[derive(PartialEq, Eq)]
pub enum StructGetterAttrib {
    GetterForceCopy,
    NoGetter,
}

impl StructGetterAttrib {
    pub fn try_from_attrib_path(ident: &Path) -> Option<StructGetterAttrib> {
        if ident.is_ident("getter_force_copy") { return Some(StructGetterAttrib::GetterForceCopy); }
        if ident.is_ident("no_getter") { return Some(StructGetterAttrib::NoGetter); }
        None
    }
}