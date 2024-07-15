use joy_macro::EnumStr;

#[derive(EnumStr)]
enum Test {
    Variant1,
    Variant2,
    V,
}

fn main() {
    assert_eq!("V", Test::V.as_str());
    assert_eq!("Variant1", Test::Variant1.as_str());
    assert_eq!("Variant2", Test::Variant2.as_str());
}
