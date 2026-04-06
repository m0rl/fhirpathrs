use super::*;

#[test]
fn testescapehtml() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'\"1<2\"'.escape('html')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::String("&quot;1&lt;2&quot;".to_string())]
    );
}

#[test]
fn testescapejson() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'\"1<2\"'.escape('json')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("\\\"1<2\\\"".to_string())]);
}

#[test]
fn testunescapehtml() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'&quot;1&lt;2&quot;'.unescape('html')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("\"1<2\"".to_string())]);
}

#[test]
fn testunescapejson() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'\\\"1<2\\\"'.unescape('json')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("\"1<2\"".to_string())]);
}
