use rust_utils::iter::zip_self::ZipSelf;

fn main() {
    let a = [5, 6, 8];
    let expected = vec![5, 5, 5, 6, 6, 6, 8, 8, 8];
    let res = a.into_iter().zip_self(3).collect::<Vec<_>>();
    assert_eq!(res, expected);
}
