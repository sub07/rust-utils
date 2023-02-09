use crate::number::Number;

#[derive(Debug, Clone)]
pub struct Vector<T: Number, const SIZE: usize>([T; SIZE]);

impl<T: Number, const SIZE: usize> Vector<T, SIZE> {
    pub fn from(values: [T; SIZE]) -> Vector<T, SIZE> {
        Vector(values)
    }

    pub fn as_slice(&self) -> &[T; SIZE] {
        &self.0
    }
}

impl <T: Number> Vector<T, 2> {
    pub fn new(x: T, y: T) -> Vector<T, 2> { Vector([x, y]) }
    pub fn x(&self) -> T { self.0[0] }
    pub fn y(&self) -> T { self.0[1] }
}

impl <T: Number> Vector<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Vector<T, 3> { Vector([x, y, z]) }
    pub fn x(&self) -> T { self.0[0] }
    pub fn y(&self) -> T { self.0[1] }
    pub fn z(&self) -> T { self.0[2] }
}

macro_rules! enable_multiple_args {
    ($macro_name:ident,) => ();
    ($macro_name:ident, $op:tt $($other:tt)*) => {
        $macro_name!($op);
        enable_multiple_args!($macro_name, $($other)*);
    };
}

macro_rules! emit_vec_binary_op_impl {
    ($trait_name:ident, $fn_name:ident, |$left:ident:$impl_ty:ty, $right:ident:$rhs_ty:ty| -> $output_ty:ty $fn_body:block) => {
        impl <T: Number, const SIZE: usize> std::ops::$trait_name<$rhs_ty> for $impl_ty {
            type Output = $output_ty;

            fn $fn_name(self, rhs: $rhs_ty) -> Self::Output {
                |$left:$impl_ty, $right:$rhs_ty| -> $output_ty { $fn_body }(self, rhs)
            }
        }
    };
}

macro_rules! emit_vec_assign_op_impl {
    ($trait_name:ident, $fn_name:ident, |$left:ident: &mut $impl_ty:ty, $right:ident:$rhs_ty:ty| $fn_body:block) => {
        impl <T: Number, const SIZE: usize> std::ops::$trait_name<$rhs_ty> for $impl_ty {
            #[allow(clippy::redundant_closure_call)]
            fn $fn_name(&mut self, rhs: $rhs_ty) {
                (|$left:&mut $impl_ty, $right:$rhs_ty| { $fn_body })(self, rhs)
            }
        }
    };
}

macro_rules! impl_vec_op_vec {
    ($op:tt) => (parse_op!($op, impl_vec_op_vec););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_vec!($op, $trait_name, $fn_name, Vector<T, SIZE>, Vector<T, SIZE>);
        impl_vec_op_vec!($op, $trait_name, $fn_name, &Vector<T, SIZE>, Vector<T, SIZE>);
        impl_vec_op_vec!($op, $trait_name, $fn_name, Vector<T, SIZE>, &Vector<T, SIZE>);
        impl_vec_op_vec!($op, $trait_name, $fn_name, &Vector<T, SIZE>, &Vector<T, SIZE>);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty1:ty, $ty2:ty) => {
        emit_vec_binary_op_impl!($trait_name, $fn_name, |l: $ty1, r: $ty2| -> Vector<T, SIZE> { Vector(l.0.zip(r.0).map(|(l, r)| l $op r)) });
    };
}

macro_rules! impl_vec_op_num {
    ($op:tt) => (parse_op!($op, impl_vec_op_num););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_num!($op, $trait_name, $fn_name, Vector<T, SIZE>);
        impl_vec_op_num!($op, $trait_name, $fn_name, &Vector<T, SIZE>);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty:ty) => {
        emit_vec_binary_op_impl!($trait_name, $fn_name, |l: $ty, r: T| -> Vector<T, SIZE> { Vector(l.0.map(|v| v $op r)) });
    };
}

macro_rules! impl_vec_op_assign_vec {
    ($op:tt) => (parse_op!(assign, $op, impl_vec_op_assign_vec););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_assign_vec!($op, $trait_name, $fn_name, Vector<T, SIZE>, Vector<T, SIZE>);
        impl_vec_op_assign_vec!($op, $trait_name, $fn_name, Vector<T, SIZE>, &Vector<T, SIZE>);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty1:ty, $ty2:ty) => {
        emit_vec_assign_op_impl!($trait_name, $fn_name, |l: &mut $ty1, r: $ty2| { for i in 0..SIZE { l.0[i] = l.0[i] $op r.0[i]; } });
    };
}

macro_rules! impl_vec_op_assign_num {
    ($op:tt) => (parse_op!(assign, $op, impl_vec_op_assign_num););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_assign_num!($op, $trait_name, $fn_name, Vector<T, SIZE>);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty1:ty) => {
        emit_vec_assign_op_impl!($trait_name, $fn_name, |l: &mut $ty1, r: T| { for i in 0..SIZE { l.0[i] = l.0[i] $op r; } });
    };
}

macro_rules! parse_op {
    (assign, +=, $macro_name:ident) => ($macro_name!(+, AddAssign, add_assign););
    (assign, -=, $macro_name:ident) => ($macro_name!(-, SubAssign, sub_assign););
    (assign, *=, $macro_name:ident) => ($macro_name!(*, MulAssign, mul_assign););
    (assign, /=, $macro_name:ident) => ($macro_name!(/, DivAssign, div_assign););
    (+, $macro_name:ident) => ($macro_name!(+, Add, add););
    (-, $macro_name:ident) => ($macro_name!(-, Sub, sub););
    (*, $macro_name:ident) => ($macro_name!(*, Mul, mul););
    (/, $macro_name:ident) => ($macro_name!(/, Div, div););
}

enable_multiple_args!(impl_vec_op_vec, + - * /);
enable_multiple_args!(impl_vec_op_num, + - * /);
enable_multiple_args!(impl_vec_op_assign_vec, += -= *= /=);
enable_multiple_args!(impl_vec_op_assign_num, += -= *= /=);
