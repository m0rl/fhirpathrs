use super::*;

#[test]
fn testexclude1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1 | 2 | 3).exclude(2 | 4) = 1 | 3").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testexclude2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1 | 2).exclude(4) = 1 | 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testexclude3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1 | 2).exclude({}) = 1 | 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testexclude4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.combine(1).exclude(2).count() = 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
