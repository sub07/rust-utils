use std::fmt::{Debug, Display};

use joy_macro::DisplayFromDebug;

#[derive(Debug, DisplayFromDebug)]
struct A(u32);

#[derive(Debug, DisplayFromDebug)]
struct S<T: Send + Debug, I>
where
    I: Iterator<Item = u8> + Debug,
{
    t: T,
    i: I,
}

fn main() {
    let a = A(8);

    println!("{:?}", a);
    println!("{}", a);
}
