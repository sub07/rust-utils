use std::convert::Infallible;

use joy_error::ResultInfallibleExt;

fn main() {
    let res = Ok::<i32, Infallible>(42).unwrap_infallible();
    assert_eq!(42, res);
}
