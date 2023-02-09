use rust_utils::vector::Vector;

fn main() {
    let mut pos1 = Vector::from([5.6; 3]);
    pos1.set::<4>(5.4);
    dbg!(&pos1);
    // pos1 *= &pos2;
    pos1 /= 6.0;
    dbg!(&pos1);
}
