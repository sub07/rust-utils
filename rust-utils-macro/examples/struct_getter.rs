use rust_utils_macro::{Getter};

#[derive(Debug, Clone, Copy)]
struct T;

#[derive(Debug, Getter)]
struct Test {
    f1: (u64, u64, i32),
    f2: String,
    #[getter_force_copy] f3: T,
    #[no_getter] f4: T,
}

fn main() {
    let test = Test { f1: (45, 65, 23), f2: "test".into(), f3: T, f4: T };
    dbg!(test.f1());
    dbg!(test.f2());
    dbg!(test.f3());
    // dbg!(test.f4());
    dbg!(test.f4);
}
