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

pub fn try_into_helper<F, I, E>(from: Result<F, E>) -> Result<I, E>
where
    I: TryFrom<F, Error = E>,
    F: TryInto<I, Error = E>,
{
    TryInto::<I>::try_into(from?)
}

#[macro_export]
macro_rules! try_into {
    ($input:tt -> $($chain:ident)->*) => {
        try_into!(@internal $input < [$($chain) *])
    };
    (@internal $input:tt < [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
        try_into!(@internal $input < [$($rest)*] $first $($reversed)*)
    };
    (@internal $input:tt < [] $($reversed:tt)*) => {
        try_into!(@internal $input | $($reversed),*)
    };
    (@internal $input:tt | $output_ty:ident, $($rest_output:ident),+) => {
        $crate::transform::try_into_helper::<_, $output_ty, _>(try_into!(@internal $input | $($rest_output),+))
    };
    (@internal $input:tt | $output_ty:ident) => {
        $crate::transform::try_into_helper::<_, $output_ty, _>(Ok($input))
    };
}
