use super::*;

#[test]
fn testminus1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 - 1 = 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testminus2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 - 0 = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testminus3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.8 - 1.2 = 0.6").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testminus4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a'-'b' = 'ab'").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testminus5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1974-12-25 - 1 'month'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_date_str("1974-11-25").expect("date")]);
}

#[test]
fn testminus6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1974-12-25 - 1 'cm'").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testminus7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T00:30:00 - 1 hour").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_time_str("23:30:00").expect("time")]);
}

#[test]
fn testminus8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T01:00:00 - 2 hours").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_time_str("23:00:00").expect("time")]);
}

#[test]
fn testminusempty1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 - {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testminusempty2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} - 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testminusempty3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} - {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
