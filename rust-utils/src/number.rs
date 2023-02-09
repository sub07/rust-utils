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
{}

macro_rules! impl_primitive {
    ($($ty:path)*) => {
        $(
            impl Number for $ty {}
        )*
    };
}

impl_primitive!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);