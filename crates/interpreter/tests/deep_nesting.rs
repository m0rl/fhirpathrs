#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;
use std::collections::HashMap;

#[test]
fn test_deep_nested_where_no_stack_overflow() {
    let depth = 1000;
    let mut expr = "true".to_string();
    for _ in 0..depth {
        expr = format!("{expr}.where(true)");
    }

    let parsed = parse(&expr).expect("should parse deeply nested where");
    let data = Value::Boolean(true);
    let context = InterpreterContext::new(data);

    let (result, _) = interpret(&parsed, context).expect("should evaluate without stack overflow");
    assert_eq!(result, Value::collection(vec![Value::Boolean(true)]));
}

#[test]
fn test_deep_nested_select_no_stack_overflow() {
    let depth = 1000;
    let mut expr = "1".to_string();
    for _ in 0..depth {
        expr = format!("{expr}.select($this)");
    }

    let parsed = parse(&expr).expect("should parse deeply nested select");
    let data = Value::Number(1.0, 0);
    let context = InterpreterContext::new(data);

    let (result, _) = interpret(&parsed, context).expect("should evaluate without stack overflow");
    assert_eq!(result, Value::collection(vec![Value::Number(1.0, 0)]));
}

#[test]
fn test_deep_nested_member_access_no_stack_overflow() {
    let depth = 500;

    let mut inner_data = Value::Number(42.0, 0);
    for i in (0..depth).rev() {
        let key = format!("f{i}");
        let mut obj = HashMap::new();
        obj.insert(key, inner_data);
        inner_data = Value::object(obj);
    }

    let path: Vec<String> = (0..depth).map(|i| format!("f{i}")).collect();
    let expr_str = path.join(".");

    let parsed = parse(&expr_str).expect("should parse deep member access");
    let context = InterpreterContext::new(inner_data);

    let (result, _) = interpret(&parsed, context).expect("should evaluate without stack overflow");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_deep_nested_iif_no_stack_overflow() {
    let depth = 500;
    let mut expr = "42".to_string();
    for _ in 0..depth {
        expr = format!("iif(true, {expr}, 0)");
    }

    let parsed = parse(&expr).expect("should parse deeply nested iif");
    let data = Value::Null;
    let context = InterpreterContext::new(data);

    let (result, _) = interpret(&parsed, context).expect("should evaluate without stack overflow");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_deep_nested_all_no_stack_overflow() {
    let depth = 500;
    let mut expr = "true".to_string();
    for _ in 0..depth {
        expr = format!("{expr}.all(true)");
    }

    let parsed = parse(&expr).expect("should parse deeply nested all");
    let data = Value::Boolean(true);
    let context = InterpreterContext::new(data);

    let (result, _) = interpret(&parsed, context).expect("should evaluate without stack overflow");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_deep_nested_mixed_functions_no_stack_overflow() {
    let depth = 300;
    let mut expr = "1".to_string();
    for i in 0..depth {
        expr = match i % 3 {
            0 => format!("{expr}.where(true)"),
            1 => format!("{expr}.select($this)"),
            _ => format!("{expr}.all(true)"),
        };
    }

    let parsed = parse(&expr).expect("should parse deeply nested mixed functions");
    let data = Value::Number(1.0, 0);
    let context = InterpreterContext::new(data);

    let _result = interpret(&parsed, context).expect("should evaluate without stack overflow");
}
