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

    rust_utils::define_bounded_value_object!(pub Num, i8, 0, -12, 34);

    println!("{:?}", Num::DEFAULT);
    println!("{:?}", Num::check(-13));
    println!("{:?}", Num::check(-12));
    println!("{:?}", Num::check(24));
    println!("{:?}", Num::check(34));
    println!("{:?}", Num::check(35));
}
