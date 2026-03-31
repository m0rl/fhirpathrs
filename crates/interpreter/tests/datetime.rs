#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]

use chrono::{Datelike, NaiveDate};
use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;


#[test]
fn test_date_less_than() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-01 < @2024-12-31").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_date_greater_than() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-12-31 > @2024-01-01").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_date_less_equal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15 <= @2024-01-15").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_date_greater_equal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15 >= @2024-06-01").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_date_equal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-03-15 = @2024-03-15").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_date_not_equal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-01 != @2024-12-31").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_datetime_comparison() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00 > @2024-01-15T09:00:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_datetime_less_than() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T08:30:00 < @2024-01-15T09:00:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_datetime_equal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15T12:30:45 = @2024-06-15T12:30:45").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_time_less_equal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@T10:30:00 <= @T10:30:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_time_greater_than() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@T15:00:00 > @T10:00:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_date_plus_days() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-01 + 30 days").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 1, 31).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_plus_weeks() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-01 + 1 week").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 1, 8).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_minus_days() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-31 - 15 days").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 1, 16).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_plus_one_month() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15 + 1 month").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 2, 15).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_plus_month_clamps_to_valid_day_leap_year() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-31 + 1 month").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 2, 29).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_plus_month_clamps_to_valid_day_non_leap_year() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2023-01-31 + 1 month").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2023, 2, 28).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_minus_month_clamps() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-03-31 - 1 month").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 2, 29).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_plus_one_year() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15 + 1 year").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2025, 1, 15).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_leap_day_plus_year_clamps() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-02-29 + 1 year").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2025, 2, 28).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_plus_multiple_months() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15 + 6 months").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 7, 15).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_date_minus_years() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06-15 - 10 years").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        assert_eq!(d, NaiveDate::from_ymd_opt(2014, 6, 15).expect("valid date"));
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_datetime_plus_hours() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00 + 2 hours").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert_eq!(dt.date().year(), 2024);
        assert_eq!(dt.date().month(), 1);
        assert_eq!(dt.date().day(), 15);
        assert_eq!(dt.format("%H:%M:%S").to_string(), "12:00:00");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_datetime_plus_month() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-31T14:30:00 + 1 month").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert_eq!(
            dt.date(),
            NaiveDate::from_ymd_opt(2024, 2, 29).expect("valid date")
        );
        assert_eq!(dt.format("%H:%M:%S").to_string(), "14:30:00");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_today_minus_years() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("today() - 38 years").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        let today = context.evaluation_timestamp.date_naive();
        let expected_year = today.year() - 38;
        assert_eq!(d.year(), expected_year);
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_now_plus_days() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("now() + 7 days").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        let now = context.evaluation_timestamp.naive_utc();
        let diff = dt.signed_duration_since(now);
        assert_eq!(diff.num_days(), 7);
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_datetime_with_positive_offset_stores_local_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00+05:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert_eq!(dt.format("%H:%M:%S").to_string(), "10:00:00");
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_datetime_with_negative_offset_stores_local_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00-05:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert_eq!(dt.format("%H:%M:%S").to_string(), "10:00:00");
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_datetime_with_z_suffix() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00Z").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert_eq!(dt.format("%H:%M:%S").to_string(), "10:00:00");
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_datetime_offset_preserves_local_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T02:00:00+05:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert_eq!(dt.format("%H:%M:%S").to_string(), "02:00:00");
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_datetime_without_timezone() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert_eq!(dt.format("%H:%M:%S").to_string(), "10:00:00");
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_duration_date_years() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2020.duration(@2024)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(4.0, 0, "year".to_string(), None));
}

#[test]
fn test_duration_date_months() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01.duration(@2024-06)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(5.0, 0, "month".to_string(), None));
}

#[test]
fn test_duration_date_days() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-01.duration(@2024-01-31)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(30.0, 0, "day".to_string(), None));
}

#[test]
fn test_duration_datetime_seconds() {
    let context = InterpreterContext::new(Value::Null);
    let expr =
        parse("(@2024-01-01T00:00:00).duration(@2024-01-01T06:00:00)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(21600.0, 0, "second".to_string(), None));
}

#[test]
fn test_duration_negative() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-06.duration(@2024-01)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(-5.0, 0, "month".to_string(), None));
}

#[test]
fn test_duration_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("(@T10:00:00).duration(@T14:30:00)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(16200.0, 0, "second".to_string(), None));
}

#[test]
fn test_duration_mismatched_types_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-01.duration(@T10:00:00)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_difference_date_days() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-01.difference(@2024-02-01)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(31.0, 0, "day".to_string(), None));
}

#[test]
fn test_difference_datetime_seconds() {
    let context = InterpreterContext::new(Value::Null);
    let expr =
        parse("(@2024-01-01T00:00:00).difference(@2024-01-01T00:01:30)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(90.0, 0, "second".to_string(), None));
}

#[test]
fn test_difference_date_years_physical() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2020.difference(@2024)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Quantity(4.0, 0, "year".to_string(), None));
}

#[test]
fn test_datetime_same_instant_different_offsets_are_equal() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00+05:00 = @2024-01-15T05:00:00Z").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_datetime_mixed_tz_equality_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00+05:00 = @2024-01-15T10:00:00").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_datetime_mixed_tz_inequality_returns_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("@2024-01-15T10:00:00+05:00 < @2024-01-15T10:00:00").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_datetime_same_offset_comparison() {
    let context = InterpreterContext::new(Value::Null);
    let expr =
        parse("@2024-01-15T10:00:00+05:00 < @2024-01-15T11:00:00+05:00").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}
