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

#[derive(Debug)]
pub enum Bound {
    Lower,
    In,
    Upper,
}

#[macro_export]
macro_rules! define_bounded_value_object {
    ($vis:vis $name:ident, $ty:ty, $default:expr, $min:expr, $max:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name($ty);

        impl $name {
            pub const DEFAULT: $name = $name::new_unchecked($default);
            pub const MAX: $ty = $max;
            pub const MIN: $ty = $min;

            pub const fn value(&self) -> $ty {
                self.0
            }

            pub const fn new(value: $ty) -> Option<$name> {
                if $name::is_valid(value) {
                    Some($name(value))
                } else {
                    None
                }
            }

            pub const fn new_unchecked(value: $ty) -> $name {
                match $name::new(value) {
                    Some(x) => x,
                    None => panic!("Invalid value"),
                }
            }

            pub const fn is_valid(value: $ty) -> bool {
                value >= $name::MIN && value <= $name::MAX
            }

            pub fn check(value: $ty) -> $crate::value_object::Bound {
                use std::cmp::Ordering::*;
                match (value.cmp(&$name::MIN), value.cmp(&$name::MAX)) {
                    (Greater, Less) | (Equal, _) | (_, Equal) => $crate::value_object::Bound::In,
                    (Less, _) => $crate::value_object::Bound::Lower,
                    (_, Greater) => $crate::value_object::Bound::Upper,
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::DEFAULT
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

#[macro_export]
macro_rules! generate_bounded_value_object_consts {
    ($value_object_ty:ident, $($ident:ident => $value:literal,)+) => {
        impl $value_object_ty {
            $(
                pub const $ident: $value_object_ty = $value_object_ty::new_unchecked($value);
            )+
        }
    };
}