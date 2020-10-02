use crate::ExpFloat;
use std::ops::Div;

impl ExpFloat {
    pub fn decimal(b: f32, i: i32) -> Self {
        let exp = b.log10().floor();
        ExpFloat::Decimal(b.div(10f32.powf(exp)), i + exp as i32)
    }
}

macro_rules! from_int {
    ($($t:ty)*) => ($(
        impl From<$t> for ExpFloat {
            fn from(i: $t) -> Self {ExpFloat::Integer(i as i64)}
        }
    )*)
}

from_int!(i8 i16 i32 i64 u8 u16 u32);
