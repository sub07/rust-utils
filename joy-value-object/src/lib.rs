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

#[derive(Debug, Eq, PartialEq)]
pub enum Bound {
    Greater,
    Between,
    Lower,
}

#[macro_export]
macro_rules! define_bounded_value_object {
    ($vis:vis $name:ident: $ty:ty, default: $default:expr, min: $min:expr, max: $max:expr $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name($ty);

        impl $name {
            pub const DEFAULT: $name = $name::new_unchecked_with_message($default, "Provided default is invalid");
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
                $name::new_unchecked_with_message(value, "Invalid value")
            }

            pub const fn new_unchecked_with_message(value: $ty, message: &'static str) -> $name {
                match $name::new(value) {
                    Some(x) => x,
                    None => panic!("{}", message),
                }
            }

            pub const fn is_valid(value: $ty) -> bool {
                value >= $name::MIN && value <= $name::MAX
            }

            pub fn check(value: $ty) -> $crate::Bound {
                use std::cmp::Ordering::*;
                match (value.partial_cmp(&$name::MIN).unwrap(), value.partial_cmp(&$name::MAX).unwrap()) {
                    (Greater, Less) | (Equal, _) | (_, Equal) => $crate::Bound::Between,
                    (Less, _) => $crate::Bound::Lower,
                    (_, Greater) => $crate::Bound::Greater,
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
    ($value_object_ty:ident, $($ident:ident => $value:expr,)+) => {
        impl $value_object_ty {
            $(
                pub const $ident: $value_object_ty = $value_object_ty::new_unchecked($value);
            )+
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

        define_bounded_value_object! {
            pub Num: i8,
            default: 0,
            min: -12,
            max: 34,
        };

        generate_bounded_value_object_consts! {
            Num,
            CONST_1 => 5i8,
            CONST_2 => Num::MAX - 2,
        }

        assert_eq!(Bound::Lower, Num::check(-13));
        assert_eq!(Bound::Between, Num::check(-12));
        assert_eq!(Bound::Between, Num::check(24));
        assert_eq!(Bound::Between, Num::check(34));
        assert_eq!(Bound::Greater, Num::check(35));

        assert_eq!(Num::MAX - 2, *Num::CONST_2);
    }
}
