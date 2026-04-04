use super::*;

#[test]
fn testperiodinvariantold() {
    let data = fixtures::PATIENT_EXAMPLE_PERIOD.with(Value::clone);
    let expr = parse("Patient.identifier.period.all(start.hasValue().not() or end.hasValue().not() or (start <= end))").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[ignore] // lowBoundary/highBoundary for dates not fully implemented
#[test]
fn testperiodinvariantnew() {
    let data = fixtures::PATIENT_EXAMPLE_PERIOD.with(Value::clone);
    let expr = parse("Patient.identifier.period.all(start.empty() or end.empty() or (start.lowBoundary() < end.highBoundary()))").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
