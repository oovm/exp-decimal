mod display;
mod math;
mod convert;

#[derive(Debug, Copy, Clone)]
pub enum ExpFloat {
    Integer(i64),
    Decimal(f32, f32),
}
