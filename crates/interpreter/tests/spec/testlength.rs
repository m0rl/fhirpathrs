use super::*;

#[test]
fn testlength1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'123456'.length() = 6").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlength2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.length() = 5").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlength3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'123'.length() = 3").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlength4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.length() = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlength5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("''.length() = 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlength6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.length().empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
