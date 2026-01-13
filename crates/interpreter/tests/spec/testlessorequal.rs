use super::*;

#[test]
fn testlessorequal1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 <= 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0 <= 1.2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a' <= 'b'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'A' <= 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-12 <= @2014-12-13").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13T12:00:00 <= @2014-12-13T12:00:01").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T12:00:00 <= @T14:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 <= 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0 <= 1.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a' <= 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'A' <= 'A'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-12 <= @2014-12-12").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal13() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13T12:00:00 <= @2014-12-13T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal14() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T12:00:00 <= @T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal15() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2 <= 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testlessorequal16() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.1 <= 1.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testlessorequal17() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'b' <= 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testlessorequal18() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'B' <= 'A'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testlessorequal19() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13 <= @2014-12-12").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testlessorequal20() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13T12:00:01 <= @2014-12-13T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testlessorequal21() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T12:00:01 <= @T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testlessorequal22() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value <= 200 '[lb_av]'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal23() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2018-03 <= @2018-03-01").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testlessorequal24() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2018-03-01T10:30 <= @2018-03-01T10:30:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testlessorequal25() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T10:30 <= @T10:30:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testlessorequal26() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2018-03-01T10:30:00 <= @2018-03-01T10:30:00.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequal27() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T10:30:00 <= @T10:30:00.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testlessorequalempty1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 <= {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testlessorequalempty2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} <= 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testlessorequalempty3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} <= {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
