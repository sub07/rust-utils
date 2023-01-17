use rust_utils::{EnumIter, EnumValue};

#[derive(EnumIter, EnumValue, Debug)]
pub enum TestEnum2 {
    #[value(offset = 8.0, ding = 5.6)]
    Var1,
    #[value(offset = 8.0, ding = 5.6)]
    Var2,
    #[value(offset = 8.0, ding = 5.6)]
    Var3,
    #[value(offset = 8.0, ding = 5.6)]
    Var4,
}

fn main() {
    println!("{}", TestEnum2::SIZE);
    println!("{}", TestEnum2::size());
    println!("{:?}", TestEnum2::VARIANTS);
    println!("{:?}", TestEnum2::variants());
    println!("{:?}", TestEnum2::iter());
}