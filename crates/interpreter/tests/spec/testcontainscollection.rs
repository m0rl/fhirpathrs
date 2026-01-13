use super::*;

#[test]
fn testcontainscollection1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1 | 2 | 3) contains 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testcontainscollection2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(2 | 3) contains 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testcontainscollection3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("('a' | 'c' | 'd') contains 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testcontainscollection4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("('a' | 'c' | 'd') contains 'b'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testcontainscollectionempty1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} contains 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testcontainscollectionempty2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} contains 'value'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testcontainscollectionempty3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} contains true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testcontainscollectionempty4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} contains {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testcontainscollectionemptydatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} contains @2023-01-01").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}
