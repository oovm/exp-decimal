use crate::ExpFloat;
mod wrapper;
use std::ops::{Add, Div, Mul, Sub};

impl Add<ExpFloat> for ExpFloat {
    type Output = ExpFloat;

    fn add(self, rhs: ExpFloat) -> Self::Output {
        match (self, rhs) {
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => {
                if let Some(s) = i1.checked_add(i2) {
                    return ExpFloat::Integer(s);
                }
                unimplemented!()
            }
            (ExpFloat::Integer(i), ExpFloat::Decimal(b, e)) | (ExpFloat::Decimal(b, e), ExpFloat::Integer(i)) => {
                unimplemented!()
            }
            (ExpFloat::Decimal(b1, e1), ExpFloat::Decimal(b2, e2)) => unimplemented!(),
        }
    }
}

impl Sub<ExpFloat> for ExpFloat {
    type Output = ExpFloat;
    fn sub(self, rhs: ExpFloat) -> Self::Output {
        match (self, rhs) {
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => {
                if let Some(s) = i1.checked_sub(i2) {
                    return ExpFloat::Integer(s);
                }
                ExpFloat::decimal(i1 as f32, 0) + ExpFloat::decimal(-i1 as f32, 0)
            }
            (ExpFloat::Integer(i), ExpFloat::Decimal(b, e)) => ExpFloat::decimal(i as f32, 0) + ExpFloat::Decimal(-b, e),
            (ExpFloat::Decimal(b, e), ExpFloat::Integer(i)) => ExpFloat::Decimal(b, e) + ExpFloat::decimal(-i as f32, 0),
            (ExpFloat::Decimal(b1, e1), ExpFloat::Decimal(b2, e2)) => ExpFloat::Decimal(b1, e1) + ExpFloat::Decimal(-b2, e2),
        }
    }
}

impl Mul<ExpFloat> for ExpFloat {
    type Output = ExpFloat;
    fn mul(self, rhs: ExpFloat) -> Self::Output {
        match (self, rhs) {
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => {
                if let Some(s) = i1.checked_mul(i2) {
                    return ExpFloat::Integer(s);
                }
                ExpFloat::decimal(i1 as f32, 0) * ExpFloat::decimal(i2 as f32, 0)
            }
            (ExpFloat::Integer(i), ExpFloat::Decimal(b, e)) | (ExpFloat::Decimal(b, e), ExpFloat::Integer(i)) => {
                ExpFloat::decimal(i as f32, 0) * ExpFloat::Decimal(b, e)
            }
            (ExpFloat::Decimal(b1, e1), ExpFloat::Decimal(b2, e2)) => ExpFloat::decimal(b1 * b2, e1 + e2),
        }
    }
}

impl Div<ExpFloat> for ExpFloat {
    type Output = ExpFloat;
    fn div(self, rhs: ExpFloat) -> Self::Output {
        match (self, rhs) {
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => ExpFloat::decimal(i1 as f32, 0) / ExpFloat::decimal(i2 as f32, 0),
            (ExpFloat::Integer(i), ExpFloat::Decimal(b, e)) => ExpFloat::decimal(i as f32, 0) / ExpFloat::Decimal(b, e),
            (ExpFloat::Decimal(b, e), ExpFloat::Integer(i)) => ExpFloat::Decimal(b, e) / ExpFloat::decimal(i as f32, 0),
            (ExpFloat::Decimal(b1, e1), ExpFloat::Decimal(b2, e2)) => ExpFloat::decimal(b1 / b2, e1 - e2),
        }
    }
}
