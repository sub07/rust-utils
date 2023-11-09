#![cfg_attr(feature = "nightly", feature(const_fn_floating_point_arithmetic))]



use rust_utils::define_value_object;

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

    #[cfg(feature = "nightly")]
    {
        rust_utils::define_bounded_value_object!(pub Num, f32, 0.0, -12.0, 34.0);

        println!("{:?}", Num::DEFAULT);
    }
}
