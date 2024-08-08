use std::fmt::Debug;

use crate::gen_common_ignore_impl;

pub struct PartialEqImplIgnore<T>(pub T);

impl<T> PartialEqImplIgnore<T> {
    pub fn take(self) -> T {
        self.0
    }
}

impl<T> PartialEq for PartialEqImplIgnore<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T: Debug> Debug for PartialEqImplIgnore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

gen_common_ignore_impl!(PartialEqImplIgnore);

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use super::*;

    #[derive(PartialEq, Clone)]
    #[allow(dead_code)]
    struct EqImpl(i32);

    impl Debug for EqImpl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("EqImpl").field(&self.0).finish()
        }
    }

    #[derive(PartialEq)]
    struct TestPartialEq(PartialEqImplIgnore<EqImpl>, i32, String);

    impl Debug for TestPartialEq {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("TestPartialEq")
                .field(&"ignored for test")
                .field(&self.1)
                .field(&self.2)
                .finish()
        }
    }

    fn get_eq_impl() -> EqImpl {
        EqImpl(23)
    }

    fn get_bypassed_eq() -> PartialEqImplIgnore<EqImpl> {
        get_eq_impl().into()
    }

    fn get_test_struct() -> TestPartialEq {
        TestPartialEq(get_bypassed_eq(), 23, "test".into())
    }

    #[test]
    fn test_large_output_without_bypass() {
        let eq_impl = get_eq_impl();
        let same_eq_impl = get_eq_impl();
        let diff_eq_impl = EqImpl(4);

        assert_eq!(eq_impl, same_eq_impl);
        assert_ne!(eq_impl, diff_eq_impl);
    }

    #[test]
    fn test_that_ignored_eq_is_always_true() {
        let bypassed = get_bypassed_eq();
        let diff_bypassed = PartialEqImplIgnore(EqImpl(5));
        assert!(bypassed == diff_bypassed);
    }

    #[test]
    fn test_that_clone_works_when_underlying_type_is_clonable() {
        let clonable = get_bypassed_eq();
        let _ = clonable.clone();
    }

    #[test]
    fn test_that_copy_works_when_underlying_type_is_copiable() {
        let copiable = PartialEqImplIgnore(32i32);
        let _: i32 = *copiable;
    }

    #[test]
    fn test_that_ignored_struct_fields_are_ignored_but_not_the_others() {
        let s = get_test_struct();
        let same_s = get_test_struct();
        let diff_ignored_but_same_s_otherwise = TestPartialEq(EqImpl(4).into(), 23, "test".into());

        assert_eq!(same_s, s);
        assert_eq!(diff_ignored_but_same_s_otherwise, s);
    }
}
