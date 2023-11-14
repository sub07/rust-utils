use crate::Direction::{Down, Left, Right, Up};
use rust_utils_macro::EnumValue;

use TestEnum2::Var1;

use crate::TestEnum2::{Var2, Var3, Var4};

#[derive(Debug)]
pub struct Test(u32);

#[derive(EnumValue, Debug)]
pub enum TestEnum2 {
    #[value(offset: f32 = 1.8)]
    Var1(Test),
    #[value(offset: f32 = 2.56)]
    Var2,
    #[value(offset: f32 = 3.2)]
    Var3,
    #[value(offset: f32 = 4.4)]
    Var4,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumValue)]
pub enum Direction {
    #[value(x: i32 = -1, y: i32 = 0)]
    Left,
    #[value(x: i32 = 1, y: i32 = 0)]
    Right,
    #[value(x: i32 = 0, y: i32 = -1)]
    Up,
    #[value(x: i32 = 0, y: i32 = 1)]
    Down,
}

fn main() {
    for variant in [Var1(Test(5)), Var2, Var3, Var4] {
        println!("{:?} {}", variant, variant.offset());
    }

    for variant in &[Left, Right, Up, Down] {
        println!("{:?} x:{} y:{}", variant, variant.x(), variant.y());
    }
}
