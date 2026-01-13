use super::*;

#[test]
fn testnow1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate < now()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnow2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("now().toString().length() > 10").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
