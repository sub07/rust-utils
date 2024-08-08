mod debug;
mod eq;

#[macro_export]
macro_rules! gen_common_ignore_impl {
    ($name:ident) => {
        impl<T: Clone> Clone for $name<T> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<T: Copy> Copy for $name<T> {}

        impl<T> std::ops::Deref for $name<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T> From<T> for $name<T> {
            fn from(value: T) -> Self {
                $name(value)
            }
        }
    };
}

#[cfg(test)]
mod test {
    use debug::DebugImplIgnore;
    use eq::PartialEqImplIgnore;

    use super::*;

    #[derive(PartialEq, Debug)]
    struct TestDebugPartialEq(i32, &'static str, PartialEqImplIgnore<DebugImplIgnore<i32>>);

    #[test]
    fn test_combine_debug_and_eq_ignore() {
        let s = TestDebugPartialEq(5, "test", PartialEqImplIgnore(DebugImplIgnore(5)));
        let diff_s = TestDebugPartialEq(5, "test", PartialEqImplIgnore(DebugImplIgnore(6)));
        assert_eq!(s, diff_s);
        assert_eq!(r#"TestDebugPartialEq(5, "test", **ignored**)"#, format!("{s:?}"));
    }
}
