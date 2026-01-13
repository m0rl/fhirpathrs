use super::*;

#[test]
fn testbooleanimplies1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true implies true) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true implies false) = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true implies {}).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false implies true) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false implies false) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false implies {}) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} implies true) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} implies false).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanimplies9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} implies {}).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
