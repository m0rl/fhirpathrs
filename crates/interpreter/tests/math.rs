#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;

#[test]
fn test_math_abs() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(-5).abs()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0, 0));

    let expr = parse("5.abs()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0, 0));

    let expr = parse("0.abs()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));

    let expr = parse("(-3.14).abs()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.14, 2));
}

#[test]
fn test_math_ceiling() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1.1.ceiling()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("1.9.ceiling()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("(-1.1).ceiling()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-1.0, 0));

    let expr = parse("2.ceiling()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_math_floor() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1.1.floor()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("1.9.floor()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("(-1.1).floor()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-2.0, 0));

    let expr = parse("2.floor()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_math_round() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1.4.round()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("1.5.round()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("1.6.round()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("3.14159.round(2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.14, 2));

    let expr = parse("3.14159.round(3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.142, 3));

    let expr = parse("1234.round(-2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1200.0, 0));
}

#[test]
fn test_math_truncate() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1.9.truncate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("(-1.9).truncate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-1.0, 0));

    let expr = parse("5.truncate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0, 0));
}

#[test]
fn test_math_sqrt() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("4.sqrt()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("9.sqrt()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("2.sqrt()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Number(n, _) = result {
        assert!((n - std::f64::consts::SQRT_2).abs() < 1e-10);
    } else {
        panic!("Expected number");
    }

    let expr = parse("(-4).sqrt()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let expr = parse("0.sqrt()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_math_exp() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("0.exp()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("1.exp()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Number(n, _) = result {
        assert!((n - std::f64::consts::E).abs() < 1e-10);
    } else {
        panic!("Expected number");
    }

    let expr = parse("2.exp()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Number(n, _) = result {
        assert!((n - std::f64::consts::E.powi(2)).abs() < 1e-10);
    } else {
        panic!("Expected number");
    }
}

#[test]
fn test_math_ln() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1.ln()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));

    let data = Value::Number(std::f64::consts::E, 0);
    let context = InterpreterContext::new(data);
    let expr = parse("ln()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Number(n, _) = result {
        assert!((n - 1.0).abs() < 1e-10);
    } else {
        panic!("Expected number");
    }

    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(-1).ln()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let expr = parse("0.ln()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);
}

#[test]
fn test_math_log() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("100.log()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("10.log()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("8.log(2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("27.log(3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("(-10).log()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let expr = parse("10.log(1)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let expr = parse("10.log(-2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);
}

#[test]
fn test_math_power() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("2.power(3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(8.0, 0));

    let expr = parse("3.power(2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(9.0, 0));

    let expr = parse("4.power(0.5)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("2.power(0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("2.power(-1)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.5, 1));

    let expr = parse("(-2).power(0.5)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);
}

#[test]
fn test_quantity_abs() {
    let context = InterpreterContext::new(Value::Quantity(-3.5, 1, "kg".to_string(), None));

    let expr = parse("abs()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.5, 1, "kg".to_string(), None));
}

#[test]
fn test_quantity_ceiling() {
    let context = InterpreterContext::new(Value::Quantity(2.3, 1, "m".to_string(), None));

    let expr = parse("ceiling()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.0, 0, "m".to_string(), None));
}

#[test]
fn test_quantity_floor() {
    let context = InterpreterContext::new(Value::Quantity(2.7, 1, "m".to_string(), None));

    let expr = parse("floor()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(2.0, 0, "m".to_string(), None));
}

#[test]
fn test_quantity_round() {
    let context = InterpreterContext::new(Value::Quantity(3.456, 3, "mg".to_string(), None));

    let expr = parse("round(2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.46, 2, "mg".to_string(), None));

    let expr = parse("round()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.0, 0, "mg".to_string(), None));
}

#[test]
fn test_quantity_truncate() {
    let context = InterpreterContext::new(Value::Quantity(3.9, 1, "L".to_string(), None));

    let expr = parse("truncate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.0, 0, "L".to_string(), None));
}
