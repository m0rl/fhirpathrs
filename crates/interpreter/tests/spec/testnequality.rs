use super::*;

#[test]
fn testnequality1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 != 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} != {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testnequality3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 != 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a' != 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a' != 'b'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.1 != 1.1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.1 != 1.2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.10 != 1.1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0 != 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.0 != 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15 != @2012-04-15").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15 != @2012-04-16").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality13() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15 != @2012-04-15T10:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testnequality14() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15T15:00:00 != @2012-04-15T10:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality15() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15T15:30:31 != @2012-04-15T15:30:31.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality16() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15T15:30:31 != @2012-04-15T15:30:31.1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality17() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15T15:00:00Z != @2012-04-15T10:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testnequality18() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2012-04-15T15:00:00+02:00 != @2012-04-15T16:00:00+03:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality19() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name != name").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality20() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.take(2) != name.take(2).first() | name.take(2).last()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality21() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("name.take(2) != name.take(2).last() | name.take(2).first()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality22() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1.2 / 1.8).round(2) != 0.6666667").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnequality23() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1.2 / 1.8).round(2) != 0.67").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnequality24() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value != 185 'kg'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
