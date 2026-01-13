use super::*;

#[test]
fn teststartswith1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.startsWith('2') = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.startsWith('1') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.startsWith('12') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.startsWith('13') = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.startsWith('12345') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.startsWith('123456') = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'12345'.startsWith('') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.startsWith('1').empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.startsWith('').empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("''.startsWith('') = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.startsWith('').exists() = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststartswith12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'987654321'.select(startsWith(length().toString()))").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // semantic validation not implemented
#[test]
fn teststartswith12a() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'987654321'.startsWith(length().toString())").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // semantic validation not implemented
#[test]
fn teststartswithnonstring1() {
    let data = fixtures::APPOINTMENT_EXAMPLEREQ.with(Value::clone);
    let expr = parse("Appointment.identifier.startsWith('rand')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
