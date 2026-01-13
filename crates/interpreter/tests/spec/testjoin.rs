use super::*;

#[test]
fn testjoin() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.given.join(',')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("Peter,James,Jim,Peter,James".to_string())]);
}
