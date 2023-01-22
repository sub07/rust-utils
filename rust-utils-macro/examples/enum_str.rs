use rust_utils_macro::EnumStr;

#[derive(EnumStr)]
pub enum TestEnum2 {
    Var1,
    VName,
    TestWord,
    AnotherTest5Word5,
}

fn main() {
    dbg!(TestEnum2::TestWord.as_str());
    dbg!(TestEnum2::TestWord.as_str_snakecase());

    dbg!(TestEnum2::Var1.as_str());
    dbg!(TestEnum2::Var1.as_str_snakecase());

    dbg!(TestEnum2::AnotherTest5Word5.as_str());
    dbg!(TestEnum2::AnotherTest5Word5.as_str_snakecase());
}