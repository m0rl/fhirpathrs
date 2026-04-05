use super::*;

#[test]
fn testendswith1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.endsWith('2') = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.endsWith('5') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.endsWith('45') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.endsWith('35') = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.endsWith('12345') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.endsWith('012345') = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.endsWith('') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.endsWith('1').empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.endsWith('').empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'123456789'.select(endsWith(length().toString()))").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testendswith10a() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'123456789'.endsWith(length().toString())").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testendswithnonstring1() {
    let data = fixtures::APPOINTMENT_EXAMPLEREQ.with(Value::clone);
    let expr = parse("Appointment.identifier.endsWith('rand')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
