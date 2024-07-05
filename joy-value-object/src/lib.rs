#![feature(const_fn_floating_point_arithmetic)]

#[derive(Debug, Eq, PartialEq)]
pub enum Bound {
    Greater,
    Between,
    Lower,
}

#[macro_export]
macro_rules! mk_vo {
    ($vis:vis $name:ident: $ty:ty, default: $default:expr, min: $min:expr, max: $max:expr $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name($ty);

        impl $name {
            pub const DEFAULT: $name = $name::new_unchecked_with_message($default, "Provided default is invalid");
            pub const MAX_VALUE: $ty = $max;
            pub const MIN_VALUE: $ty = $min;
            pub const MAX: $name = $name::new_unchecked_with_message(Self::MAX_VALUE, "Provided max is invalid");
            pub const MIN: $name = $name::new_unchecked_with_message(Self::MIN_VALUE, "Provided min is invalid");

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

            pub const fn new_clamped(value: $ty) -> $name {
                let final_value = if value < Self::MIN_VALUE {
                    Self::MIN_VALUE
                } else if value > Self::MAX_VALUE {
                    Self::MAX_VALUE
                } else {
                    value
                };

                Self::new_unchecked(final_value)
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
                value >= $name::MIN_VALUE && value <= $name::MAX_VALUE
            }

            pub fn check(value: $ty) -> $crate::Bound {
                use std::cmp::Ordering::*;
                match (value.partial_cmp(&$name::MIN_VALUE).unwrap(), value.partial_cmp(&$name::MAX_VALUE).unwrap()) {
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

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self::new_unchecked(value)
            }
        }

        impl std::ops::Add<$name> for $name {
            type Output = $name;

            fn add(self, rhs: $name) -> Self::Output {
                Self::new_clamped(self.value() + rhs.value())
            }
        }

        impl std::ops::Add<$ty> for $name {
            type Output = $name;

            fn add(self, rhs: $ty) -> Self::Output {
                Self::new_clamped(self.value() + rhs)
            }
        }

        impl std::ops::Sub<$name> for $name {
            type Output = $name;

            fn sub(self, rhs: $name) -> Self::Output {
                Self::new_clamped(self.value() - rhs.value())
            }
        }

        impl std::ops::Sub<$ty> for $name {
            type Output = $name;

            fn sub(self, rhs: $ty) -> Self::Output {
                Self::new_clamped(self.value() - rhs)
            }
        }

        impl std::ops::Mul<$name> for $name {
            type Output = $name;

            fn mul(self, rhs: $name) -> Self::Output {
                Self::new_clamped(self.value() * rhs.value())
            }
        }

        impl std::ops::Mul<$ty> for $name {
            type Output = $name;

            fn mul(self, rhs: $ty) -> Self::Output {
                Self::new_clamped(self.value() * rhs)
            }
        }

        impl std::ops::Div<$name> for $name {
            type Output = $name;

            fn div(self, rhs: $name) -> Self::Output {
                Self::new_clamped(self.value() / rhs.value())
            }
        }

        impl std::ops::Div<$ty> for $name {
            type Output = $name;

            fn div(self, rhs: $ty) -> Self::Output {
                Self::new_clamped(self.value() / rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! mk_vo_consts {
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

    mk_vo! {
        pub Num: i8,
        default: 0,
        min: -12,
        max: 34,
    }

    mk_vo_consts! {
        Num,
        CONST_1 => 5i8,
        CONST_2 => Num::MAX_VALUE - 2,
    }

    mk_vo! {
        pub Numf32: f32,
        default: 0.0,
        min: -12.6,
        max: 34.5,
    }

    mk_vo_consts! {
        Numf32,
        CONST_1 => 5.0,
        CONST_2 => Numf32::MAX_VALUE - 2.0,
    }

    #[test]
    fn test_new_clamped() {
        assert_eq!(Num::MIN, Num::new_clamped(-89));
        assert_eq!(Num(18), Num::new_clamped(18));
        assert_eq!(Num::MAX, Num::new_clamped(82));
    }

    #[test]
    fn test_from() {
        let num: Num = 8.into();
        assert_eq!(Num(8), num);
    }

    #[test]
    fn test_const_values() {
        assert_eq!(Num(5), Num::CONST_1);
        assert_eq!(Num(Num::MAX_VALUE - 2), Num::CONST_2);
    }

    #[test]
    fn test_bounds() {
        assert_eq!(Bound::Lower, Num::check(-13));
        assert_eq!(Bound::Between, Num::check(-12));
        assert_eq!(Bound::Between, Num::check(24));
        assert_eq!(Bound::Between, Num::check(34));
        assert_eq!(Bound::Greater, Num::check(35));
    }

    #[test]
    fn test_deref() {
        assert_eq!(Num::MAX_VALUE - 2, *Num::CONST_2);
    }

    #[test]
    fn test_add() {
        assert_eq!(Num::MAX, Num::new_unchecked(30) + Num::new_unchecked(9));
        assert_eq!(Num::MIN, Num::new_unchecked(-10) + Num::new_unchecked(-10));
        assert_eq!(Num(5 + 9), Num::new_unchecked(5) + Num::new_unchecked(9));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Num::MAX, Num::new_unchecked(30) - Num::new_unchecked(-9));
        assert_eq!(Num::MIN, Num::new_unchecked(1) - Num::new_unchecked(30));
        assert_eq!(Num(5 - 9), Num::new_unchecked(5) - Num::new_unchecked(9));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Num::MAX, Num::new_unchecked(5) * Num::new_unchecked(9));
        assert_eq!(Num::MIN, Num::new_unchecked(5) * Num::new_unchecked(-9));
        assert_eq!(Num(3 * 3), Num::new_unchecked(3) * Num::new_unchecked(3));
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Numf32::MAX,
            Numf32::new_unchecked(5.0) / Numf32::new_unchecked(0.01)
        );
        assert_eq!(
            Numf32::MIN,
            Numf32::new_unchecked(-9.0) / Numf32::new_unchecked(0.01)
        );
        assert_eq!(Num(9 / 3), Num::new_unchecked(9) / Num::new_unchecked(3));
    }

    #[test]
    fn test_add_value() {
        assert_eq!(Num::MAX, Num::new_unchecked(30) + 9);
        assert_eq!(Num::MIN, Num::new_unchecked(-10) + -10);
        assert_eq!(Num(5 + 9), Num::new_unchecked(5) + 9);
    }

    #[test]
    fn test_sub_value() {
        assert_eq!(Num::MAX, Num::new_unchecked(30) - -9);
        assert_eq!(Num::MIN, Num::new_unchecked(1) - 30);
        assert_eq!(Num(5 - 9), Num::new_unchecked(5) - 9);
    }

    #[test]
    fn test_mul_value() {
        assert_eq!(Num::MAX, Num::new_unchecked(5) * 9);
        assert_eq!(Num::MIN, Num::new_unchecked(5) * -9);
        assert_eq!(Num(3 * 3), Num::new_unchecked(3) * 3);
    }

    #[test]
    fn test_div_value() {
        assert_eq!(Numf32::MAX, Numf32::new_unchecked(5.0) / 0.01);
        assert_eq!(Numf32::MIN, Numf32::new_unchecked(-9.0) / 0.01);
        assert_eq!(Num(9 / 3), Num::new_unchecked(9) / 3);
    }
}
