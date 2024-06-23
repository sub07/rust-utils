#[macro_export]
macro_rules! define_value_object {
    ($vis:vis $name:ident, $ty:ty, $default:expr) => {
        define_value_object!($vis $name, $ty, $default, |_v| { true });
    };
    ($vis:vis $name:ident, $ty:ty, $default:expr, |$value:ident| $validation_body:block) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name($ty);

        impl $name {
            pub fn value(&self) -> $ty {
                self.0
            }

            pub fn new(value: $ty) -> Option<Self> {
                #[allow(clippy::redundant_closure_call)]
                let valid = (|$value: $ty| $validation_body)(value);
                if valid { Some($name(value)) } else { None }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                ($default).try_into().expect("Invalid default value {}")
            }
        }

        impl TryFrom<$ty> for $name {
            type Error = String;

            fn try_from(value: $ty) -> Result<Self, String> {
                $name::new(value).ok_or(format!("Invalid value: {}", value))
            }
        }

        impl std::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    // Should be split up
    #[test]
    fn vo_test() {
        define_value_object!(pub Percentage, f32, 0.0, |v| { (0.0..=1.0).contains(&v) });
        define_value_object!(pub PercentageI32, i32, 0, |v| { (0..100).contains(&v) });
        let perc1 = Percentage::new(5.0);
        let perc2 = Percentage::new(0.2);

        let perci32_1 = PercentageI32::new(54);
        let perci32_2 = PercentageI32::new(-5);

        assert!(perc1.is_none());
        assert!(perc2.is_some());

        assert_eq!(Some(0.2), perc2.map(|p| p.value()));

        assert!(perci32_1.is_some());
        assert!(perci32_2.is_none());
    }
}
