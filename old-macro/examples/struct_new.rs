use old_macro::New;
use std::fmt::Display;

#[derive(New, Debug)]
pub struct GenericsWithBound<'a, T: Display, E: Default, const SIZE: usize> {
    _t1: i32,
    _t: T,
    _e: E,
    _r: &'a u8,
    _a: [u8; SIZE],
}

fn main() {
    // let test = Test::new(Rc::new(RefCell::new(S)), 5, 6,);
    // dbg!(&test);
    // let gen = TGenerics::new(test, 8, 6);
    // dbg!(gen);
    let i = 5;
    let g = GenericsWithBound::new(5, 7.0, 9u8, &i, [6; 5]);
    dbg!(g);
}
