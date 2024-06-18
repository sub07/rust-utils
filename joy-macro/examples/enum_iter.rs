use joy_macro::EnumIter;

#[derive(EnumIter, Debug)]
pub enum Test {
    Var1,
    Var2,
    Var3,
    Var4,
}

fn main() {
    println!("{}", Test::SIZE);
    println!("{:?}", Test::VARIANTS);
    println!("{:?}", Test::Var3.ordinal());
}
