use super::*;

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimaldefault() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.highBoundary()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.5875_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal1() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.highBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.59_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal2() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.highBoundary(6)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.5875_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal3() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.highBoundary(-1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal4() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).highBoundary()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-1.5865_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal5() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).highBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-1.58_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal6() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).highBoundary(6)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-1.5865_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal7() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.highBoundary(39)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal8() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.highBoundary()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.5_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal9() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.highBoundary(0)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(2.0_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal10() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.highBoundary(5)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.5_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal11() {
    let data = Value::object(HashMap::new());
    let expr = parse("12.587.highBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(12.59_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal12() {
    let data = Value::object(HashMap::new());
    let expr = parse("12.500.highBoundary(4)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(12.5005_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal13() {
    let data = Value::object(HashMap::new());
    let expr = parse("120.highBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(120.5_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal14() {
    let data = Value::object(HashMap::new());
    let expr = parse("-120.highBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-120.5_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal15() {
    let data = Value::object(HashMap::new());
    let expr = parse("0.0034.highBoundary(1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(0.0_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal16() {
    let data = Value::object(HashMap::new());
    let expr = parse("-0.0034.highBoundary(1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(0.0_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydecimal() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.highBoundary(8)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.5875_f64)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundaryquantity() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587 'm'.highBoundary(8)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Quantity(1.5875_f64, "m".to_string(), None)]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydatemonth() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014.highBoundary(6)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-12").expect("datetime")]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydatetimemillisecond1() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014-01-01T08.highBoundary(17)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-01-01T08:00:59.999-12:00").expect("datetime")]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydatetimemillisecond2() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014-01-01T08:05-05:00.highBoundary(17)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-01-01T08:05:59.999-05:00").expect("datetime")]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarydatetimemillisecond3() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014-01-01T08.highBoundary(17)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-01-01T08:00:59.999-12:00").expect("datetime")]);
}

#[ignore] // not implemented: HighBoundary
#[test]
fn highboundarytimemillisecond() {
    let data = Value::object(HashMap::new());
    let expr = parse("@T10:30.highBoundary(9)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_time_str("10:30:59.999").expect("time")]);
}
