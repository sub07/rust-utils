mod gen;

#[cfg(test)]
mod vec_tests {
    use crate::gen_vector;

    #[test]
    fn from_slice() {
        crate::gen_vector!(Vector<f32, 3>);

        let initial_slice = [5.0, 3.0, 2.0];
        let v = Vector::from(initial_slice);
        assert_eq!(v.as_slice(), &initial_slice);

        let initial_slice = [5.0, 3.0, 2.0];
        let v = Vector::from(&initial_slice);
        assert_eq!(v.as_slice(), &initial_slice);
    }

    #[test]
    fn from_number() {
        crate::gen_vector!(Vector<f64, 5>);
        let initial_num = 5.0;
        let v: Vector = Vector::from(initial_num);
        for x in v.iter() {
            approx::assert_relative_eq!(*x, initial_num, epsilon = 0.01);
        }
    }

    #[test]
    fn test_destruct() {
        crate::gen_vector!(Vector<i32, 3>);
        let mut vec = Vector([10, 5, 6]);
        let [_, _, _] = vec.into();
        let [_, _, _] = vec.as_slice();
        let [_, _, _] = vec.as_slice_mut();
        let Vector([_, _, _]) = vec;
    }

    #[test]
    fn test_vector_macro() {
        crate::gen_vector!(Vector<i32, 3>);
        let e1 = 5;
        let e2 = 6;
        let e3 = 7;

        let v = Vector([e1, e2, e3]);

        assert_eq!(&[e1, e2, e3], v.as_slice());
    }

    #[test]
    fn test_vector_macro_trailing_comma() {
        crate::gen_vector!(Vector<i32, 3>);
        let e1 = 5;
        let e2 = 6;
        let e3 = 7;

        let v = Vector([e1, e2, e3]);

        assert_eq!(&[e1, e2, e3], v.as_slice());
    }

    #[test]
    fn add_vec_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let v1 = Vector([10, 20, 30]);
        let v2 = Vector([5, -5, -15]);

        let res = v1 + v2;

        assert_eq!(res, [15, 15, 15].into());
    }

    #[test]
    fn sub_vec_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let v1 = Vector([10, 20, 30]);
        let v2 = Vector([5, -5, -15]);

        let res = v1 - v2;

        assert_eq!(res, [5, 25, 45].into());
    }

    #[test]
    fn mul_vec_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let v1 = Vector([10, 20, 30]);
        let v2 = Vector([5, -5, -15]);

        let res = v1 * v2;

        assert_eq!(res, [50, -100, -450].into());
    }

    #[test]
    fn div_vec_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let v1 = Vector([10, 20, 30]);
        let v2 = Vector([5, -5, -15]);

        let res = v1 / v2;

        assert_eq!(res, [2, -4, -2].into());
    }

    #[test]
    fn add_num_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let num = 5;
        let v = Vector([10, 20, 30]);

        let res = num + v;

        assert_eq!(res, [15, 25, 35].into());
    }

    #[test]
    fn sub_num_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let num = 5;
        let v = Vector([10, 20, 30]);

        let res = num - v;

        assert_eq!(res, [-5, -15, -25].into());
    }

    #[test]
    fn mul_num_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let num = 5;
        let v = Vector([10, 20, 30]);

        let res = num * v;

        assert_eq!(res, [50, 100, 150].into());
    }

    #[test]
    fn div_num_vec() {
        crate::gen_vector!(Vector<i32, 3>);
        let num = 1000;
        let v = Vector([10, 20, 30]);

        let res = num / v;

        assert_eq!(res, [100, 50, 33].into());
    }

    #[test]
    fn add_vec_num() {
        crate::gen_vector!(Vector<i32, 3>);
        let v = Vector([10, 20, 30]);
        let num = 5;

        let res = v + num;

        assert_eq!(res, [15, 25, 35].into());
    }

    #[test]
    fn sub_vec_num() {
        crate::gen_vector!(Vector<i32, 3>);
        let v = Vector([10, 20, 30]);
        let num = 5;

        let res = v - num;

        assert_eq!(res, [5, 15, 25].into());
    }

    #[test]
    fn mul_vec_num() {
        crate::gen_vector!(Vector<i32, 3>);
        let v = Vector([10, 20, 30]);
        let num = 5;

        let res = v * num;

        assert_eq!(res, [50, 100, 150].into());
    }

    #[test]
    fn div_vec_num() {
        crate::gen_vector!(Vector<i32, 3>);
        let v = Vector([1000, 2000, 3000]);
        let num = 10;

        let res = v / num;

        assert_eq!(res, [100, 200, 300].into());
    }

    #[test]
    fn add_assign_vec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = Vector([1, 2, 3, 4]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(v, Vector([1, 2, 3, 4]));
        v += v2;
        assert_eq!(v, Vector([3, 5, 7, 9]));
    }

    #[test]
    fn sub_assign_vec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = Vector([1, 2, 3, 4]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(v, Vector([1, 2, 3, 4]));
        v -= v2;
        assert_eq!(v, Vector([-1, -1, -1, -1]));
    }

    #[test]
    fn mul_assign_vec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = Vector([1, 2, 3, 4]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(v, Vector([1, 2, 3, 4]));
        v *= v2;
        assert_eq!(v, Vector([2, 6, 12, 20]));
    }

    #[test]
    fn div_assign_vec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = Vector([4, 8, 16, 32]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(v, Vector([4, 8, 16, 32]));
        v /= v2;
        assert_eq!(v, Vector([2, 2, 4, 6]));
    }

    #[test]
    fn add_assign_refmutvec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = &mut Vector([1, 2, 3, 4]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(*v, Vector([1, 2, 3, 4]));
        v += v2;
        assert_eq!(*v, Vector([3, 5, 7, 9]));
    }

    #[test]
    fn sub_assign_refmutvec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = &mut Vector([1, 2, 3, 4]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(*v, Vector([1, 2, 3, 4]));
        v -= v2;
        assert_eq!(*v, Vector([-1, -1, -1, -1]));
    }

    #[test]
    fn mul_assign_refmutvec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = &mut Vector([1, 2, 3, 4]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(*v, Vector([1, 2, 3, 4]));
        v *= v2;
        assert_eq!(*v, Vector([2, 6, 12, 20]));
    }

    #[test]
    fn div_assign_refmutvec_vec() {
        crate::gen_vector!(Vector<i32, 4>);
        let mut v = &mut Vector([4, 8, 16, 32]);
        let v2 = Vector([2, 3, 4, 5]);

        assert_eq!(*v, Vector([4, 8, 16, 32]));
        v /= v2;
        assert_eq!(*v, Vector([2, 2, 4, 6]));
    }

    #[test]
    fn norm_high_dimension() {
        gen_vector!(Vector<f32, 4> with linear_algebra_f32);
        let v = Vector([5.0, 6.0, 8.0, 3.0]);
        approx::assert_relative_eq!(134.0, v.norm2(), epsilon = 0.01);
        approx::assert_relative_eq!(11.57, v.norm(), epsilon = 0.01);
    }

    #[test]
    fn norm_eq_0() {
        {
            gen_vector!(Vector<f32, 45> with linear_algebra_f32);
            let v = Vector::default();
            approx::assert_relative_eq!(0.0, v.norm(), epsilon = 0.01);
            approx::assert_relative_eq!(0.0, v.norm2(), epsilon = 0.01);
        }

        {
            gen_vector!(Vector<f32, 4> with linear_algebra_f32);
            let v = Vector::default();
            approx::assert_relative_eq!(0.0, v.norm(), epsilon = 0.01);
            approx::assert_relative_eq!(0.0, v.norm2(), epsilon = 0.01);
        }
    }
}
