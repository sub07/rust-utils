use rust_utils::number::{Number, IntoPrimitive};

fn f<N: Number>(_: N) -> impl Number {
    5
}

fn f1(i: i32) {
    println!("{}", i);
}

fn main() {
    f(6);
}
