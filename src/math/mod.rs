use crate::ExpFloat;
use std::ops::{Add, Div, Mul, Sub};

impl Add<ExpFloat> for ExpFloat {
    type Output = ExpFloat;

    fn add(self, rhs: ExpFloat) -> Self::Output {
        match (self, rhs) {
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => {
                ExpFloat::Integer(i1 + i2)
            },
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
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => ExpFloat::Integer(i1 * i2),
            (ExpFloat::Integer(i), ExpFloat::Decimal(b, e)) => unimplemented!(),
            (ExpFloat::Decimal(b, e), ExpFloat::Integer(i)) => unimplemented!(),
            (ExpFloat::Decimal(b1, e1), ExpFloat::Decimal(b2, e2)) => unimplemented!(),
        }
    }
}

impl Mul<ExpFloat> for ExpFloat {
    type Output = ExpFloat;

    fn mul(self, rhs: ExpFloat) -> Self::Output {
        match (self, rhs) {
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => ExpFloat::Integer(i1 * i2),
            (ExpFloat::Integer(i), ExpFloat::Decimal(b, e)) | (ExpFloat::Decimal(b, e), ExpFloat::Integer(i)) => {
                unimplemented!()
            }
            (ExpFloat::Decimal(b1, e1), ExpFloat::Decimal(b2, e2)) => unimplemented!(),
        }
    }
}

impl Div<ExpFloat> for ExpFloat {
    type Output = ExpFloat;

    fn div(self, rhs: ExpFloat) -> Self::Output {
        match (self, rhs) {
            (ExpFloat::Integer(i1), ExpFloat::Integer(i2)) => ExpFloat::Integer(i1 * i2),
            (ExpFloat::Integer(i), ExpFloat::Decimal(b, e)) => unimplemented!(),
            (ExpFloat::Decimal(b, e), ExpFloat::Integer(i)) => unimplemented!(),
            (ExpFloat::Decimal(b1, e1), ExpFloat::Decimal(b2, e2)) => unimplemented!(),
        }
    }
}