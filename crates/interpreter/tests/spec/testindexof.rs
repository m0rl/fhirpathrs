use super::*;

#[test]
fn testindexof1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'LogicalModel-Person'.indexOf('-')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::number(12.0, 0)]);
}

#[test]
fn testindexof2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'LogicalModel-Person'.indexOf('z')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::number(-1.0, 0)]);
}

#[test]
fn testindexof3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'LogicalModel-Person'.indexOf('')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::number(0.0, 0)]);
}

#[test]
fn testindexof5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'LogicalModel-Person'.indexOf({}).empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testindexof4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.indexOf('-').empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testindexof6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.indexOf({}).empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
