use joy_macro::DisplayFromDebug;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug, DisplayFromDebug)]
struct A(u32);

#[derive(Debug, DisplayFromDebug)]
#[allow(dead_code)]
enum B<T: Debug> {
    Var1,
    Var2(T),
}

#[allow(dead_code)]
#[derive(Debug, DisplayFromDebug)]
struct C<T: Send + Debug, I>
where
    I: Iterator<Item = u8> + Debug,
{
    t: T,
    i: I,
}

fn main() {
    let a = A(8);
    assert_eq!(format!("{a}"), format!("{a:?}"));

    let b = B::<u32>::Var2(45);
    assert_eq!(format!("{b}"), format!("{b:?}"));
}
