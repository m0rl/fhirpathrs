use super::*;

#[test]
fn testplus1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 + 1 = 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testplus2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 + 0 = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testplus3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.2 + 1.8 = 3.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testplus4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a'+'b' = 'ab'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testplus5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a'+{}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testplusdate1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 7 days").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1974-01-01").expect("date")]
    );
}

#[test]
fn testplusdate2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 7.7 days").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1974-01-01").expect("date")]
    );
}

#[test]
fn testplusdate3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 7 days").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1974-01-01T00:00:00.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 7.7 days").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1974-01-01T00:00:00.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 1 second").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T00:00:01.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 10 millisecond").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T00:00:00.010+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 1 minute").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T00:01:00.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 1 hour").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T01:00:00.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 day").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1973-12-26").expect("date")]
    );
}

#[test]
fn testplusdate10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 month").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1974-01-25").expect("date")]
    );
}

#[test]
fn testplusdate11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 week").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1974-01-01").expect("date")]
    );
}

#[test]
fn testplusdate12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 year").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1974-12-25").expect("date")]
    );
}

#[test]
fn testplusdate13() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 'd'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1973-12-26").expect("date")]
    );
}

#[test]
fn testplusdate14() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 'mo'").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testplusdate15() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 'wk'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_date_str("1974-01-01").expect("date")]
    );
}

#[test]
fn testplusdate16() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25 + 1 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testplusdate17() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1975-12-25 + 1 'a'").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testplusdate18() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 1 's'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T00:00:01.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate19() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 0.1 's'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T00:00:00.100+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate20() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 10 'ms'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T00:00:00.010+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate21() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 1 'min'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T00:01:00.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplusdate22() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1973-12-25T00:00:00.000+10:00 + 1 'h'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_datetime_str("1973-12-25T01:00:00.000+10:00").expect("datetime")]
    );
}

#[test]
fn testplus6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@1974-12-25 + 7").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testplustime1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T01:00:00 + 2 hours").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_time_str("03:00:00").expect("time")]
    );
}

#[test]
fn testplustime2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T23:00:00 + 2 hours").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_time_str("01:00:00").expect("time")]
    );
}

#[test]
fn testplustime3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T23:00:00 + 50 hours").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(
        actual,
        vec![Value::from_time_str("01:00:00").expect("time")]
    );
}

#[test]
fn testplusempty1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 + {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testplusempty2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} + 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testplusempty3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{} + {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
