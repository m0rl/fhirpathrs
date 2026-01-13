use super::*;

#[test]
fn testabs1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-5).abs() = 5").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testabs2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-5.5).abs() = 5.5").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testabs3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-5.5 'mg').abs() = 5.5 'mg'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testabsempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.abs().empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
