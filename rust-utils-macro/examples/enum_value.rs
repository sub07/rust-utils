use rust_utils_macro::{EnumValue};
use TestEnum2::Var1;
use crate::TestEnum2::{Var2, Var3, Var4};

#[derive(EnumValue, Debug)]
pub enum TestEnum2 {
    #[value(offset: f32 = 1.8)]
    Var1,
    #[value(offset: f32 = 2.56)]
    Var2,
    #[value(offset: f32 = 3.2)]
    Var3,
    #[value(offset: f32 = 4.4)]
    Var4,
}

fn main() {
    for variant in vec![Var1, Var2, Var3, Var4] {
        println!("{:?} {}", variant, variant.offset());
    }
}
