use crate::ExpFloat;
use std::fmt::{self, Display, Formatter};

pub trait Scientific {
    fn to_scientific(&self) -> String;
    fn to_scientific_with(&self, e: char) -> String {
        self.to_scientific().replace('e', &e.to_string())
    }
}

pub trait Engineering {
    fn to_engineering(&self) -> String;
    fn to_engineering_with(&self) -> String {
        self.to_engineering()
    }
}

impl Display for ExpFloat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExpFloat::Integer(i) => write!(f, "{}", i),
            ExpFloat::Decimal(b, e) => {
                let mut exp = String::new();
                for c in e.to_string().chars() {
                    let r = match c {
                        '0' => '⁰',
                        '1' => '¹',
                        '2' => '²',
                        '3' => '³',
                        '4' => '⁴',
                        '5' => '⁵',
                        '6' => '⁶',
                        '7' => '⁷',
                        '8' => '⁸',
                        '9' => '⁹',
                        '+' => '⁺',
                        '-' => '⁻',
                        _ => c,
                    };
                    exp.push(r)
                }
                write!(f, "{}×10{}", b, exp)
            }
        }
    }
}
