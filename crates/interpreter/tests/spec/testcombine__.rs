use super::*;

#[test]
fn testcombine1() {
    let data = fixtures::CODESYSTEM_EXAMPLE.with(Value::clone);
    let expr = parse("concept.code.combine($this.descendants().concept.code).isDistinct()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testcombine2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.given.combine(name.family).exclude('Jim')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Chalmers".to_string()),
        Value::String("Windsor".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn testcombine3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.given.combine($this.name.family).exclude('Jim')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Chalmers".to_string()),
        Value::String("Windsor".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}
