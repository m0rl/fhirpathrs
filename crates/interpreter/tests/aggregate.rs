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
        Value::number(1.0, 0),
        Value::number(2.0, 0),
        Value::number(3.0, 0),
        Value::number(4.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(10.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(0.0, 0));

    let data = Value::collection(vec![
        Value::number(1.0, 0),
        Value::Null,
        Value::number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(3.0, 0));
}

#[test]
fn test_math_avg() {
    let data = Value::collection(vec![
        Value::number(2.0, 0),
        Value::number(4.0, 0),
        Value::number(6.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(4.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let context = InterpreterContext::new(Value::collection(vec![Value::number(5.0, 0)]));
    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(5.0, 0));
}

#[test]
fn test_math_min() {
    let data = Value::collection(vec![
        Value::number(5.0, 0),
        Value::number(2.0, 0),
        Value::number(8.0, 0),
        Value::number(1.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(1.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let data = Value::collection(vec![
        Value::number(-5.0, 0),
        Value::number(2.0, 0),
        Value::number(-8.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(-8.0, 0));
}

#[test]
fn test_math_max() {
    let data = Value::collection(vec![
        Value::number(5.0, 0),
        Value::number(2.0, 0),
        Value::number(8.0, 0),
        Value::number(1.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(8.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let data = Value::collection(vec![
        Value::number(-5.0, 0),
        Value::number(-2.0, 0),
        Value::number(-8.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(-2.0, 0));
}

#[test]
fn test_math_combined_operations() {
    let data = Value::collection(vec![
        Value::number(1.0, 0),
        Value::number(2.0, 0),
        Value::number(3.0, 0),
        Value::number(4.0, 0),
        Value::number(5.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("sum().sqrt()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(3.87298335, 8));

    let expr = parse("avg().round()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(3.0, 0));

    let expr = parse("max().power(2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::number(25.0, 0));
}
