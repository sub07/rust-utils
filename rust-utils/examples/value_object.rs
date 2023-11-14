use rust_utils::{
    define_bounded_value_object, define_value_object, generate_bounded_value_object_consts,
    value_object::Bound,
};

fn main() {
    define_value_object!(pub Percentage, f32, 0.0, |v| { (0.0..=1.0).contains(&v) });
    define_value_object!(pub PercentageI32, i32, 0, |v| { (0..100).contains(&v) });
    let perc1 = Percentage::new(5.0);
    let perc2 = Percentage::new(0.2);

    let perci32_1 = PercentageI32::new(54);
    let perci32_2 = PercentageI32::new(-5);

    assert!(perc1.is_none());
    assert!(perc2.is_some());

    assert!(perci32_1.is_some());
    assert!(perci32_2.is_none());

    define_bounded_value_object! {
        pub Num: i8,
        default: 0,
        min: -12,
        max: 34,
    };

    generate_bounded_value_object_consts! {
        Num,
        CONST_1 => 5i8,
        CONST_2 => Num::MAX - 2,
    }

    assert_eq!(Bound::Lower, Num::check(-13));
    assert_eq!(Bound::In, Num::check(-12));
    assert_eq!(Bound::In, Num::check(24));
    assert_eq!(Bound::In, Num::check(34));
    assert_eq!(Bound::Upper, Num::check(35));

    assert_eq!(Num::MAX - 2, *Num::CONST_2);
}
