use rust_utils::hash_map_of;

fn main() {
    let hash_map = hash_map_of!(
        0 => "zero",
        1 => "one",
        2 => "two",
    );

    dbg!(hash_map);
}