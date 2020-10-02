use crate::ExpFloat;

impl ExpFloat {
    pub fn decimal(b: f32, e: f32) -> Self {
        ExpFloat::Decimal(b, e)
    }
}

impl From<i64> for ExpFloat {
    fn from(i: i64) -> Self {
        ExpFloat::Integer(i)
    }
}

impl From<i32> for ExpFloat {
    fn from(i: i32) -> Self {
        ExpFloat::Integer(i as i64)
    }
}
