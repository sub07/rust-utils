use joy_macro::DisplayFromDebug;
use std::fmt::Debug;

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

#[derive(Debug, DisplayFromDebug)]
enum B<T: Debug> {
    Var1,
    Var2(T),
}

fn main() {
    let a = A(8);

    println!("{:?}", a);
    println!("{}", a);

    println!("{}", B::<u32>::Var2(45));
}
