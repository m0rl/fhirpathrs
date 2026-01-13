use super::*;

#[test]
fn testencodebase64a() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'test'.encode('base64')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("dGVzdA==".to_string())]);
}

#[test]
fn testencodehex() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'test'.encode('hex')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("74657374".to_string())]);
}

#[test]
fn testencodebase64b() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'subjects?_d'.encode('base64')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("c3ViamVjdHM/X2Q=".to_string())]);
}

#[test]
fn testencodeurlbase64() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'subjects?_d'.encode('urlbase64')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("c3ViamVjdHM_X2Q=".to_string())]);
}

#[test]
fn testdecodebase64a() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'dGVzdA=='.decode('base64')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("test".to_string())]);
}

#[test]
fn testdecodehex() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'74657374'.decode('hex')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("test".to_string())]);
}

#[test]
fn testdecodebase64b() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'c3ViamVjdHM/X2Q='.decode('base64')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("subjects?_d".to_string())]);
}

#[test]
fn testdecodeurlbase64() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'c3ViamVjdHM_X2Q='.decode('urlbase64')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("subjects?_d".to_string())]);
}
