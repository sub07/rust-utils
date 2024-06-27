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

            pub fn saturating_add(&self, rhs: $ty) -> $name {
                let new_value = self.value() + rhs;
                match $name::check(new_value) {
                    $crate::Bound::Greater => Self::MAX,
                    $crate::Bound::Between => Self::new_unchecked(new_value),
                    $crate::Bound::Lower => Self::MIN,
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
                Num::new_unchecked(value)
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

    #[test]
    fn vo_test() {
        mk_vo! {
            pub Num: i8,
            default: 0,
            min: -12,
            max: 34,
        };

        mk_vo_consts! {
            Num,
            CONST_1 => 5i8,
            CONST_2 => Num::MAX_VALUE - 2,
        }

        let num: Num = 8.into();
        assert_eq!(Num(8), num);

        assert_eq!(Num(5), Num::CONST_1);
        assert_eq!(Num(Num::MAX_VALUE - 2), Num::CONST_2);

        let num = Num::new(5).unwrap();
        let num2 = num.saturating_add(9);
        assert_eq!(14, num2.value());

        let num3 = num2.saturating_add(100);
        assert_eq!(Num::MAX, num3);

        let num3 = num2.saturating_add(-100);
        assert_eq!(Num::MIN, num3);

        assert_eq!(Bound::Lower, Num::check(-13));
        assert_eq!(Bound::Between, Num::check(-12));
        assert_eq!(Bound::Between, Num::check(24));
        assert_eq!(Bound::Between, Num::check(34));
        assert_eq!(Bound::Greater, Num::check(35));

        assert_eq!(Num::MAX_VALUE - 2, *Num::CONST_2);
    }
}
