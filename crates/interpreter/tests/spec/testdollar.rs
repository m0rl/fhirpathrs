use super::*;

#[test]
fn testdollarthis1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.where(substring($this.length()-3) = 'out')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testdollarthis2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.where(substring($this.length()-3) = 'ter')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("Peter".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn testdollarorderallowed() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.skip(1).given").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Jim".to_string()),
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn testdollarorderalloweda() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.skip(3).given").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // mode=strict not supported
#[test]
fn testdollarordernotallowed() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.children().skip(1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
