use super::*;

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("4.0000 'g' = 4000.0 'mg'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("4 'g' ~ 4000 'mg'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testquantity3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("4 'g' != 4040 'mg'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("4 'g' ~ 4040 'mg'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("7 days = 1 week").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("7 days = 1 'wk'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("6 days < 1 week").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("8 days > 1 week").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2.0 'cm' * 2.0 'm' = 0.040 'm2'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("4.0 'g' / 2.0 'm' = 2 'g/m'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testQuantity
#[test]
fn testquantity11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0 'm' / 1.0 'm' = 1 '1'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
