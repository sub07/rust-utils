#![allow(dead_code)]

use std::fmt::Display;

use joy_macro::New;

#[derive(New, Debug)]
pub struct GenericsWithBound<'a, T: Display, E: Default, const SIZE: usize> {
    _t1: i32,
    _t: T,
    _e: E,
    _r: &'a u8,
    _a: [u8; SIZE],
}

struct A;

#[derive(New)]
struct B<'a, I>
where
    I: Iterator<Item = &'a A>,
{
    i: I,
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let i = 5;
    let g = GenericsWithBound::new(5, 7.0f32, 9u8, &i, [6; 5]);
    dbg!(g);
    let v = [A, A];
    let _ = B::new(v.iter(), 45, 1, 9);
}
