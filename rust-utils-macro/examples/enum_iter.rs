use rust_utils_macro::{EnumIter};

#[derive(EnumIter, Debug)]
pub enum TestEnum2 {
    Var1,
    Var2,
    Var3,
    Var4,
}

fn main() {
    println!("{}", TestEnum2::SIZE);
    println!("{}", TestEnum2::size());
    println!("{:?}", TestEnum2::VARIANTS);
    println!("{:?}", TestEnum2::variants());
    println!("{:?}", TestEnum2::iter());
}
