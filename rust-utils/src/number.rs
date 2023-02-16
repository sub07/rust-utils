use std::ops::{Add, Div, Mul, Sub};

pub trait Number:
Copy +
Clone +
PartialOrd +
PartialEq +
Add<Self, Output=Self> +
Sub<Self, Output=Self> +
Mul<Self, Output=Self> +
Div<Self, Output=Self> +
~ const DefaultConst +
{}

macro_rules! impl_primitive {
    ($($ty:path => $default:literal,)*) => {
        $(
            impl Number for $ty {}
            impl const DefaultConst for $ty {
                fn default_const() -> Self {
                    $default
                }
            }
        )*
    };
}

#[const_trait]
pub trait DefaultConst {
    fn default_const() -> Self;
}

impl_primitive!(
    i8 => 0,
    i16 => 0,
    i32 => 0,
    i64 => 0,
    i128 => 0,
    isize => 0,
    u8 => 0,
    u16 => 0,
    u32 => 0,
    u64 => 0,
    u128 => 0,
    usize => 0,
    f32  => 0.0,
    f64 => 0.0,
);

// https://stackoverflow.com/questions/54552847/build-all-pairs-of-elements-quadratic-set-in-declarative-macro
macro_rules! for_all_pairs {
    ($mac:ident: $($x:ident)*) => { for_all_pairs!(@inner $mac: $($x)*; $($x)*);};
    (@inner $mac:ident: ; $($x:ident)*) => {};
    (@inner $mac:ident: $head:ident $($tail:ident)*; $($x:ident)*) => {
        $($mac!($head $x);)*
        for_all_pairs!(@inner $mac: $($tail)*; $($x)*);
    };
}

macro_rules! impl_from {
    ($a:ident $b:ident) => {
        impl FromPrimitive<$a> for $b {
            fn from_p(value: $a) -> $b {
                value as $b
            }
        }
    }
}

for_all_pairs!(impl_from: i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);

#[const_trait]
pub trait FromPrimitive<T: Number>: Number {
    fn from_p(value: T) -> Self;
}

#[const_trait]
pub trait IntoPrimitive<T: Number>: Number {
    fn into_p(self) -> T;
}

impl<T: Number, U: Number + ~const FromPrimitive<T>> const IntoPrimitive<U> for T {
    #[inline(always)]
    fn into_p(self) -> U {
        U::from_p(self)
    }
}
