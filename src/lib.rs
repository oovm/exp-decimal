mod display;
mod math;

#[derive(Debug, Copy, Clone)]
pub enum ExpFloat {
    Integer(i64),
    Decimal(f32, f32),
}
