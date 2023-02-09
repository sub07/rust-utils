#![feature(box_syntax)]

use rust_utils::vector::Vector;

fn main() {
    let pos1 = Vector::from([5; 2]);
    let pos2 = Vector::from([6; 2]);
    let sum = pos1 + pos2;

    dbg!(sum);
    // dbg!(pos1);
    // dbg!(pos2);
}
