#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]

use chrono::{NaiveDate, NaiveTime};
use interpreter::{
    DatePrecision, DateTimePrecision, InterpreterContext, TimePrecision, Value, interpret,
};
use parser::parse;

#[test]
fn test_converts_to_date() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'2024-01-15'.convertsToDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'2024-06'.convertsToDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'2024'.convertsToDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'invalid'.convertsToDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("@2024-01-15.convertsToDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("123.convertsToDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_converts_to_date_time() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'2024-01-15T10:30:00'.convertsToDateTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'invalid'.convertsToDateTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("@2024-01-15.convertsToDateTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("123.convertsToDateTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_converts_to_time() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'10:30:00'.convertsToTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'14:45'.convertsToTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'invalid'.convertsToTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("(@T10:30:00).convertsToTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("123.convertsToTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_converts_to_quantity() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("42.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("10 'kg'.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'5.5 \\'mg\\''.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'5 day'.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'5.5 mg'.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'100'.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'invalid'.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("true.convertsToQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_to_integer() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'123'.toInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(123.0, 0));

    let expr = parse("'12.5'.toInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let expr = parse("12.7.toInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(12.0, 0));

    let expr = parse("true.toInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("false.toInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_to_decimal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'3.14'.toDecimal()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.14, 2));

    let expr = parse("'abc'.toDecimal()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);
}

#[test]
fn test_to_boolean() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'true'.toBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'false'.toBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'yes'.toBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'no'.toBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("1.toBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("0.toBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'invalid'.toBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);
}

#[test]
fn test_converts_to_integer() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'123'.convertsToInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'12.5'.convertsToInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'abc'.convertsToInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("12.convertsToInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("12.5.convertsToInteger()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_converts_to_decimal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'3.14'.convertsToDecimal()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'abc'.convertsToDecimal()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("42.convertsToDecimal()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_converts_to_boolean() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'true'.convertsToBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'yes'.convertsToBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'invalid'.convertsToBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("1.convertsToBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("0.convertsToBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("2.convertsToBoolean()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_converts_to_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("123.convertsToString()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("true.convertsToString()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'abc'.convertsToString()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_to_date_from_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'2024-01-15'.toDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Date(
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            DatePrecision::Day
        )
    );

    let expr = parse("'2024-06'.toDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Date(
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            DatePrecision::Month
        )
    );

    let expr = parse("'2024'.toDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Date(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            DatePrecision::Year
        )
    );

    let expr = parse("'invalid'.toDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_to_date_from_datetime() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(@2024-01-15T10:30:00).toDate()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Date(
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            DatePrecision::Day
        )
    );
}

#[test]
fn test_to_date_time_from_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'2024-01-15T10:30:00'.toDateTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::DateTime(
            NaiveDate::from_ymd_opt(2024, 1, 15)
                .unwrap()
                .and_hms_opt(10, 30, 0)
                .unwrap(),
            DateTimePrecision::Second,
            None,
        )
    );

    let expr = parse("'invalid'.toDateTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_to_date_time_from_date() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("@2024-01-15.toDateTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::DateTime(
            NaiveDate::from_ymd_opt(2024, 1, 15)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            DateTimePrecision::Day,
            None,
        )
    );
}

#[test]
fn test_to_time_from_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'10:30:00'.toTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Time(
            NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
            TimePrecision::Second
        )
    );

    let expr = parse("'14:45'.toTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Time(
            NaiveTime::from_hms_opt(14, 45, 0).unwrap(),
            TimePrecision::Minute
        )
    );

    let expr = parse("'invalid'.toTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_to_time_from_datetime() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(@2024-01-15T10:30:45).toTime()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Time(
            NaiveTime::from_hms_opt(10, 30, 45).unwrap(),
            TimePrecision::Second
        )
    );
}

#[test]
fn test_to_quantity_from_number() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("42.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(42.0, 0, "1".to_string(), None));

    let expr = parse("10.toQuantity('kg')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(10.0, 0, "kg".to_string(), None));
}

#[test]
fn test_to_quantity_from_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'10 \\'kg\\''.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(10.0, 0, "kg".to_string(), None));

    let expr = parse("'5.5 \\'mg\\''.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(5.5, 1, "mg".to_string(), None));

    let expr = parse("'5 day'.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(5.0, 0, "day".to_string(), None));

    let expr = parse("'5.5 mg'.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("'100'.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(100.0, 0, "1".to_string(), None));

    let expr = parse("'invalid'.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_to_quantity_preserves_existing() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("10 'cm'.toQuantity()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(10.0, 0, "cm".to_string(), None));
}

#[test]
fn test_to_long_from_integer() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("42.toLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_to_long_from_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'12345678901234'.toLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(12345678901234.0, 0));

    let expr = parse("'not_a_number'.toLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_to_long_from_boolean() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true.toLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("false.toLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_to_long_from_decimal_returns_empty() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("3.14.toLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_converts_to_long() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("42.convertsToLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("3.14.convertsToLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'999'.convertsToLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'abc'.convertsToLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("true.convertsToLong()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}
