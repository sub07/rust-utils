use rust_utils::number::Number;

fn f<N: Number>(_: N) -> impl Number {
    5
}

fn main() {
    f(6);
    // f(true);
}
