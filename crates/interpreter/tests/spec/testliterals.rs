use super::*;

#[test]
fn testliteraltrue() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.exists() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.empty() = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralstring1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.first() = 'Peter'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralinteger1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralinteger0() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegernegative1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-1).convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegernegative1invalid() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("-1.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testliteralintegermax() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2147483647.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralstring2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'test'.convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralstringescapes() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'\\\\\\/\\f\\r\\n\\t\\\"\\`\\'\\u002a'.convertsToString()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralbooleantrue() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralbooleanfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("false.convertsToBoolean()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimal10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.0.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimal01() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.1.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimal00() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.0.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimalnegative01() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(-0.1).convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimalnegative01invalid() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("-0.1.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testliteraldecimalmax() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1234567890987654321.0.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimalstep() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("0.00000001.convertsToDecimal()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldateyear() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015.is(Date)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatemonth() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02.is(Date)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldateday() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04.is(Date)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimeyear() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015T.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimemonth() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02T.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimeday() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04T.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimehour() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04T14.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimeminute() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04T14:34.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimesecond() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04T14:34:28.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimemillisecond() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04T14:34:28.123.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimeutc() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04T14:34:28Z.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimetimezoneoffset() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2015-02-04T14:34:28+10:00.is(DateTime)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraltimehour() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T14.is(Time)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraltimeminute() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T14:34.is(Time)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraltimesecond() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T14:34:28.is(Time)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraltimemillisecond() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T14:34:28.123.is(Time)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraltimeutc() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T14:34:28Z.is(Time)").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testliteraltimetimezoneoffset() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@T14:34:28+10:00.is(Time)").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testliteralquantitydecimal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("10.1 'mg'.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralquantityinteger() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("10 'mg'.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralquantityday() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("4 days.convertsToQuantity()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegernotequal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("-3 != 3").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegerequal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.count() = 5").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testpolarityprecedence() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("-Patient.name.given.count() = -5").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegergreaterthan() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.count() > -3").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegercountnotequal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.count() != 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegerlessthantrue() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 < 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegerlessthanfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1 < -2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testliteralintegerlessthanpolaritytrue() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("+1 < +2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralintegerlessthanpolarityfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("-1 < 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimalgreaterthannonzerotrue() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value.value > 180.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimalgreaterthanzerotrue() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value.value > 0.0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimalgreaterthanintegertrue() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value.value > 0").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimallessthaninteger() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value.value < 190").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldecimallessthaninvalid() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.value.value < 'test'").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testdateequal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate = @1974-12-25").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdatenotequal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate != @1974-12-25T12:34:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testdatenotequaltimezoneoffsetbefore() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate != @1974-12-25T12:34:00-10:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testdatenotequaltimezoneoffsetafter() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate != @1974-12-25T12:34:00+10:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testdatenotequalutc() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate != @1974-12-25T12:34:00Z").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testdatenotequaltimesecond() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate != @T12:14:15").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdatenotequaltimeminute() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate != @T12:14").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdatenotequaltoday() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate < today()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdatetimegreaterthandate1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("now() > Patient.birthDate").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdategreaterthandate() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("today() > Patient.birthDate").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testdatetimegreaterthandate2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("now() > today()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testliteraldatetimetzgreater() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2017-11-05T01:30:00.0-04:00 > @2017-11-05T01:15:00.0-05:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testliteraldatetimetzless() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2017-11-05T01:30:00.0-04:00 < @2017-11-05T01:15:00.0-05:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteraldatetimetzequalfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2017-11-05T01:30:00.0-04:00 = @2017-11-05T01:15:00.0-05:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testliteraldatetimetzequaltrue() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("@2017-11-05T01:30:00.0-04:00 = @2017-11-05T00:30:00.0-05:00").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralunicode() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.first() = 'P\\u0065ter'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testcollectionnotempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.empty().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testcollectionnotequalempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given != {}").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn testexpressions() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.select(given | family).distinct()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Chalmers".to_string()),
        Value::String("Jim".to_string()),
        Value::String("Windsor".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn testexpressionsequal() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.given.count() = 1 + 4").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testnotempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.empty().not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.link.empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralnotonempty() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.not().empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralnottrue() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.not() = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testliteralnotfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("false.not() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not() on integer not specified
#[test]
fn testintegerbooleannottrue() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(0).not() = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testintegerbooleannotfalse() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1).not() = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not() on collection not specified
#[test]
fn testnotinvalid() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("(1|2).not() = false").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}
