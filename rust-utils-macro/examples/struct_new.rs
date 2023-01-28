use std::cell::RefCell;
use std::rc::Rc;
use rust_utils_macro::{New};

#[derive(Debug, New)]
struct T;

#[derive(New)]
struct PatternsView {
    pub model: Rc<RefCell<T>>,
    #[new_default]
    pub x_offset: i32,
    #[new_default]
    pub y_offset: i32,
    // In glyph unit
    pub height: i32,
    // In glyph unit
    pub width: i32,
}

fn main() {
    // let test = Test::new("test".into(), T::new());
    // dbg!(test);
}