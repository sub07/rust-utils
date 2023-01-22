use rust_utils_macro::{New};

#[derive(Debug, New)]
struct T;

#[derive(New, Debug)]
struct Test {
    #[new_default] f1: u64,
    f2: String,
    f3: T,
}

fn main() {
    let test = Test::new("test".into(), T::new());
    dbg!(test);
}