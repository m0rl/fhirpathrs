#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]

use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;

#[test]
fn test_precision_date_year() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024.precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(4.0, 0));
}

#[test]
fn test_precision_date_month() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06.precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(6.0, 0));
}

#[test]
fn test_precision_date_day() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(8.0, 0));
}

#[test]
fn test_precision_datetime_second() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-01-15T10:30:00).precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(14.0, 0));
}

#[test]
fn test_precision_datetime_minute() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-01-15T10:30).precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(12.0, 0));
}

#[test]
fn test_precision_time_second() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T10:30:00).precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(6.0, 0));
}

#[test]
fn test_precision_time_minute() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T10:30).precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(4.0, 0));
}

#[test]
fn test_precision_decimal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("3.14.precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_precision_integer() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("42.precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_low_boundary_date_year() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024.lowBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-01-01T00:00:00.000"
        );
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_low_boundary_date_month() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06.lowBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-06-01T00:00:00.000"
        );
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_low_boundary_date_day() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.lowBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-06-15T00:00:00.000"
        );
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_high_boundary_date_year() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024.highBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-12-31T23:59:59.999"
        );
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_high_boundary_date_month_feb_leap() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-02.highBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-02-29T23:59:59.999"
        );
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_high_boundary_date_month_feb_non_leap() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2023-02.highBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2023-02-28T23:59:59.999"
        );
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_high_boundary_date_day() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.highBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-06-15T23:59:59.999"
        );
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_low_boundary_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T10:30).lowBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Time(t, _) = result {
        assert_eq!(t.format("%H:%M:%S%.3f").to_string(), "10:30:00.000");
    } else {
        panic!("Expected Time, got {:?}", result);
    }
}

#[test]
fn test_high_boundary_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T10:30).highBoundary()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Time(t, _) = result {
        assert_eq!(t.format("%H:%M:%S%.3f").to_string(), "10:30:59.999");
    } else {
        panic!("Expected Time, got {:?}", result);
    }
}

#[test]
fn test_equality_mismatched_date_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024 = @2024-01-01").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_not_equal_mismatched_date_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024 != @2024-01-01").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_equivalent_mismatched_precision_ignores_precision() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024 ~ @2024-01-01").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_not_equivalent_mismatched_precision() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024 !~ @2024-01-01").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_inequality_mismatched_date_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024 < @2024-01-01").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_inequality_same_precision_works() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01 < @2024-06").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_equality_same_precision_year() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024 = @2024").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_equality_same_precision_month() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06 = @2024-06").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_date_precision_preserved_through_arithmetic() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06 + 1 month).precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(6.0, 0));
}

#[test]
fn test_datetime_precision_preserved_through_arithmetic() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-01-15T10:30:00 + 2 hours).precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(14.0, 0));
}

#[test]
fn test_today_precision_is_day() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("today().precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(8.0, 0));
}

#[test]
fn test_now_precision_is_millisecond() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("now().precision()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(17.0, 0));
}

#[test]
fn test_display_date_year_precision() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024.toString()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("2024".to_string()));
}

#[test]
fn test_display_date_month_precision() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06.toString()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("2024-06".to_string()));
}

#[test]
fn test_display_date_day_precision() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.toString()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("2024-06-15".to_string()));
}

#[test]
fn test_year_from_date() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.year()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(2024.0, 0));
}

#[test]
fn test_year_from_year_precision_date() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024.year()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(2024.0, 0));
}

#[test]
fn test_month_from_date() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.month()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(6.0, 0));
}

#[test]
fn test_month_from_year_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024.month()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_day_from_date() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.day()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(15.0, 0));
}

#[test]
fn test_day_from_month_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06.day()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_hour_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:00).hour()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));
}

#[test]
fn test_hour_from_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T14:30:00).hour()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(14.0, 0));
}

#[test]
fn test_hour_from_day_precision_datetime_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.toDateTime().hour()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_minute_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:00).minute()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(30.0, 0));
}

#[test]
fn test_minute_from_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T14:30:00).minute()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(30.0, 0));
}

#[test]
fn test_second_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:45).second()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(45.0, 0));
}

#[test]
fn test_second_from_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T14:30:45).second()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(45.0, 0));
}

#[test]
fn test_millisecond_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:45.123).millisecond()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(123.0, 0));
}

#[test]
fn test_millisecond_from_second_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:45).millisecond()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_timezone_returns_empty_for_naive() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15T10:30:45.timezone()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_timezone_positive_offset() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:30:00+05:00.timezone()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("+05:00".to_string()));
}

#[test]
fn test_timezone_negative_offset() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:30:00-08:00.timezone()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("-08:00".to_string()));
}

#[test]
fn test_timezone_utc() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:30:00Z.timezone()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("+00:00".to_string()));
}

#[test]
fn test_hour_preserves_local_time_with_offset() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:30:00+05:00.hour()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));
}

#[test]
fn test_year_from_number_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("42.year()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_year_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:00).year()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(2024.0, 0));
}

#[test]
fn test_month_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:00).month()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(6.0, 0));
}

#[test]
fn test_day_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@2024-06-15T10:30:00).day()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(15.0, 0));
}

#[test]
fn test_year_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2014-01-05T10:30:00.000.yearOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(2014.0, 0));
}

#[test]
fn test_month_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2014-01-05T10:30:00.000.monthOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));
}

#[test]
fn test_month_of_from_year_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2012.monthOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_day_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2014-01-05T10:30:00.000.dayOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0, 0));
}

#[test]
fn test_hour_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2014-01-05T10:30:00.000.hourOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));
}

#[test]
fn test_minute_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2014-01-05T10:30:00.000.minuteOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(30.0, 0));
}

#[test]
fn test_second_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2014-01-05T10:30:00.000.secondOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_millisecond_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2014-01-05T10:30:00.002.millisecondOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_timezone_offset_of_negative() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2012-01-01T12:30:00.000-07:00.timezoneOffsetOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(-7.0, 0));
}

#[test]
fn test_timezone_offset_of_fractional() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2012-01-01T12:30:00.000+08:45.timezoneOffsetOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(8.75, 2));
}

#[test]
fn test_timezone_offset_of_naive_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2012-01-01T12:30:00.000.timezoneOffsetOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_date_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2012-01-01T12:30:00.000-07:00.dateOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::Date(
            chrono::NaiveDate::from_ymd_opt(2012, 1, 1).unwrap(),
            interpreter::DatePrecision::Day
        )
    );
}

#[test]
fn test_time_of_from_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2012-01-01T12:30:00.000-07:00.timeOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::Time(
            chrono::NaiveTime::from_hms_milli_opt(12, 30, 0, 0).unwrap(),
            interpreter::TimePrecision::Millisecond
        )
    );
}

#[test]
fn test_time_of_from_day_precision_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15.toDateTime().timeOf()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_comparable_same_unit() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("10 'kg'.comparable(20 'kg')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_comparable_compatible_units() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("10 'kg'.comparable(500 'g')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_comparable_incompatible_units() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("10 'kg'.comparable(5 'mL')").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_comparable_non_quantity_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("42.comparable(10)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}
