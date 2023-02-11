use std::cell::RefCell;
use std::rc::Rc;
use rust_utils_macro::{New};

#[derive(Debug, New)]
struct T;

#[allow(dead_code)]
#[derive(New, Debug)]
struct Test {
    pub gen: Rc<RefCell<T>>,
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
struct TGenerics<T> {
    t: T,
    i: i8,
}

fn main() {
    let test = Test::new(Rc::new(RefCell::new(T)), 5, 6,);
    dbg!(&test);
    let gen = TGenerics::<Test>::new(test, 8);
    dbg!(gen);
}
