use std::ops::{Add, Deref, DerefMut, Div, Index, IndexMut, Mul, Sub};

pub trait Number:
    Copy
    + PartialEq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Default
{
}

impl<T> Number for T where
    T: Copy
        + PartialEq
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Default
{
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const SIZE: usize>(pub [T; SIZE]);

impl<T, const SIZE: usize> Vector<T, SIZE> {
    pub const fn as_slice(&self) -> &[T; SIZE] {
        &self.0
    }
    pub fn as_slice_mut(&mut self) -> &mut [T; SIZE] {
        &mut self.0
    }
    pub const fn size(&self) -> usize {
        SIZE
    }
}

impl<T: Number, const SIZE: usize> Default for Vector<T, SIZE> {
    fn default() -> Self {
        T::default().into()
    }
}

impl<T: Number> Vector<T, 2> {
    pub const fn new(x: T, y: T) -> Vector<T, 2> {
        Vector([x, y])
    }
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
    pub fn w(&self) -> T {
        self.x()
    }
    pub fn h(&self) -> T {
        self.y()
    }
    pub fn set_x(&mut self, new_x: T) {
        self[0] = new_x
    }
    pub fn set_y(&mut self, new_y: T) {
        self[1] = new_y;
    }
}

impl<T: Number> Vector<T, 3> {
    pub const fn new(x: T, y: T, z: T) -> Vector<T, 3> {
        Vector([x, y, z])
    }
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
    pub fn z(&self) -> T {
        self[2]
    }
    pub fn set_x(&mut self, new_x: T) {
        self[0] = new_x;
    }
    pub fn set_y(&mut self, new_y: T) {
        self[1] = new_y;
    }
    pub fn set_z(&mut self, new_z: T) {
        self[2] = new_z;
    }
}

#[macro_export]
macro_rules! vector {
    [$($val:expr),+ $(,)?] => { $crate::Vector::from([$($val),+]) };
}

impl<T, const SIZE: usize> From<[T; SIZE]> for Vector<T, SIZE> {
    fn from(values: [T; SIZE]) -> Self {
        Vector(values)
    }
}

impl<T: Clone, const SIZE: usize> From<&[T; SIZE]> for Vector<T, SIZE> {
    fn from(values: &[T; SIZE]) -> Self {
        Vector(values.clone())
    }
}

impl<T: Number, const SIZE: usize> From<T> for Vector<T, SIZE> {
    fn from(value: T) -> Self {
        Vector([value; SIZE])
    }
}

impl<T, const SIZE: usize> From<Vector<T, SIZE>> for [T; SIZE] {
    fn from(val: Vector<T, SIZE>) -> Self {
        val.0
    }
}

impl<T, const SIZE: usize> Index<usize> for Vector<T, SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T, const SIZE: usize> IndexMut<usize> for Vector<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl<T, const SIZE: usize> Deref for Vector<T, SIZE> {
    type Target = [T; SIZE];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const SIZE: usize> DerefMut for Vector<T, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

/*
   Impl | vec   op  vec
        | &vec  op  vec
        | vec   op  &vec
        | &vec  op  &vec
*/

macro_rules! impl_op_vec_vec {
    ($op:tt, $trait_name:ident, $trait_fn:ident, $lhs_ty:ty, $rhs_ty:ty) => {
        impl <T: Number, const SIZE: usize> std::ops::$trait_name<$rhs_ty> for $lhs_ty {
            type Output = Vector<T, SIZE>;

            fn $trait_fn(self, rhs: $rhs_ty) -> Self::Output {
                core::array::from_fn(|i| self[i] $op rhs[i]).into()
            }
        }
    };
}

macro_rules! gen_op_vec_vec_ref {
    ($op:tt, $trait_name:ident, $trait_fn:ident) => {
        impl_op_vec_vec!($op, $trait_name, $trait_fn, Vector<T, SIZE>, Vector<T, SIZE>);
        impl_op_vec_vec!($op, $trait_name, $trait_fn, &Vector<T, SIZE>, Vector<T, SIZE>);
        impl_op_vec_vec!($op, $trait_name, $trait_fn, Vector<T, SIZE>, &Vector<T, SIZE>);
        impl_op_vec_vec!($op, $trait_name, $trait_fn, &Vector<T, SIZE>, &Vector<T, SIZE>);
    };
}

gen_op_vec_vec_ref!(+, Add, add);
gen_op_vec_vec_ref!(-, Sub, sub);
gen_op_vec_vec_ref!(*, Mul, mul);
gen_op_vec_vec_ref!(/, Div, div);

/*
   Impl | num   op  vec
        | num   op  &vec
*/

macro_rules! impl_op_num_vec {
    ($op:tt, $trait_name:ident, $trait_fn:ident, $num_ty:ty) => {
        impl_op_num_vec!($op, $trait_name, $trait_fn, $num_ty, Vector<$num_ty, SIZE>);
        impl_op_num_vec!($op, $trait_name, $trait_fn, $num_ty, &Vector<$num_ty, SIZE>);
    };
    ($op:tt, $trait_name:ident, $trait_fn:ident, $num_ty:ty, $vec_ty:ty) => {
        impl <const SIZE: usize> std::ops::$trait_name<$vec_ty> for $num_ty {
            type Output = Vector<$num_ty, SIZE>;

            fn $trait_fn(self, rhs: $vec_ty) -> Self::Output {
                core::array::from_fn(|i| self $op rhs[i]).into()
            }
        }
    };
}

macro_rules! gen_op_num_vec {
    ($num_ty:tt $($other_num_ty:tt) *) => {
        impl_op_num_vec!(+, Add, add, $num_ty);
        impl_op_num_vec!(-, Sub, sub, $num_ty);
        impl_op_num_vec!(*, Mul, mul, $num_ty);
        impl_op_num_vec!(/, Div, div, $num_ty);
        gen_op_num_vec!($($other_num_ty) *);
    };
    () => {};
}

gen_op_num_vec!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64);

/*
   Impl | vec   op  num
        | &vec   op  num
*/

macro_rules! impl_op_vec_num {
    ($op:tt, $trait_name:ident, $trait_fn:ident, $vec_ty:ty) => {
        impl <T: Number, const SIZE: usize> std::ops::$trait_name<T> for $vec_ty {
            type Output = Vector<T, SIZE>;

            fn $trait_fn(self, rhs: T) -> Self::Output {
                core::array::from_fn(|i| self[i] $op rhs).into()
            }
        }
    };
}

macro_rules! gen_op_vec_num {
    ($op:tt, $trait_name:ident, $trait_fn:ident) => {
        impl_op_vec_num!($op, $trait_name, $trait_fn, Vector<T, SIZE>);
        impl_op_vec_num!($op, $trait_name, $trait_fn, &Vector<T, SIZE>);
    };
}

gen_op_vec_num!(+, Add, add);
gen_op_vec_num!(-, Sub, sub);
gen_op_vec_num!(*, Mul, mul);
gen_op_vec_num!(/, Div, div);

/*
   Impl | vec op= vec
        | vec op= &vec
*/

macro_rules! impl_op_assign_vec_vec {
    ($op:tt, $trait_name:ident, $trait_fn:ident, $vec_ty:ty) => {
        impl<T: Number, const SIZE: usize> std::ops::$trait_name<$vec_ty> for Vector<T, SIZE> {
            fn $trait_fn(&mut self, rhs: $vec_ty) {
                for (a, b) in self.as_slice_mut().iter_mut().zip(rhs.as_slice().iter()) {
                    *a = *a $op *b;
                }
            }
        }
    };
}

macro_rules! gen_op_assign_vec_vec {
    ($op:tt, $trait_name:ident, $trait_fn:ident) => {
        impl_op_assign_vec_vec!($op, $trait_name, $trait_fn, Vector<T, SIZE>);
        impl_op_assign_vec_vec!($op, $trait_name, $trait_fn, &Vector<T, SIZE>);
    };
}

gen_op_assign_vec_vec!(+, AddAssign, add_assign);
gen_op_assign_vec_vec!(-, SubAssign, sub_assign);
gen_op_assign_vec_vec!(*, MulAssign, mul_assign);
gen_op_assign_vec_vec!(/, DivAssign, div_assign);

/*
   Impl vec op= num
*/

macro_rules! impl_op_assign_vec_num {
    ($op:tt, $trait_name:ident, $trait_fn:ident) => {
        impl<T: Number, const SIZE: usize> std::ops::$trait_name<T> for Vector<T, SIZE> {
            fn $trait_fn(&mut self, rhs: T) {
                for val in self.as_slice_mut().iter_mut() {
                    *val = *val $op rhs;
                }
            }
        }
    };
}

impl_op_assign_vec_num!(+, AddAssign, add_assign);
impl_op_assign_vec_num!(-, SubAssign, sub_assign);
impl_op_assign_vec_num!(*, MulAssign, mul_assign);
impl_op_assign_vec_num!(/, DivAssign, div_assign);

#[cfg(test)]
mod vec_tests {
    use super::*;

    #[test]
    fn from_slice() {
        let initial_slice = [5.0, 3.0, 2.0];
        let v = Vector::from(initial_slice);
        assert_eq!(v.as_slice(), &initial_slice);

        let initial_slice = [5.0, 3.0, 2.0];
        let v = Vector::from(&initial_slice);
        assert_eq!(v.as_slice(), &initial_slice);
    }

    #[test]
    fn from_number() {
        const SIZE: usize = 5;
        let initial_num = 5.0;
        let v: Vector<f64, SIZE> = Vector::from(initial_num);
        assert_eq!(v.size(), SIZE);
        for x in v.iter() {
            assert_eq!(*x, initial_num);
        }
    }

    #[test]
    fn test_destruct() {
        let mut vec = vector![10, 5, 6];
        let [_, _, _] = vec.into();
        let [_, _, _] = vec.as_slice();
        let [_, _, _] = vec.as_slice_mut();
        let Vector([_, _, _]) = vec;
    }

    #[test]
    fn test_vector_macro() {
        let e1 = 5;
        let e2 = 6;
        let e3 = 7;

        let v = vector![e1, e2, e3];

        assert_eq!(&[e1, e2, e3], v.as_slice());
    }

    #[test]
    fn test_vector_macro_trailing_comma() {
        let e1 = 5;
        let e2 = 6;
        let e3 = 7;

        let v = vector![e1, e2, e3,];

        assert_eq!(&[e1, e2, e3], v.as_slice());
    }

    #[test]
    fn add_vec_vec() {
        let v1 = vector![10, 20, 30];
        let v2 = vector![5, -5, -15];

        let res = v1 + v2;

        assert_eq!(res, [15, 15, 15].into())
    }

    #[test]
    fn sub_vec_vec() {
        let v1 = vector![10, 20, 30];
        let v2 = vector![5, -5, -15];

        let res = v1 - v2;

        assert_eq!(res, [5, 25, 45].into())
    }

    #[test]
    fn mul_vec_vec() {
        let v1 = vector![10, 20, 30];
        let v2 = vector![5, -5, -15];

        let res = v1 * v2;

        assert_eq!(res, [50, -100, -450].into())
    }

    #[test]
    fn div_vec_vec() {
        let v1 = vector![10, 20, 30];
        let v2 = vector![5, -5, -15];

        let res = v1 / v2;

        assert_eq!(res, [2, -4, -2].into())
    }

    #[test]
    fn add_num_vec() {
        let num = 5;
        let v = vector![10, 20, 30];

        let res = num + v;

        assert_eq!(res, [15, 25, 35].into())
    }

    #[test]
    fn sub_num_vec() {
        let num = 5;
        let v = vector![10, 20, 30];

        let res = num - v;

        assert_eq!(res, [-5, -15, -25].into())
    }

    #[test]
    fn mul_num_vec() {
        let num = 5;
        let v = vector![10, 20, 30];

        let res = num * v;

        assert_eq!(res, [50, 100, 150].into())
    }

    #[test]
    fn div_num_vec() {
        let num = 1000;
        let v = vector![10, 20, 30];

        let res = num / v;

        assert_eq!(res, [100, 50, 33].into())
    }

    #[test]
    fn add_vec_num() {
        let v = vector![10, 20, 30];
        let num = 5;

        let res = v + num;

        assert_eq!(res, [15, 25, 35].into())
    }

    #[test]
    fn sub_vec_num() {
        let v = vector![10, 20, 30];
        let num = 5;

        let res = v - num;

        assert_eq!(res, [5, 15, 25].into())
    }

    #[test]
    fn mul_vec_num() {
        let v = vector![10, 20, 30];
        let num = 5;

        let res = v * num;

        assert_eq!(res, [50, 100, 150].into())
    }

    #[test]
    fn div_vec_num() {
        let v = vector![1000, 2000, 3000];
        let num = 10;

        let res = v / num;

        assert_eq!(res, [100, 200, 300].into())
    }

    #[test]
    fn add_assign_vec_vec() {
        let mut v = vector![1, 2, 3, 4];
        let v2 = vector![2, 3, 4, 5];

        assert_eq!(v, vector![1, 2, 3, 4]);
        v += v2;
        assert_eq!(v, vector![3, 5, 7, 9]);
    }

    #[test]
    fn sub_assign_vec_vec() {
        let mut v = vector![1, 2, 3, 4];
        let v2 = vector![2, 3, 4, 5];

        assert_eq!(v, vector![1, 2, 3, 4]);
        v -= v2;
        assert_eq!(v, vector![-1, -1, -1, -1]);
    }

    #[test]
    fn mul_assign_vec_vec() {
        let mut v = vector![1, 2, 3, 4];
        let v2 = vector![2, 3, 4, 5];

        assert_eq!(v, vector![1, 2, 3, 4]);
        v *= v2;
        assert_eq!(v, vector![2, 6, 12, 20]);
    }

    #[test]
    fn div_assign_vec_vec() {
        let mut v = vector![4, 8, 16, 32];
        let v2 = vector![2, 3, 4, 5];

        assert_eq!(v, vector![4, 8, 16, 32]);
        v /= v2;
        assert_eq!(v, vector![2, 2, 4, 6]);
    }
}
