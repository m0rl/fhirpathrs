use super::*;

#[test]
fn testwhere1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.count() = 3").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testwhere2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.where(given = 'Jim').count() = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testwhere3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.where(given = 'X').count() = 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testwhere4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.where($this.given = 'Jim').count() = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
