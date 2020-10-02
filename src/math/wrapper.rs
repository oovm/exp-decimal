use crate::ExpFloat;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! wrapper_math {
    ($($t:ty)*) => ($(
        impl Add<$t> for ExpFloat {
            type Output = ExpFloat;
            fn add(self, rhs: $t) -> Self::Output {
                self + ExpFloat::from(rhs)
            }
        }

        impl Sub<$t> for ExpFloat {
            type Output = ExpFloat;
            fn sub(self, rhs: $t) -> Self::Output {
                self - ExpFloat::from(rhs)
            }
        }

        impl Mul<$t> for ExpFloat {
            type Output = ExpFloat;
            fn mul(self, rhs: $t) -> Self::Output {
                self * ExpFloat::from(rhs)
            }
        }

        impl Div<$t> for ExpFloat {
            type Output = ExpFloat;
            fn div(self, rhs: $t) -> Self::Output {
                self / ExpFloat::from(rhs)
            }
        }
    )*)
}

wrapper_math!(i8 i16 i32 i64 u8 u16 u32);
