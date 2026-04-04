use crate::context::InterpreterContext;
use crate::datetime;
use crate::datetime::{DatePrecision, DateTimePrecision, TimePrecision};
use crate::error::InterpreterError;
use crate::units::quantity_cmp_units;
use crate::value::{MAX_DECIMAL_PRECISION, Value};
use crate::{InterpreterResult, QuantityType};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveTime, Timelike};
use std::collections::HashMap;

pub fn now(_base: &Value, context: InterpreterContext) -> InterpreterResult {
    Ok((
        Value::DateTime(
            context.evaluation_timestamp.naive_utc(),
            DateTimePrecision::Millisecond,
            FixedOffset::east_opt(0),
        ),
        context,
    ))
}

pub fn today(_base: &Value, context: InterpreterContext) -> InterpreterResult {
    Ok((
        Value::Date(
            context.evaluation_timestamp.date_naive(),
            DatePrecision::Day,
        ),
        context,
    ))
}

pub fn time_of_day(_base: &Value, context: InterpreterContext) -> InterpreterResult {
    Ok((
        Value::Time(context.evaluation_timestamp.time(), TimePrecision::Second),
        context,
    ))
}

pub fn value_type(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    let mut results = Vec::new();
    for item in &items {
        let (namespace, name) = match item {
            Value::Quantity(_, _, _, Some(qt)) => ("FHIR", qt.as_str().to_string()),
            Value::Object(obj) => match obj.get("resourceType") {
                Some(Value::String(rt)) => ("FHIR", rt.clone()),
                _ => continue,
            },
            _ => match item.type_name() {
                Some(n) => ("System", n.to_string()),
                None => continue,
            },
        };
        results.push(Value::object(HashMap::from([
            (
                "namespace".to_string(),
                Value::String(namespace.to_string()),
            ),
            ("name".to_string(), Value::String(name)),
        ])));
    }
    Ok((Value::collection(results), context))
}

pub fn precision(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Date(_, p) => match p {
            DatePrecision::Year => Value::Number(4.0, 0),
            DatePrecision::Month => Value::Number(6.0, 0),
            DatePrecision::Day => Value::Number(8.0, 0),
        },
        Value::DateTime(_, p, _) => match p {
            DateTimePrecision::Year => Value::Number(4.0, 0),
            DateTimePrecision::Month => Value::Number(6.0, 0),
            DateTimePrecision::Day => Value::Number(8.0, 0),
            DateTimePrecision::Hour => Value::Number(10.0, 0),
            DateTimePrecision::Minute => Value::Number(12.0, 0),
            DateTimePrecision::Second => Value::Number(14.0, 0),
            DateTimePrecision::Millisecond => Value::Number(17.0, 0),
        },
        Value::Time(_, p) => match p {
            TimePrecision::Hour => Value::Number(2.0, 0),
            TimePrecision::Minute => Value::Number(4.0, 0),
            TimePrecision::Second => Value::Number(6.0, 0),
            TimePrecision::Millisecond => Value::Number(9.0, 0),
        },
        Value::Number(_, p) =>
        {
            #[allow(clippy::cast_lossless)]
            Value::Number(*p as f64, 0)
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn low_boundary(
    base: &Value,
    args: &[Value],
    context: InterpreterContext,
) -> InterpreterResult {
    let result = match base {
        Value::Number(n, vp) | Value::Quantity(n, vp, ..) => {
            let precision = if args.is_empty() {
                MAX_DECIMAL_PRECISION
            } else {
                let arg_precision = args[0].to_f64().ok_or_else(|| {
                    InterpreterError::TypeMismatch(
                        "boundary precision must be a number".to_string(),
                    )
                })? as i32;
                if !(0..=i32::from(MAX_DECIMAL_PRECISION)).contains(&arg_precision) {
                    return Ok((Value::collection(vec![]), context));
                }
                arg_precision as u8
            };
            let result = if !args.is_empty() && precision <= *vp {
                let half = 0.5 * 10.0_f64.powi(-i32::from(precision));
                let boundary = *n - half;
                let scale = 10.0_f64.powi(i32::from(precision));
                (boundary * scale).trunc() / scale
            } else {
                *n - 0.5 * 10.0_f64.powi(-i32::from(*vp))
            };
            match base {
                Value::Quantity(_, _, u, t) => Value::Quantity(result, precision, u.clone(), *t),
                _ => Value::Number(result, precision),
            }
        }
        Value::Date(d, p) => {
            let target = if args.is_empty() {
                DateTimePrecision::Millisecond
            } else {
                match DateTimePrecision::from_ord(args[0].to_f64().ok_or_else(|| {
                    InterpreterError::TypeMismatch(
                        "boundary precision must be a number".to_string(),
                    )
                })? as i32)
                {
                    Some(v) => v,
                    None => return Ok((Value::collection(vec![]), context)),
                }
            };
            let date = match p {
                DatePrecision::Year => NaiveDate::from_ymd_opt(d.year(), 1, 1).unwrap_or(*d),
                DatePrecision::Month => {
                    NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap_or(*d)
                }
                DatePrecision::Day => *d,
            };
            let dt = date.and_hms_milli_opt(0, 0, 0, 0).unwrap_or_default();
            Value::DateTime(target.trunc(dt), target, None)
        }
        Value::DateTime(dt, p, tz) => {
            let target = if args.is_empty() {
                DateTimePrecision::Millisecond
            } else {
                match DateTimePrecision::from_ord(args[0].to_f64().ok_or_else(|| {
                    InterpreterError::TypeMismatch(
                        "boundary precision must be a number".to_string(),
                    )
                })? as i32)
                {
                    Some(v) => v,
                    None => return Ok((Value::collection(vec![]), context)),
                }
            };
            let p = if *p == DateTimePrecision::Hour { DateTimePrecision::Minute } else { *p };
            let low = p.min(target).trunc(*dt);
            let out_tz = if target == DateTimePrecision::Millisecond && tz.is_none() {
                FixedOffset::east_opt(14 * 3600)
            } else {
                *tz
            };
            Value::DateTime(low, target, out_tz)
        }
        Value::Time(t, p) => {
            let low = match p {
                TimePrecision::Hour => {
                    NaiveTime::from_hms_milli_opt(t.hour(), 0, 0, 0).unwrap_or(*t)
                }
                TimePrecision::Minute => {
                    NaiveTime::from_hms_milli_opt(t.hour(), t.minute(), 0, 0).unwrap_or(*t)
                }
                TimePrecision::Second => {
                    NaiveTime::from_hms_milli_opt(t.hour(), t.minute(), t.second(), 0).unwrap_or(*t)
                }
                TimePrecision::Millisecond => *t,
            };
            Value::Time(low, TimePrecision::Millisecond)
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn high_boundary(
    base: &Value,
    args: &[Value],
    context: InterpreterContext,
) -> InterpreterResult {
    let result = match base {
        Value::Number(n, vp) | Value::Quantity(n, vp, ..) => {
            let precision = if args.is_empty() {
                MAX_DECIMAL_PRECISION
            } else {
                let arg_precision = args[0].to_f64().ok_or_else(|| {
                    InterpreterError::TypeMismatch(
                        "boundary precision must be a number".to_string(),
                    )
                })? as i32;
                if !(0..=i32::from(MAX_DECIMAL_PRECISION)).contains(&arg_precision) {
                    return Ok((Value::collection(vec![]), context));
                }
                arg_precision as u8
            };
            let result = if !args.is_empty() && precision <= *vp {
                let scale = 10.0_f64.powi(i32::from(precision));
                let scaled = *n * scale;
                if (scaled - scaled.round()).abs() < 1e-10 {
                    scaled.round() / scale + 10.0_f64.powi(-i32::from(precision))
                } else {
                    let half = 0.5 * 10.0_f64.powi(-i32::from(precision));
                    ((n + half) * scale).trunc() / scale
                }
            } else {
                *n + 0.5 * 10.0_f64.powi(-i32::from(*vp))
            };
            match base {
                Value::Quantity(_, _, u, t) => Value::Quantity(result, precision, u.clone(), *t),
                _ => Value::Number(result, precision),
            }
        }
        Value::Date(d, p) => {
            let target = if args.is_empty() {
                DateTimePrecision::Millisecond
            } else {
                match DateTimePrecision::from_ord(args[0].to_f64().ok_or_else(|| {
                    InterpreterError::TypeMismatch(
                        "boundary precision must be a number".to_string(),
                    )
                })? as i32)
                {
                    Some(v) => v,
                    None => return Ok((Value::collection(vec![]), context)),
                }
            };
            let date = match p {
                DatePrecision::Year => NaiveDate::from_ymd_opt(d.year(), 12, 31).unwrap_or(*d),
                DatePrecision::Month => datetime::last_day_of_month(d.year(), d.month()),
                DatePrecision::Day => *d,
            };
            let dt = date.and_hms_milli_opt(23, 59, 59, 999).unwrap_or_default();
            Value::DateTime(target.trunc(dt), target, None)
        }
        Value::DateTime(dt, p, tz) => {
            let target = if args.is_empty() {
                DateTimePrecision::Millisecond
            } else {
                match DateTimePrecision::from_ord(args[0].to_f64().ok_or_else(|| {
                    InterpreterError::TypeMismatch(
                        "boundary precision must be a number".to_string(),
                    )
                })? as i32)
                {
                    Some(v) => v,
                    None => return Ok((Value::collection(vec![]), context)),
                }
            };
            let p = if *p == DateTimePrecision::Hour { DateTimePrecision::Minute } else { *p };
            let high = p.ceil(*dt);
            let out_tz = if target == DateTimePrecision::Millisecond && tz.is_none() {
                FixedOffset::east_opt(-12 * 3600)
            } else {
                *tz
            };
            Value::DateTime(target.trunc(high), target, out_tz)
        }
        Value::Time(t, p) => {
            let high = match p {
                TimePrecision::Hour => {
                    NaiveTime::from_hms_milli_opt(t.hour(), 59, 59, 999).unwrap_or(*t)
                }
                TimePrecision::Minute => {
                    NaiveTime::from_hms_milli_opt(t.hour(), t.minute(), 59, 999).unwrap_or(*t)
                }
                TimePrecision::Second => {
                    NaiveTime::from_hms_milli_opt(t.hour(), t.minute(), t.second(), 999)
                        .unwrap_or(*t)
                }
                TimePrecision::Millisecond => *t,
            };
            Value::Time(high, TimePrecision::Millisecond)
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn year(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Date(d, _) => Value::Number(f64::from(d.year()), 0),
        Value::DateTime(dt, _, _) => Value::Number(f64::from(dt.date().year()), 0),
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

#[allow(clippy::cast_precision_loss)]
pub fn month(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Date(_, DatePrecision::Year) | Value::DateTime(_, DateTimePrecision::Year, _) => {
            Value::collection(vec![])
        }
        Value::Date(d, _) => Value::Number(f64::from(d.month()), 0),
        Value::DateTime(dt, _, _) => Value::Number(f64::from(dt.date().month()), 0),
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn day(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Date(_, DatePrecision::Year | DatePrecision::Month)
        | Value::DateTime(_, DateTimePrecision::Year | DateTimePrecision::Month, _) => {
            Value::collection(vec![])
        }
        Value::Date(d, _) => Value::Number(f64::from(d.day()), 0),
        Value::DateTime(dt, _, _) => Value::Number(f64::from(dt.date().day()), 0),
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn hour(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::DateTime(
            _,
            DateTimePrecision::Year | DateTimePrecision::Month | DateTimePrecision::Day,
            _,
        ) => Value::collection(vec![]),
        Value::DateTime(dt, _, _) => Value::Number(f64::from(dt.time().hour()), 0),
        Value::Time(t, _) => Value::Number(f64::from(t.hour()), 0),
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn minute(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::DateTime(
            _,
            DateTimePrecision::Year
            | DateTimePrecision::Month
            | DateTimePrecision::Day
            | DateTimePrecision::Hour,
            _,
        )
        | Value::Time(_, TimePrecision::Hour) => Value::collection(vec![]),
        Value::DateTime(dt, _, _) => Value::Number(f64::from(dt.time().minute()), 0),
        Value::Time(t, _) => Value::Number(f64::from(t.minute()), 0),
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn second(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::DateTime(
            _,
            DateTimePrecision::Year
            | DateTimePrecision::Month
            | DateTimePrecision::Day
            | DateTimePrecision::Hour
            | DateTimePrecision::Minute,
            _,
        )
        | Value::Time(_, TimePrecision::Hour | TimePrecision::Minute) => Value::collection(vec![]),
        Value::DateTime(dt, _, _) => Value::Number(f64::from(dt.time().second()), 0),
        Value::Time(t, _) => Value::Number(f64::from(t.second()), 0),
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn millisecond(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::DateTime(dt, DateTimePrecision::Millisecond, _) => {
            Value::Number(f64::from(dt.and_utc().timestamp_subsec_millis() % 1000), 0)
        }
        Value::Time(t, TimePrecision::Millisecond) => {
            Value::Number(f64::from((t.nanosecond() / 1_000_000) % 1000), 0)
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn timezone(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::DateTime(_, _, Some(offset)) => {
            let total_seconds = offset.local_minus_utc();
            if total_seconds == 0 {
                Value::String("+00:00".to_string())
            } else {
                let hours = total_seconds / 3600;
                let minutes = (total_seconds.abs() % 3600) / 60;
                Value::String(format!("{:+03}:{:02}", hours, minutes))
            }
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn comparable(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "comparable() requires an argument".to_string(),
        ));
    }
    let result = match (base, &args[0]) {
        (Value::Quantity(_, _, u1, t1), Value::Quantity(_, _, u2, t2)) if t1 == t2 => {
            Value::Boolean(quantity_cmp_units(u1, u2))
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn duration(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "duration() requires an argument".to_string(),
        ));
    }
    let result = match (base, &args[0]) {
        (Value::Date(d1, p1), Value::Date(d2, p2)) => {
            let coarsest = (*p1).min(*p2);
            datetime::calendar_duration_date(*d1, coarsest, *d2)
        }
        (Value::DateTime(dt1, p1, tz1), Value::DateTime(dt2, p2, tz2)) => {
            let coarsest = (*p1).min(*p2);
            let ndt1 = datetime::normalize_dt(*dt1, tz1);
            let ndt2 = datetime::normalize_dt(*dt2, tz2);
            datetime::calendar_duration_datetime(ndt1, coarsest, ndt2)
        }
        (Value::Time(t1, p1), Value::Time(t2, p2)) => {
            let coarsest = (*p1).min(*p2);
            datetime::calendar_duration_time(*t1, coarsest, *t2)
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn difference(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "difference() requires an argument".to_string(),
        ));
    }
    let result = match (base, &args[0]) {
        (Value::Date(d1, p1), Value::Date(d2, p2)) => {
            let coarsest = (*p1).min(*p2);
            datetime::physical_difference_date(*d1, coarsest, *d2)
        }
        (Value::DateTime(dt1, p1, tz1), Value::DateTime(dt2, p2, tz2)) => {
            let coarsest = (*p1).min(*p2);
            let ndt1 = datetime::normalize_dt(*dt1, tz1);
            let ndt2 = datetime::normalize_dt(*dt2, tz2);
            datetime::physical_difference_datetime(ndt1, coarsest, ndt2)
        }
        (Value::Time(t1, p1), Value::Time(t2, p2)) => {
            let coarsest = (*p1).min(*p2);
            datetime::calendar_duration_time(*t1, coarsest, *t2)
        }
        _ => Value::collection(vec![]),
    };
    Ok((result, context))
}

pub fn resolve_field(obj: &HashMap<String, Value>, member: &str) -> Option<Value> {
    obj.get(member)
        .cloned()
        .or_else(|| resolve_polymorphic(obj, member))
}

fn resolve_polymorphic(obj: &HashMap<String, Value>, member: &str) -> Option<Value> {
    for (key, raw) in obj {
        if key.len() > member.len()
            && key.starts_with(member)
            && key.as_bytes()[member.len()].is_ascii_uppercase()
        {
            let type_suffix = &key[member.len()..];
            return match type_suffix {
                "Date" => match raw {
                    Value::Date(..) => Some(raw.clone()),
                    Value::String(s) => Value::from_date_str(s),
                    _ => None,
                },
                "DateTime" | "Instant" => match raw {
                    Value::DateTime(..) => Some(raw.clone()),
                    Value::String(s) => Value::from_datetime_str(s),
                    _ => None,
                },
                "Time" => match raw {
                    Value::Time(..) => Some(raw.clone()),
                    Value::String(s) => Value::from_time_str(s),
                    _ => None,
                },
                "Quantity" | "Age" | "Count" | "Distance" | "Duration" | "Money"
                | "SimpleQuantity" => match raw {
                    Value::Quantity(_, _, _, t) if t.is_some_and(|t| t.as_str() == type_suffix) => {
                        Some(raw.clone())
                    }
                    Value::Quantity(v, p, c, None) => Some(Value::Quantity(
                        *v,
                        *p,
                        c.clone(),
                        QuantityType::from_suffix(type_suffix),
                    )),
                    Value::Object(obj) => {
                        if let Some((num, num_p, code)) =
                            match (obj.get("value"), obj.get("code"), obj.get("unit")) {
                                (Some(Value::Number(n, p)), Some(Value::String(c)), _) => {
                                    Some((*n, *p, c.clone()))
                                }
                                (Some(Value::Number(n, p)), _, Some(Value::String(u))) => {
                                    Some((*n, *p, u.clone()))
                                }
                                _ => None,
                            }
                        {
                            Some(Value::Quantity(
                                num,
                                num_p,
                                code,
                                QuantityType::from_suffix(type_suffix),
                            ))
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                "Boolean" => match raw {
                    Value::Boolean(_) => Some(raw.clone()),
                    _ => None,
                },
                "Integer" | "PositiveInt" | "UnsignedInt" | "Integer64" | "Decimal" => match raw {
                    Value::Number(..) => Some(raw.clone()),
                    _ => None,
                },
                "String" | "Code" | "Id" | "Markdown" | "Base64Binary" | "Xhtml" | "Uri"
                | "Url" | "Uuid" | "Canonical" | "Oid" => match raw {
                    Value::String(_) => Some(raw.clone()),
                    _ => None,
                },
                _ => None,
            };
        }
    }
    None
}
