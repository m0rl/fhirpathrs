use super::*;

#[test]
fn teststringyearconvertstodate() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015'.convertsToDate()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringmonthconvertstodate() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02'.convertsToDate()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringdayconvertstodate() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04'.convertsToDate()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringyearconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringmonthconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringdayconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringhourconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04T14'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringminuteconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04T14:34'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringsecondconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04T14:34:28'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringmillisecondconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04T14:34:28.123'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringutcconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04T14:34:28Z'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringtzconvertstodatetime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'2015-02-04T14:34:28+10:00'.convertsToDateTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringhourconvertstotime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'14'.convertsToTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringminuteconvertstotime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'14:34'.convertsToTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringsecondconvertstotime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'14:34:28'.convertsToTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringmillisecondconvertstotime() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'14:34:28.123'.convertsToTime()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralconvertstointeger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralisinteger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.is(Integer)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralissysteminteger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.is(System.Integer)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringliteralconvertstointeger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringliteralconvertstointegerfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'a'.convertsToInteger().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringdecimalconvertstointegerfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.0'.convertsToInteger().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringliteralisnotinteger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.is(Integer).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteralconvertstointeger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteralisnotinteger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.is(Integer).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdateisnotinteger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2013-04-05.is(Integer).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteraltointeger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.toInteger() = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringintegerliteraltointeger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.toInteger() = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteraltointeger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.1'.toInteger() = {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testdecimalliteraltointegerisempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.1'.toInteger().empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteraltointeger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.toInteger() = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralconvertstodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // Integer/Decimal type distinction not implemented
#[test]
fn testintegerliteralisnotdecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.is(Decimal).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteralconvertstodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteralisdecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.is(Decimal)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringintegerliteralconvertstodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringintegerliteralisnotdecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.is(Decimal).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringliteralconvertstodecimalfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.a'.convertsToDecimal().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringdecimalliteralconvertstodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.0'.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringdecimalliteralisnotdecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.0'.is(Decimal).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteralconvertstodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteralisnotdecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.is(Decimal).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteraltodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.toDecimal() = 1.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteraltodeciamlequivalent() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.toDecimal() ~ 1.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteraltodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.toDecimal() = 1.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteraltodecimalequal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.1'.toDecimal() = 1.1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteraltodecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.toDecimal() = 1").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralconvertstoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralisnotquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.is(Quantity).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteralconvertstoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteralisnotquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.is(System.Quantity).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringintegerliteralconvertstoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringintegerliteralisnotquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.is(System.Quantity).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringquantityliteralconvertstoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 day'.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringquantityweekconvertstoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 \\'wk\\''.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // UCUM quantity validation not implemented
#[test]
fn teststringquantityweekconvertstoquantityfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 wk'.convertsToQuantity().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // quantity string validation not implemented
#[test]
fn teststringdecimalliteralconvertstoquantityfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.a'.convertsToQuantity().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringdecimalliteralconvertstoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.0'.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringdecimalliteralisnotsystemquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.0'.is(System.Quantity).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // Boolean.convertsToQuantity not implemented
#[test]
fn testbooleanliteralconvertstoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteralisnotsystemquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.is(System.Quantity).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.toQuantity() = 1 '1'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.toQuantity() = 1.0 '1'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringintegerliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.toQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Quantity(1.0_f64, "1".to_string(), None)]);
}

#[test]
fn teststringquantityliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 day'.toQuantity() = 1 day").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // UCUM quantity alias not implemented
#[test]
fn teststringquantitydayliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 day'.toQuantity() = 1 'd'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // UCUM quantity alias not implemented
#[test]
fn teststringquantityweekliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 \\'wk\\''.toQuantity() = 1 week").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // UCUM quantity alias not implemented
#[test]
fn teststringquantitymonthliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 \\'mo\\''.toQuantity() = 1 month").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // UCUM quantity alias not implemented
#[test]
fn teststringquantityyearliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1 \\'a\\''.toQuantity() = 1 year").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn teststringdecimalliteraltoquantity() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1.0'.toQuantity() ~ 1 '1'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralconvertstobooleanfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testnegativeintegerliteralconvertstobooleanfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-1).convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testintegerliteralfalseconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteralconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringtrueliteralconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'true'.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringfalseliteralconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'false'.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringfalseliteralalsoconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'False'.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtrueliteralconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testfalseliteralconvertstoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("false.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteraltoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.toBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteraltobooleanempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2.toBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testintegerliteraltobooleanfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.toBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn teststringtruetoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'true'.toBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringfalsetoboolean() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'false'.toBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testintegerliteralconvertstostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteralisnotstring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.is(String).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnegativeintegerliteralconvertstostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-1).convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdecimalliteralconvertstostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn teststringliteralconvertstostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'true'.convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testbooleanliteralconvertstostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testquantityliteralconvertstostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 'wk'.convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerliteraltostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.toString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("1".to_string())]);
}

#[test]
fn testnegativeintegerliteraltostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-1).toString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("-1".to_string())]);
}

#[ignore] // Integer/Decimal type distinction not implemented
#[test]
fn testdecimalliteraltostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.toString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("1.0".to_string())]);
}

#[test]
fn teststringliteraltostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'true'.toString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("true".to_string())]);
}

#[test]
fn testbooleanliteraltostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.toString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("true".to_string())]);
}

#[test]
fn testquantityliteralwktostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 'wk'.toString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("1 'wk'".to_string())]);
}

#[ignore] // Quantity toString not implemented
#[test]
fn testquantityliteralweektostring() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 week.toString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("1 week".to_string())]);
}
