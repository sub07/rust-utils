use crate::number::Number;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
pub struct Vector<T: Number, const SIZE: usize>([T; SIZE]);

impl<T: Number, const SIZE: usize> Vector<T, SIZE> {
    pub fn from(values: [T; SIZE]) -> Vector<T, SIZE> {
        Vector(values)
    }
}

// macro_rules! impl_vec_op {
//     ($trait_name:ident,$fn_name:ident,$op:expr) => {
//         impl <T: Number, const SIZE: usize> $trait_name<Vector<T, SIZE>> for Vector<T, SIZE> {
//             type Output = Vector<T, SIZE>;
//
//             fn $fn_name(self, rhs: Vector<T, SIZE>) -> Self::Output {
//                 Vector(self.0.zip(rhs.0).map($op))
//             }
//         }
//     }
// }

impl<T: Number, const SIZE: usize> Add<Vector<T, SIZE>> for Vector<T, SIZE> {
    type Output = Vector<T, SIZE>;

    fn add(self, rhs: Vector<T, SIZE>) -> Self::Output {
        Vector(self.0.zip(rhs.0).map(|(l, r)| l + r))
    }
}

// impl_vec_op!(Add, add, |(l, r)| l + r);
// impl_vec_op!(Sub, sub, |(l, r)| l - r);
// impl_vec_op!(Mul, mul, |(l, r)| l * r);
// impl_vec_op!(Div, div, |(l, r)| l / r);
