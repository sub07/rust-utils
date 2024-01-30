#[macro_export]
macro_rules! into {
    ($input:tt -> $($chain:ident)->*) => {
        into!(@internal $input < [$($chain) *])
    };
    (@internal $input:tt < [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
        into!(@internal $input < [$($rest)*] $first $($reversed)*)
    };
    (@internal $input:tt < [] $($reversed:tt)*) => {
        into!(@internal $input | $($reversed),*)
    };
    (@internal $input:tt | $output_ty:ident, $($rest_output:ident),+) => {
        Into::<$output_ty>::into(into!(@internal $input | $($rest_output),+))
    };
    (@internal $input:tt | $output_ty:ident) => {
        Into::<$output_ty>::into($input)
    };
}
