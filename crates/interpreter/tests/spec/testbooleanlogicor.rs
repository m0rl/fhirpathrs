use super::*;

#[test]
fn testbooleanlogicor1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true or true) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true or false) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true or {}) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false or true) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false or false) = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false or {}).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} or true) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} or false).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicor9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} or {}).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
