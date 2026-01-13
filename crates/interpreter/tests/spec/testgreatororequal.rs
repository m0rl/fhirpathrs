use super::*;

#[test]
fn testgreatororequal1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 >= 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testgreatororequal2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0 >= 1.2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testgreatororequal3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a' >= 'b'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testgreatororequal4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'A' >= 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testgreatororequal5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-12 >= @2014-12-13").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testgreatororequal6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13T12:00:00 >= @2014-12-13T12:00:01").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testgreatororequal7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T12:00:00 >= @T14:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testgreatororequal8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 >= 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0 >= 1.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a' >= 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'A' >= 'A'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-12 >= @2014-12-12").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal13() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13T12:00:00 >= @2014-12-13T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal14() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T12:00:00 >= @T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal15() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2 >= 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal16() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.1 >= 1.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal17() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'b' >= 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal18() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'B' >= 'A'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal19() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13 >= @2014-12-12").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal20() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2014-12-13T12:00:01 >= @2014-12-13T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal21() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T12:00:01 >= @T12:00:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal22() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value >= 100 '[lb_av]'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal23() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2018-03 >= @2018-03-01").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testgreatororequal24() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2018-03-01T10:30 >= @2018-03-01T10:30:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testgreatororequal25() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T10:30 >= @T10:30:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testgreatororequal26() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2018-03-01T10:30:00 >= @2018-03-01T10:30:00.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequal27() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T10:30:00 >= @T10:30:00.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testgreatororequalempty1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 >= {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testgreatororequalempty2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} >= 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testgreatororequalempty3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} >= {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
