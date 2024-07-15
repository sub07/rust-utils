#![allow(dead_code)]

use std::fmt::Display;

use joy_macro::New;

#[derive(New, Debug, PartialEq)]
pub struct GenericsWithBound<'a, T: Display, E: Default, const SIZE: usize> {
    _t1: i32,
    _t: T,
    _e: E,
    _r: &'a u8,
    _a: [u8; SIZE],
}

fn main() {
    let i = 5;
    let g = GenericsWithBound::new(5, 7.0f32, 9u8, &i, [6; 5]);
    assert_eq!(
        GenericsWithBound {
            _t1: 5,
            _t: 7.0f32,
            _e: 9u8,
            _r: &i,
            _a: [6; 5]
        },
        g
    );
}
