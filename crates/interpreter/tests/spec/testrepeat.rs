use super::*;

#[test]
fn testrepeat1() {
    let data = fixtures::VALUESET_EXAMPLE_EXPANSION.with(Value::clone);
    let expr = parse("ValueSet.expansion.repeat(contains).count() = 10").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testrepeat2() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.repeat(item).code.count() = 11").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // descendants() with complex Questionnaire fixture
#[test]
fn testrepeat3() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.descendants().code.count() = 23").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // children() with complex Questionnaire fixture
#[test]
fn testrepeat4() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.children().code.count() = 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testrepeat5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.repeat('test')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("test".to_string())]);
}
