use super::*;

#[test]
fn comparable1() {
    let data = Value::object(HashMap::new());
    let expr = parse("1 'cm'.comparable(1 '[in_i]')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn comparable2() {
    let data = Value::object(HashMap::new());
    let expr = parse("1 'cm'.comparable(1 '[s]')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn comparable3() {
    let data = Value::object(HashMap::new());
    let expr = parse("1 'cm'.comparable(1 's')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}
