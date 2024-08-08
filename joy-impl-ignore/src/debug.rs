use std::fmt::Debug;

use crate::gen_common_ignore_impl;

pub struct DebugImplIgnore<T>(pub T);

impl<T> DebugImplIgnore<T> {
    pub fn take(self) -> T {
        self.0
    }
}

impl<T> Debug for DebugImplIgnore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**ignored**")
    }
}

impl<T: PartialEq> PartialEq for DebugImplIgnore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

gen_common_ignore_impl!(DebugImplIgnore);

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct LargeDebug(Vec<i32>);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct TestIgnore(DebugImplIgnore<LargeDebug>, i32, String);

    fn get_large() -> LargeDebug {
        LargeDebug(vec![0; 20000])
    }

    fn get_bypassed_large() -> DebugImplIgnore<LargeDebug> {
        get_large().into()
    }

    fn get_test_struct() -> TestIgnore {
        TestIgnore(get_bypassed_large(), 23, "test".into())
    }

    #[test]
    fn test_large_output_without_bypass() {
        let large = get_large();
        let debug_str = format!("{large:?}");
        assert_eq!(2 + 2 + 59998 + 10, debug_str.len());
    }

    #[test]
    fn test_that_debug_impl_is_eq_to_ignore_substitute() {
        let bypassed = get_bypassed_large();
        assert_eq!("**ignored**", format!("{bypassed:?}"));
    }

    #[test]
    fn test_that_clone_works_when_underlying_type_is_clonable() {
        let clonable = get_bypassed_large();
        let _ = clonable.clone();
    }

    #[test]
    fn test_that_copy_works_when_underlying_type_is_copiable() {
        let copiable = DebugImplIgnore(32i32);
        let _: i32 = *copiable;
    }

    #[test]
    fn test_that_ignored_struct_fields_are_ignored_but_not_the_others() {
        let s = get_test_struct();
        assert_eq!("TestIgnore(**ignored**, 23, \"test\")", format!("{s:?}"));
    }
}
