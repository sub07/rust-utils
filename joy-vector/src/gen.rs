#[macro_export]
macro_rules! gen_vector {
    ($name:ident<$t: ty, $len: literal>) => {
        $crate::gen_vector!($name < $t, $len > with);
    };
    ($name:ident<$t: tt, $len: tt> with $($feature:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name(pub [$t; $len]);

        impl $name {
            #[allow(clippy::cast_precision_loss)]
            pub const ZERO: Self = Self([0 as $t; $len]);

            #[must_use]
            pub const fn as_slice(&self) -> &[$t; $len] {
                &self.0
            }

            #[must_use]
            pub const fn as_slice_mut(&mut self) -> &mut [$t; $len] {
                &mut self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::ZERO
            }
        }

        impl From<[$t; $len]> for $name {
            fn from(values: [$t; $len]) -> Self {
                Self(values)
            }
        }

        impl From<&[$t; $len]> for $name {
            fn from(values: &[$t; $len]) -> Self {
                Self(*values)
            }
        }

        impl From<$t> for $name {
            fn from(values: $t) -> Self {
                Self([values; $len])
            }
        }

        impl From<$name> for [$t; $len] {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = [$t; $len];

            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_slice_mut()
            }
        }

        impl std::ops::Add<Self> for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                let mut out = self;
                for i in 0..$len {
                    out.0[i] += rhs.0[i];
                }
                out
            }
        }
        impl std::ops::Sub<Self> for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                let mut out = self;
                for i in 0..$len {
                    out.0[i] -= rhs.0[i];
                }
                out
            }
        }
        impl std::ops::Mul<Self> for $name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut out = self;
                for i in 0..$len {
                    out.0[i] *= rhs.0[i];
                }
                out
            }
        }
        impl std::ops::Div<Self> for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let mut out = self;
                for i in 0..$len {
                    out.0[i] /= rhs.0[i];
                }
                out
            }
        }

        impl std::ops::Add<&Self> for $name {
            type Output = Self;
            fn add(self, rhs: &Self) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] += rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Sub<&Self> for $name {
            type Output = Self;
            fn sub(self, rhs: &Self) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] -= rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Mul<&Self> for $name {
            type Output = Self;
            fn mul(self, rhs: &Self) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] *= rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Div<&Self> for $name {
            type Output = Self;
            fn div(self, rhs: &Self) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] /= rhs.0[i];
                }
                $name(data)
            }
        }

        impl std::ops::Add<$name> for &$name {
            type Output = $name;
            fn add(self, rhs: $name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] += rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Sub<$name> for &$name {
            type Output = $name;
            fn sub(self, rhs: $name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] -= rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Mul<$name> for &$name {
            type Output = $name;
            fn mul(self, rhs: $name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] *= rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Div<$name> for &$name {
            type Output = $name;
            fn div(self, rhs: $name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] /= rhs.0[i];
                }
                $name(data)
            }
        }

        impl std::ops::Add<&$name> for &$name {
            type Output = $name;
            fn add(self, rhs: &$name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] += rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Sub<&$name> for &$name {
            type Output = $name;
            fn sub(self, rhs: &$name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] -= rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Mul<&$name> for &$name {
            type Output = $name;
            fn mul(self, rhs: &$name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] *= rhs.0[i];
                }
                $name(data)
            }
        }
        impl std::ops::Div<&$name> for &$name {
            type Output = $name;
            fn div(self, rhs: &$name) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] /= rhs.0[i];
                }
                $name(data)
            }
        }

        impl std::ops::Add<$t> for $name {
            type Output = $name;
            fn add(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] += rhs;
                }
                $name(data)
            }
        }
        impl std::ops::Sub<$t> for $name {
            type Output = $name;
            fn sub(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] -= rhs;
                }
                $name(data)
            }
        }
        impl std::ops::Mul<$t> for $name {
            type Output = $name;
            fn mul(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] *= rhs;
                }
                $name(data)
            }
        }
        impl std::ops::Div<$t> for $name {
            type Output = $name;
            fn div(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] /= rhs;
                }
                $name(data)
            }
        }

        impl std::ops::Add<$t> for &$name {
            type Output = $name;
            fn add(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] += rhs;
                }
                $name(data)
            }
        }
        impl std::ops::Sub<$t> for &$name {
            type Output = $name;
            fn sub(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] -= rhs;
                }
                $name(data)
            }
        }
        impl std::ops::Mul<$t> for &$name {
            type Output = $name;
            fn mul(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] *= rhs;
                }
                $name(data)
            }
        }
        impl std::ops::Div<$t> for &$name {
            type Output = $name;
            fn div(self, rhs: $t) -> Self::Output {
                let mut data = self.0;
                for i in 0..$len {
                    data[i] /= rhs;
                }
                $name(data)
            }
        }

        impl std::ops::Add<$name> for $t {
            type Output = $name;
            fn add(self, rhs: $name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] += self;
                }
                $name(data)
            }
        }
        impl std::ops::Sub<$name> for $t {
            type Output = $name;
            fn sub(self, rhs: $name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] = self - data[i];
                }
                $name(data)
            }
        }
        impl std::ops::Mul<$name> for $t {
            type Output = $name;
            fn mul(self, rhs: $name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] *= self;
                }
                $name(data)
            }
        }
        impl std::ops::Div<$name> for $t {
            type Output = $name;
            fn div(self, rhs: $name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] = self / data[i];
                }
                $name(data)
            }
        }

        impl std::ops::Add<&$name> for $t {
            type Output = $name;
            fn add(self, rhs: &$name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] += self;
                }
                $name(data)
            }
        }
        impl std::ops::Sub<&$name> for $t {
            type Output = $name;
            fn sub(self, rhs: &$name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] = self - data[i];
                }
                $name(data)
            }
        }
        impl std::ops::Mul<&$name> for $t {
            type Output = $name;
            fn mul(self, rhs: &$name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] *= self;
                }
                $name(data)
            }
        }
        impl std::ops::Div<&$name> for $t {
            type Output = $name;
            fn div(self, rhs: &$name) -> Self::Output {
                let mut data = rhs.0;
                for i in 0..$len {
                    data[i] = self / data[i];
                }
                $name(data)
            }
        }

        impl std::ops::AddAssign<Self> for $name {
            fn add_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] += rhs.0[i];
                }
            }
        }
        impl std::ops::AddAssign<&$name> for $name {
            fn add_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] += rhs.0[i];
                }
            }
        }
        impl std::ops::SubAssign<Self> for $name {
            fn sub_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] -= rhs.0[i];
                }
            }
        }
        impl std::ops::SubAssign<&$name> for $name {
            fn sub_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] -= rhs.0[i];
                }
            }
        }
        impl std::ops::MulAssign<Self> for $name {
            fn mul_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] *= rhs.0[i];
                }
            }
        }
        impl std::ops::MulAssign<&$name> for $name {
            fn mul_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] *= rhs.0[i];
                }
            }
        }
        impl std::ops::DivAssign<Self> for $name {
            fn div_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] /= rhs.0[i];
                }
            }
        }
        impl std::ops::DivAssign<&$name> for $name {
            fn div_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] /= rhs.0[i];
                }
            }
        }

        impl std::ops::AddAssign<$name> for &mut $name {
            fn add_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] += rhs.0[i];
                }
            }
        }
        impl std::ops::AddAssign<&$name> for &mut $name {
            fn add_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] += rhs.0[i];
                }
            }
        }
        impl std::ops::SubAssign<$name> for &mut $name {
            fn sub_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] -= rhs.0[i];
                }
            }
        }
        impl std::ops::SubAssign<&$name> for &mut $name {
            fn sub_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] -= rhs.0[i];
                }
            }
        }
        impl std::ops::MulAssign<$name> for &mut $name {
            fn mul_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] *= rhs.0[i];
                }
            }
        }
        impl std::ops::MulAssign<&$name> for &mut $name {
            fn mul_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] *= rhs.0[i];
                }
            }
        }
        impl std::ops::DivAssign<$name> for &mut $name {
            fn div_assign(&mut self, rhs: $name) {
                for i in 0..$len {
                    self.0[i] /= rhs.0[i];
                }
            }
        }
        impl std::ops::DivAssign<&$name> for &mut $name {
            fn div_assign(&mut self, rhs: &$name) {
                for i in 0..$len {
                    self.0[i] /= rhs.0[i];
                }
            }
        }

        impl std::ops::AddAssign<$t> for $name {
            fn add_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v += rhs;
                }
            }
        }
        impl std::ops::SubAssign<$t> for $name {
            fn sub_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v -= rhs;
                }
            }
        }
        impl std::ops::MulAssign<$t> for $name {
            fn mul_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v *= rhs;
                }
            }
        }
        impl std::ops::DivAssign<$t> for $name {
            fn div_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v /= rhs;
                }
            }
        }

        impl std::ops::AddAssign<$t> for &mut $name {
            fn add_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v += rhs;
                }
            }
        }
        impl std::ops::SubAssign<$t> for &mut $name {
            fn sub_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v -= rhs;
                }
            }
        }
        impl std::ops::MulAssign<$t> for &mut $name {
            fn mul_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v *= rhs;
                }
            }
        }
        impl std::ops::DivAssign<$t> for &mut $name {
            fn div_assign(&mut self, rhs: $t) {
                for v in self.0.iter_mut() {
                    *v /= rhs;
                }
            }
        }

        $(
            $crate::gen_vector!(@feature $feature $name<$t, $len>);
        )*
        $crate::gen_vector!(@auto $name<$t, $len>);
    };
    (@feature linear_algebra_f32 $name:ident<$t: ty, $len: literal>) => {
        $crate::gen_vector!(@linear_algebra_impl f32 $name<$t, $len>);
    };
    (@feature linear_algebra_f64 $name:ident<$t: ty, $len: literal>) => {
        $crate::gen_vector!(@linear_algebra_impl f64 $name<$t, $len>);
    };
    (@linear_algebra_impl $math_ty:tt $name:ident<$t: ty, $len: literal>) => {
        impl $name {
            pub fn norm2(&self) -> $math_ty {
                self.0.iter().map(|x| (x * x) as $math_ty).sum()
            }

            pub fn norm(&self) -> $math_ty {
                self.norm2().sqrt()
            }
        }
    };
    (@feature two_dim $name:ident<$t: ty, 2>) => {
        impl $name {
            pub fn x(&self) -> $t {
                self.0[0]
            }

            pub fn y(&self) -> $t {
                self.0[1]
            }

            pub fn width(&self) -> $t {
                self.0[0]
            }

            pub fn height(&self) -> $t {
                self.0[1]
            }

            pub fn set_x(&mut self, value: $t) {
                self.0[0] = value;
            }

            pub fn set_y(&mut self, value: $t) {
                self.0[1] = value;
            }
        }
    };
    (@feature two_dim $name:ident<$t: ty, $len:literal>) => {
        compile_error!(concat!(
            "Feature 'two_dim' is only available for vectors of length 2, got length ",
            stringify!($len)
        ));
    };
    (@feature three_dim $name:ident<$t: ty, 3>) => {
        impl $name {
            pub fn x(&self) -> $t {
                self.0[0]
            }

            pub fn y(&self) -> $t {
                self.0[1]
            }

            pub fn z(&self) -> $t {
                self.0[2]
            }

            pub fn set_x(&mut self, value: $t) {
                self.0[0] = value;
            }

            pub fn set_y(&mut self, value: $t) {
                self.0[1] = value;
            }

            pub fn set_z(&mut self, value: $t) {
                self.0[2] = value;
            }
        }
    };
    (@feature three_dim $name:ident<$t: ty, $len:literal>) => {
        compile_error!(concat!(
            "Feature 'three_dim' is only available for vectors of length 3, got length ",
            stringify!($len)
        ));
    };
    (@auto $name:ident<f32, 0>) => {
        compile_error!("Vectors of length 0 are not supported");
    };
    (@auto $name:ident<f32, $len:literal>) => {
    };
    (@auto $name:ident<f64, $len:literal>) => {
    };
    (@auto $name:ident<$t:ty, $len:literal>) => {
        impl Eq for $name {}
    };
    (@feature $unknow:ident $($rem:tt)*) => {
        compile_error!(concat!("Unknown feature: ", stringify!($unknow)));
    };
}
