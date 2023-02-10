#![feature(const_trait_impl)]

use rust_utils::vector::Vector;

fn main() {
    const TEST: Vector<f64, 2> = Vector::from([4.0; 2]);
    let mut pos1 = Vector::from([5.0; 2]);
    const ZEROS: Vector<i32, 6> = Vector::default_const();
    let zeros: Vector<i32, 6> = Vector::default();
    const ONES: Vector<i32, 6> = Vector::init_with(1);
    dbg!(ZEROS);
    dbg!(zeros);
    dbg!(ONES);
    let [x, y] = pos1.as_slice();
    dbg!(x, y);
    dbg!(&pos1);
    // pos1 *= &pos2;
    pos1 /= TEST;
    dbg!(&pos1);
}
