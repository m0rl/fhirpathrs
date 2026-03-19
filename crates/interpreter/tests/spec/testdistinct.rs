use super::*;

#[test]
fn testdistinct1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1 | 2 | 3).isDistinct()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdistinct2() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.descendants().linkId.isDistinct()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdistinct3() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.descendants().linkId.select(substring(0,1)).isDistinct().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdistinct4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1 | 2 | 3).distinct()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn testdistinct5() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.descendants().linkId.distinct().count()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(10.0)]);
}

#[test]
fn testdistinct6() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.descendants().linkId.select(substring(0,1)).distinct().count()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(2.0)]);
}
