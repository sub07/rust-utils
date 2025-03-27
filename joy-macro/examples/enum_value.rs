use joy_macro::{EnumIter, EnumValue};

#[derive(EnumValue, EnumIter)]
enum Test {
    #[value(x: u32 = 8, y: &'static str = "test1")]
    Var1,
    #[value(x: u32 = 9, y: &'static str = "test2")]
    Var2,
    #[value(x: u32 = 10, y: &'static str = "test3")]
    Var3,
    #[value(x: u32 = 11, y: &'static str = "test4")]
    Var4,
}

fn main() {
    dbg!(Test::VARIANTS.map(|v| v.x()));
    dbg!(Test::VARIANTS.map(|v| v.y()));
}
