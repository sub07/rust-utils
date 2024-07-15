use joy_macro::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Test {
    V1,
    V2,
    V3,
    V4,
}

fn main() {
    assert_eq!(0, Test::V1.ordinal());
    assert_eq!(1, Test::V2.ordinal());
    assert_eq!(2, Test::V3.ordinal());
    assert_eq!(3, Test::V4.ordinal());

    assert_eq!(4, Test::COUNT);
    assert_eq!(4, Test::VARIANTS.len());

    assert_eq!([Test::V1, Test::V2, Test::V3, Test::V4], Test::VARIANTS);
}
