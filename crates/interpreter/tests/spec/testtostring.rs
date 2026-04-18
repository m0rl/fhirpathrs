use super::*;

#[test]
fn testtostring1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.toString() = '1'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtostring2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'-1'.toInteger() = -1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtostring3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.toString() = '0'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtostring4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.0.toString() = '0.0'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtostring5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-14.toString() = '2014-12-14'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
