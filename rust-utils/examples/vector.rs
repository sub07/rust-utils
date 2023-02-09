use rust_utils::vector::Vector;

fn main() {
    let pos1 = Vector::from([5; 3]);
    let pos2 = Vector::from([6; 3]);
    let sum = &pos1 + &pos2 * &pos1;

    dbg!(sum);
    dbg!(pos1);
    dbg!(pos2);
}
