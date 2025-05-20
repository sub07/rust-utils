use joy_macro::EnumStr;

#[derive(EnumStr)]
enum Test1 {
    Variant1,
    Variant2,
    V,
}

#[derive(EnumStr)]
enum Test2 {
    Variant1(()),
    Variant2 { _test: String },
    V,
}

fn main() {
    assert_eq!("V", Test1::V.as_str());
    assert_eq!("Variant1", Test1::Variant1.as_str());
    assert_eq!("Variant2", Test1::Variant2.as_str());

    assert_eq!("Variant1", Test2::Variant1(()).as_str());
    assert_eq!(
        "Variant2",
        Test2::Variant2 {
            _test: "test".into()
        }
        .as_str()
    );
    assert_eq!("V", Test2::V.as_str());
}
