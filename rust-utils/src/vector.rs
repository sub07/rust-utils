use crate::number::Number;

#[derive(Debug)]
pub struct Vector<T: Number, const SIZE: usize>([T; SIZE]);

impl <T: Number, const SIZE: usize> Vector<T, SIZE> {
    pub fn from(values: [T; SIZE]) -> Vector<T, SIZE> {
        Vector(values)
    }

    pub fn as_slice(&self) -> &[T; SIZE] {
        &self.0
    }
}

macro_rules! impl_vec_ops_binary {
    () => ();
    ($op:tt $($other:tt)*) => {
      impl_vec_op_binary!($op);
      impl_vec_ops_binary!($($other)*);
    };
}

macro_rules! impl_vec_op_binary {
    (+) => (impl_vec_op_binary!(+, Add, add););
    (-) => (impl_vec_op_binary!(-, Sub, sub););
    (*) => (impl_vec_op_binary!(*, Mul, mul););
    (/) => (impl_vec_op_binary!(/, Div, div););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_binary!($op, $trait_name, $fn_name, Vector<T, SIZE>, Vector<T, SIZE>);
        impl_vec_op_binary!($op, $trait_name, $fn_name, &Vector<T, SIZE>, Vector<T, SIZE>);
        impl_vec_op_binary!($op, $trait_name, $fn_name, Vector<T, SIZE>, &Vector<T, SIZE>);
        impl_vec_op_binary!($op, $trait_name, $fn_name, &Vector<T, SIZE>, &Vector<T, SIZE>);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty1:ty, $ty2:ty) => {
        impl_vec_op_binary!($trait_name, $fn_name, |l: $ty1, r: $ty2| -> Vector<T, SIZE> { Vector(l.0.zip(r.0).map(|(l, r)| l $op r)) });
    };
    ($trait_name:ident, $fn_name:ident, |$left:ident:$impl_ty:ty, $right:ident:$rhs_ty:ty| -> $output_ty:ty $fn_body:block) => {
        impl <T: Number, const SIZE: usize> std::ops::$trait_name<$rhs_ty> for $impl_ty {
            type Output = $output_ty;

            fn $fn_name(self, rhs: $rhs_ty) -> Self::Output {
                |$left:$impl_ty, $right:$rhs_ty| -> $output_ty { $fn_body }(self, rhs)
            }
        }
    };
}

impl_vec_ops_binary!(+ - * /);
