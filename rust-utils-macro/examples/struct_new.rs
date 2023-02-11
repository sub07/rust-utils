use std::cell::RefCell;
use std::rc::Rc;
use rust_utils_macro::{New};

#[derive(Debug, New)]
struct S;

#[allow(dead_code)]
#[derive(New, Debug)]
struct Test {
    pub gen: Rc<RefCell<S>>,
    #[new_default]
    pub x_offset: i32,
    #[new_default]
    pub y_offset: i32,
    // Comment
    pub height: i32,
    // Comment
    pub width: i32,
}

#[derive(New, Debug)]
struct TGenerics<T, G> {
    t: Rc<T>,
    i: i8,
    g: G,
}

fn main() {
    let test = Test::new(Rc::new(RefCell::new(S)), 5, 6,);
    dbg!(&test);
    let gen = TGenerics::new(test, 8, 6);
    dbg!(gen);
}
