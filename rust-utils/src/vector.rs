use std::ops::{Index, IndexMut};

use crate::number::{DefaultConst, Number};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T: Number + ~const DefaultConst, const SIZE: usize>([T; SIZE]);

impl<T: Number + ~const DefaultConst, const SIZE: usize> Vector<T, SIZE> {
    pub const ZERO: Vector<T, SIZE> = Vector::zeros();
    pub const fn as_slice(&self) -> &[T; SIZE] { &self.0 }
    pub fn as_slice_mut(&mut self) -> &mut [T; SIZE] { &mut self.0 }
    pub const fn size(&self) -> usize { SIZE }
    pub const fn default_const() -> Vector<T, SIZE> { Vector::from([T::default_const(); SIZE]) }
    pub const fn zeros() -> Vector<T, SIZE> { Vector::default_const() }
    pub const fn init_with(initial_value: T) -> Vector<T, SIZE> { Vector::from([initial_value; SIZE]) }
}

impl <T: Number, const SIZE: usize> Default for Vector<T, SIZE> {
    fn default() -> Self {
        Vector::default_const()
    }
}

impl<T: Number + ~const DefaultConst> Vector<T, 2> {
    pub const fn new(x: T, y: T) -> Vector<T, 2> { Vector([x, y]) }
    pub fn x(&self) -> T { self[0] }
    pub fn y(&self) -> T { self[1] }
    pub fn set_x(&mut self, new_x: T) { self[0] = new_x }
    pub fn set_y(&mut self, new_y: T) { self[1] = new_y; }
}

impl<T: Number + ~const DefaultConst> Vector<T, 3> {
    pub const fn new(x: T, y: T, z: T) -> Vector<T, 3> { Vector([x, y, z]) }
    pub fn x(&self) -> T { self[0] }
    pub fn y(&self) -> T { self[1] }
    pub fn z(&self) -> T { self[2] }
    pub fn set_x(&mut self, new_x: T) { self[0] = new_x; }
    pub fn set_y(&mut self, new_y: T) { self[1] = new_y; }
    pub fn set_z(&mut self, new_z: T) { self[2] = new_z; }
}

impl<T: Number + ~const DefaultConst, const SIZE: usize> const From<[T; SIZE]> for Vector<T, SIZE> {
    fn from(values: [T; SIZE]) -> Self {
        Vector(values)
    }
}

impl<T: Number + ~const DefaultConst, const SIZE: usize> const From<&[T; SIZE]> for Vector<T, SIZE> {
    fn from(values: &[T; SIZE]) -> Self {
        Vector(*values)
    }
}

impl<T: Number + ~const DefaultConst, const SIZE: usize> const From<&Vector<T, SIZE>> for Vector<T, SIZE> {
    fn from(value: &Vector<T, SIZE>) -> Self { *value }
}

impl<T: Number, const SIZE: usize> Index<usize> for Vector<T, SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Number, const SIZE: usize> IndexMut<usize> for Vector<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! enable_multiple_args {
    ($macro_name:ident,) => ();
    ($macro_name:ident, $op:tt $($other:tt)*) => {
        $macro_name!($op);
        enable_multiple_args!($macro_name, $($other)*);
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

macro_rules! impl_vec_op_slice {
    ($op:tt) => (parse_op!($op, impl_vec_op_slice););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_slice!($op, $trait_name, $fn_name, Vector<T, SIZE>, [T; SIZE]);
        impl_vec_op_slice!($op, $trait_name, $fn_name, &Vector<T, SIZE>, [T; SIZE]);
        impl_vec_op_slice!($op, $trait_name, $fn_name, Vector<T, SIZE>, &[T; SIZE]);
        impl_vec_op_slice!($op, $trait_name, $fn_name, &Vector<T, SIZE>, &[T; SIZE]);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty1:ty, $ty2:ty) => {
        emit_vec_binary_op_impl!($trait_name, $fn_name, |l: $ty1, r: $ty2| -> Vector<T, SIZE> { l $op Vector::from(r) });
    };
}

macro_rules! impl_vec_op_assign_vec {
    ($op:tt) => (parse_op!(assign, $op, impl_vec_op_assign_vec););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_assign_vec!($op, $trait_name, $fn_name, Vector<T, SIZE>);
        impl_vec_op_assign_vec!($op, $trait_name, $fn_name, &Vector<T, SIZE>);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty:ty) => {
        emit_vec_assign_op_impl!($trait_name, $fn_name, |l: &mut Vector<T, SIZE>, r: $ty| { for i in 0..SIZE { l.0[i] = l.0[i] $op r.0[i]; } });
    };
}

macro_rules! impl_vec_op_assign_num {
    ($op:tt) => (parse_op!(assign, $op, impl_vec_op_assign_num););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_assign_num!($op, $trait_name, $fn_name, Vector<T, SIZE>);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty:ty) => {
        emit_vec_assign_op_impl!($trait_name, $fn_name, |l: &mut $ty, r: T| { for i in 0..SIZE { l.0[i] = l.0[i] $op r; } });
    };
}

macro_rules! impl_vec_op_assign_slice {
    ($op:tt) => (parse_op!(assign, $op, impl_vec_op_assign_slice););
    ($op:tt, $trait_name:ident, $fn_name:ident) => {
        impl_vec_op_assign_slice!($op, $trait_name, $fn_name, [T; SIZE]);
        impl_vec_op_assign_slice!($op, $trait_name, $fn_name, &[T; SIZE]);
    };
    ($op:tt, $trait_name:ident, $fn_name:ident, $ty:ty) => {
        emit_vec_assign_op_impl!($trait_name, $fn_name, |l: &mut Vector<T, SIZE>, r: $ty| { for i in 0..SIZE { l.0[i] = l.0[i] $op r[i]; } });
    };
}


enable_multiple_args!(impl_vec_op_vec, + - * /);
enable_multiple_args!(impl_vec_op_num, + - * /);
enable_multiple_args!(impl_vec_op_slice, + - * /);

enable_multiple_args!(impl_vec_op_assign_vec, += -= *= /=);
enable_multiple_args!(impl_vec_op_assign_num, += -= *= /=);
enable_multiple_args!(impl_vec_op_assign_slice, += -= *= /=);
