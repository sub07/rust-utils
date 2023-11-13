pub trait EpsilonEq {
    fn epsilon_eq(&self, rhs: &Self, epsilon: Self) -> bool;
}

macro_rules! impl_epsilon_eq_float {
    ($($ty:ty),+) => {
        $(
            impl EpsilonEq for $ty {
                fn epsilon_eq(&self, rhs: &Self, epsilon: Self) -> bool {
                    (*self - *rhs).abs() < epsilon
                }
            }
        )+
    };
}

impl_epsilon_eq_float!(f32, f64);