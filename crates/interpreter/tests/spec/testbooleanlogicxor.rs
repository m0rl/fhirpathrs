use super::*;

#[test]
fn testbooleanlogicxor1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true xor true) = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true xor false) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(true xor {}).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false xor true) = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false xor false) = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(false xor {}).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} xor true).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} xor false).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanlogicxor9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("({} xor {}).empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
