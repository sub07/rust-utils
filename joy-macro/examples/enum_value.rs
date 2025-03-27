use joy_macro::EnumValue;

#[derive(EnumValue)]
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
    dbg!(Test::Var1.x());
    dbg!(Test::Var2.x());
    dbg!(Test::Var3.x());
    dbg!(Test::Var4.x());

    dbg!(Test::Var1.y());
    dbg!(Test::Var2.y());
    dbg!(Test::Var3.y());
    dbg!(Test::Var4.y());
}
