use crate::number::Number;

#[derive(Debug)]
pub struct Vector<T: Number, const SIZE: usize>([T; SIZE]);

impl<T: Number, const SIZE: usize> Vector<T, SIZE> {
    pub fn from(values: [T; SIZE]) -> Vector<T, SIZE> {
        Vector(values)
    }
}

macro_rules! impl_vec_op {
    (+, )
    ($trait_name:ident, $fn_name:ident, $impl_ty:ty, $rhs_ty:ty, $output_ty:ty, $($fn_body:tt)+) => {
        impl <T: Number, const SIZE: usize> std::ops::$trait_name<$rhs_ty> for $impl_ty {
            type Output = $output_ty;

            fn $fn_name(self, rhs: Vector<T, SIZE>) -> Self::Output {
                $($fn_body)+
            }
        }
    };
}

impl_vec_op!(Add, add, Vector<T, SIZE>, Vector<T, SIZE>, Vector<T, SIZE>, Vector(self.0.zip(rhs.0).map(|(left, right)| left + right)));
// impl_vec_op!(Sub, sub, |(l, r)| l - r);
// impl_vec_op!(Mul, mul, |(l, r)| l * r);
// impl_vec_op!(Div, div, |(l, r)| l / r);
