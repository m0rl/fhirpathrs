#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]

use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;


#[test]
fn test_math_sum() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
        Value::Number(4.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));

    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Null,
        Value::Number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));
}

#[test]
fn test_math_avg() {
    let data = Value::collection(vec![
        Value::Number(2.0, 0),
        Value::Number(4.0, 0),
        Value::Number(6.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(4.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let context = InterpreterContext::new(Value::collection(vec![Value::Number(5.0, 0)]));
    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0, 0));
}

#[test]
fn test_math_min() {
    let data = Value::collection(vec![
        Value::Number(5.0, 0),
        Value::Number(2.0, 0),
        Value::Number(8.0, 0),
        Value::Number(1.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let data = Value::collection(vec![
        Value::Number(-5.0, 0),
        Value::Number(2.0, 0),
        Value::Number(-8.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-8.0, 0));
}

#[test]
fn test_math_max() {
    let data = Value::collection(vec![
        Value::Number(5.0, 0),
        Value::Number(2.0, 0),
        Value::Number(8.0, 0),
        Value::Number(1.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(8.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let data = Value::collection(vec![
        Value::Number(-5.0, 0),
        Value::Number(-2.0, 0),
        Value::Number(-8.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-2.0, 0));
}

#[test]
fn test_math_combined_operations() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
        Value::Number(4.0, 0),
        Value::Number(5.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("sum().sqrt()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Number(n, _) = result {
        assert!((n - 15_f64.sqrt()).abs() < 1e-10);
    } else {
        panic!("Expected number");
    }

    let expr = parse("avg().round()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("max().power(2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(25.0, 0));
}
