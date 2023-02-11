use syn::{Path};

#[derive(Debug, PartialEq, Eq)]
pub enum StructNewAttrib {
    NewDefault,
}

impl StructNewAttrib {
    pub fn try_from_attrib_path(ident: &Path) -> Option<StructNewAttrib> {
        if ident.is_ident("new_default") { return Some(StructNewAttrib::NewDefault); }
        None
    }
}
