use super::*;

#[test]
fn testexists1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testexists2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.exists(use = 'nickname')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testexists3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.exists(use = 'official')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testexists4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.maritalStatus.coding.exists(code = 'P' and system = 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus')\nor Patient.maritalStatus.coding.exists(code = 'A' and system = 'http://terminology.hl7.org/CodeSystem/v3-MaritalStatus')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testexists5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1 | 2).exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
