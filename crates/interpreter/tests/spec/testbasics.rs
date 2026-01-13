use super::*;

#[test]
fn testsimple() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.given").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Jim".to_string()),
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn testsimplenone() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.suffix").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testescapedidentifier() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.`given`").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Jim".to_string()),
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn testsimplebacktick1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("`Patient`.name.`given`").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Jim".to_string()),
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[ignore] // mode=strict not supported
#[test]
fn testsimplefail() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.given1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testsimplewithcontext() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Jim".to_string()),
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[ignore] // mode=strict not supported
#[test]
fn testsimplewithwrongcontext() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Encounter.name.given").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
