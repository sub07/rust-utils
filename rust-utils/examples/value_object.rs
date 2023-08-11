use rust_utils::define_value_object;

fn main() {
    define_value_object!(pub Percentage, f32, 1.0, |v| { (0.0..=1.0).contains(&v) });
    let perc1 = Percentage::new(5.0);
    let perc2 = Percentage::new(0.2);

    assert!(perc1.is_none());
    assert!(perc2.is_some());
}