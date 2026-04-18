use super::*;

#[test]
fn testcollectionboolean1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(1 | 2 | 3, true, false)").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testcollectionboolean2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif({}, true, false)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testcollectionboolean3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(true, true, false)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testcollectionboolean4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif({} | true, true, false)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testcollectionboolean5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(true, true, 1/0)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testcollectionboolean6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(false, 1/0, true)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
