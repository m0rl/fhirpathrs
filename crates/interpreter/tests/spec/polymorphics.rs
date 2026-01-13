use super::*;

#[test]
fn testpolymorphicsa() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value.exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // semantic validation not implemented
#[test]
fn testpolymorphicsb() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.valueQuantity.exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testpolymorphicsc() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.valueQuantity.exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testpolymorphicsd() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.valueString.exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}
