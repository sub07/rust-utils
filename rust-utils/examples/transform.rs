use std::{convert::Infallible, fmt::{Debug, Display, Write}, marker::PhantomData};

#[derive(Debug, Copy, Clone)]
struct A;

#[derive(Debug, Copy, Clone)]
struct B;

#[derive(Debug, Copy, Clone)]
struct C;

pub trait ConvertInto<O> {
    fn convert_into(self) -> O;
}

pub trait TryConvertInto<O> {
    type Error;

    fn try_convert_into(self) -> Result<O, Self::Error>;
}

impl ConvertInto<B> for A {
    fn convert_into(self) -> B {
        B
    }
}

impl ConvertInto<C> for B {
    fn convert_into(self) -> C {
        C
    }
}

impl ConvertInto<B> for i32 {
    fn convert_into(self) -> B {
        B
    }
}

// impl<T: ConvertInto<T>> TryConvertInto<T> for T {
//     type Error = Infallible;

//     fn try_convert_into(self) -> Result<T, Self::Error> {
//         Ok(self.convert_into())
//     }
// }

// #[macro_export]
// macro_rules! convert {
//     ($input:tt -> $($chain:ident)->*) => {
//         convert!(@internal $input < [$($chain) *])
//     };
//     (@internal $input:tt < [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
//         convert!(@internal $input < [$($rest)*] $first $($reversed)*)
//     };
//     (@internal $input:tt < [] $($reversed:tt)*) => {
//         convert!(@internal $input | $($reversed),*)
//     };
//     (@internal $input:tt | $output_ty:ident, $($rest_output:ident),+) => {
//         ConvertInto::<$output_ty>::convert_into(convert!(@internal $input | $($rest_output),+))
//     };
//     (@internal $input:tt | $output_ty:ident) => {
//         ConvertInto::<$output_ty>::convert_into($input)
//     };
// }

// pub fn try_convert_into_helper<F, I, E>(from: Result<F, E>) -> Result<I, E>
// where
//     F: TryConvertInto<I, Error = E>,
// {
//     TryConvertInto::<I>::try_convert_into(from?)
// }

// #[macro_export]
// macro_rules! try_into {
//     ($input:tt -> $($chain:ident)->*) => {
//         try_into!(@internal $input < [$($chain) *])
//     };
//     (@internal $input:tt < [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
//         try_into!(@internal $input < [$($rest)*] $first $($reversed)*)
//     };
//     (@internal $input:tt < [] $($reversed:tt)*) => {
//         try_into!(@internal $input | $($reversed),*)
//     };
//     (@internal $input:tt | $output_ty:ident, $($rest_output:ident),+) => {
//         $crate::transform::try_into_helper::<_, $output_ty, _>(try_into!(@internal $input | $($rest_output),+))
//     };
//     (@internal $input:tt | $output_ty:ident) => {
//         $crate::transform::try_into_helper::<_, $output_ty, _>(Ok($input))
//     };
// }

fn main() {
    // let a = A;

    // let _c = convert!(a -> B -> C);
    // let _c = convert!(2i32 -> B -> C);

    // let _c = try_into!(A -> B -> C);
    // let _c = try_into!(a -> B -> C);
    // let _c = try_into!(2i32 -> B -> C);

    // let b = try_convert_into_helper::<_, B, _>(Ok(A));
}
