use super::*;

#[test]
fn testmultiply1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 * 1 = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmultiply2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 * 0 = 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmultiply3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.2 * 1.8 = 2.16").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmultiplyempty1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 * {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testmultiplyempty2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} * 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testmultiplyempty3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} * {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
