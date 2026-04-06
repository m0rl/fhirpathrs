use super::*;

#[test]
fn testextractbirthdate() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("birthDate").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1974-12-25").expect("date")]
    );
}

#[ignore] // unqualified field access without type prefix
#[test]
fn testpatienthasbirthdate() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("birthDate").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testpatienttelecomtypes() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("telecom.use").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("home".to_string()),
        Value::String("work".to_string()),
        Value::String("mobile".to_string()),
        Value::String("old".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}
