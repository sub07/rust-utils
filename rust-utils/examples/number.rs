use rust_utils::number::EpsilonEq;

fn main() {
    let f1 = 0.002;
    let f2 = 0.003;

    assert_ne!(f1, f2);
    assert!(f1.epsilon_eq(&f2, 0.003));
    assert!(f2.epsilon_eq(&f1, 0.003));
    assert!(!f2.epsilon_eq(&f1, 0.0005));
    assert!(!f1.epsilon_eq(&f2, 0.0005));
}
