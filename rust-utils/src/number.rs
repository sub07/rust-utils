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
