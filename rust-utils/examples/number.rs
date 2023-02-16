use rust_utils::number::{Number, IntoPrimitive};

fn f<N: Number>(_: N) -> impl Number {
    5
}

fn f1(i: i32) {
    println!("{}", i);
}

fn main() {
    f(6);
    // f(true);
    let i = 56u32;
    f1(i.into_p());
    let i = 56i8;
    f1(i.into_p());
    let i = 56f32;
    f1(i.into_p());
}
