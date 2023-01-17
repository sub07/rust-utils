use rust_utils::{EnumIter, EnumValue};

#[derive(EnumIter, EnumValue, Debug)]
pub enum TestEnum2 {
    #[value(offset: f32 = 1.0)]
    Var1,
    #[value(offset: f32 = 2.0)]
    Var2,
    #[value(offset: f32 = 3.0)]
    Var3,
    #[value(offset: f32 = 4.0)]
    Var4,
}

fn main() {
    println!("{}", TestEnum2::SIZE);
    println!("{}", TestEnum2::size());
    println!("{:?}", TestEnum2::VARIANTS);
    println!("{:?}", TestEnum2::variants());
    println!("{:?}", TestEnum2::iter());
}
