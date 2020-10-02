use exp_decimal::ExpFloat;

#[test]
fn test() {
    println!("{}", ExpFloat::decimal(1.2, 3));
    println!("{}", ExpFloat::decimal(12.0, 2));
}

#[test]
fn test_big() {
    println!("{}", ExpFloat::Integer(i64::max_value()));
    println!("{}", ExpFloat::Integer(i64::max_value()) * ExpFloat::Integer(i64::max_value()));
}
