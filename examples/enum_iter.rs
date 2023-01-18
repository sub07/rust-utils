use rust_utils::{EnumIter, EnumValue};

#[derive(EnumIter, EnumValue, Debug)]
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
    println!("{}", TestEnum2::SIZE);
    println!("{}", TestEnum2::size());
    println!("{:?}", TestEnum2::VARIANTS);
    println!("{:?}", TestEnum2::variants());
    println!("{:?}", TestEnum2::iter());
    for val in TestEnum2::iter() {
        println!("{:?} {}", val, val.offset());
    }
}
