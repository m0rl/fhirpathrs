use super::*;

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimaldefault() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.lowBoundary()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.5865_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal1() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.lowBoundary(6)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.5865_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal2() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.lowBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.58_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal3() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.lowBoundary(-1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal4() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.lowBoundary(0)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.0_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal5() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.lowBoundary(32)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarynegdecimaldefault() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).lowBoundary()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-1.5875_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarynegdecimal1() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).lowBoundary(6)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-1.5875_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarynegdecimal2() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).lowBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-1.59_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarynegdecimal3() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).lowBoundary(-1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarynegdecimal4() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).lowBoundary(0)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-2.0_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarynegdecimal5() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-1.587).lowBoundary(32)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal6() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587.lowBoundary(39)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal7() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.toDecimal().lowBoundary()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(0.5_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal8() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.lowBoundary(0)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(0.0_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal9() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.lowBoundary(5)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(0.5_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal10() {
    let data = Value::object(HashMap::new());
    let expr = parse("12.587.lowBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(12.58_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal11() {
    let data = Value::object(HashMap::new());
    let expr = parse("12.500.lowBoundary(4)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(12.4995_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal12() {
    let data = Value::object(HashMap::new());
    let expr = parse("120.lowBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(119.5_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal13() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-120).lowBoundary(2)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-120.5_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal14() {
    let data = Value::object(HashMap::new());
    let expr = parse("0.0034.lowBoundary(1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(0.0_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydecimal15() {
    let data = Value::object(HashMap::new());
    let expr = parse("(-0.0034).lowBoundary(1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(-0.0_f64)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundaryquantity() {
    let data = Value::object(HashMap::new());
    let expr = parse("1.587 'cm'.lowBoundary(8)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Quantity(1.5865_f64, "cm".to_string(), None)]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydatemonth() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014.lowBoundary(6)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-01").expect("datetime")]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydatetimemillisecond1() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014-01-01T08.lowBoundary(17)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-01-01T08:00:00.000+14:00").expect("datetime")]);
}

#[test]
fn lowboundarydatetimemillisecond2() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014-01-01T08:05+08:00.lowBoundary(17)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-01-01T08:05:00.000+08:00").expect("datetime")]);
}

#[ignore] // not implemented: LowBoundary
#[test]
fn lowboundarydatetimemillisecond3() {
    let data = Value::object(HashMap::new());
    let expr = parse("@2014-01-01T08.lowBoundary(8)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_datetime_str("2014-01-01").expect("datetime")]);
}

#[test]
fn lowboundarytimemillisecond() {
    let data = Value::object(HashMap::new());
    let expr = parse("@T10:30.lowBoundary(9)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::from_time_str("10:30:00.000").expect("time")]);
}
