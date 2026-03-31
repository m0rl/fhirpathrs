use super::*;

#[ignore] // decimal precision tracking not implemented
#[test]
fn precisiondecimal() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.58700.precision()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(5.0, 0)]);
}

#[test]
fn precisionyear() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014.precision()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(4.0, 0)]);
}

#[test]
fn precisiondatetimemilliseconds() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014-01-05T10:30:00.000.precision()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(17.0, 0)]);
}

#[test]
fn precisiontimeminutes() {
    let data = Value::object(HashMap::new());
    let expr = parse("@T10:30.precision()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(4.0, 0)]);
}

#[test]
fn precisiontimemilliseconds() {
    let data = Value::object(HashMap::new());
    let expr = parse("@T10:30:00.000.precision()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(9.0, 0)]);
}

#[test]
fn precisionempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.precision().empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
