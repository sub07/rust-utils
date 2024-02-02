use std::convert::Infallible;

use rust_utils::{into, try_into};

#[derive(Debug, Copy, Clone)]
struct A;

#[derive(Debug, Copy, Clone)]
struct B;

#[derive(Debug, Copy, Clone)]
struct C;

impl From<A> for B {
    fn from(_value: A) -> Self {
        B
    }
}

impl From<B> for C {
    fn from(_value: B) -> Self {
        C
    }
}

impl From<i32> for B {
    fn from(_value: i32) -> Self {
        B
    }
}

fn main() {
    let a = A;

    let _c = into!(a -> B -> C);
    let _c = into!(2i32 -> B -> C);

    let _c = try_into!(A -> B -> C);
    let _c = try_into!(a -> B -> C);
    let _c = try_into!(2i32 -> B -> C);
}
