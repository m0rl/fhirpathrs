use crate::InterpreterResult;
use crate::context::InterpreterContext;
use crate::datetime::{DatePrecision, DateTimePrecision, TimePrecision};
use crate::value::Value;
use regex::Regex;

pub fn to_string(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let s = match base {
        Value::Date(d, p) => crate::datetime::format_date(*d, *p),
        Value::DateTime(dt, p, tz) => crate::datetime::format_datetime(*dt, *p, tz),
        Value::Time(t, p) => crate::datetime::format_time(*t, *p),
        _ => base.to_string(),
    };
    Ok((Value::String(s), context))
}

#[allow(clippy::cast_precision_loss)]
pub fn to_integer(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Number(n, _) => Value::Number(n.trunc(), 0),
        Value::String(s) => match s.trim().parse::<i64>() {
            Ok(i) => Value::Number(i as f64, 0),
            Err(_) => Value::Null,
        },
        Value::Boolean(b) => Value::Number(if *b { 1.0 } else { 0.0 }, 0),
        _ => Value::Null,
    };
    Ok((value, context))
}

pub fn to_decimal(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Number(n, p) => Value::Number(*n, *p),
        Value::String(s) => match s.trim().parse::<f64>() {
            Ok(n) => Value::Number(n, Value::precision(n)),
            Err(_) => Value::Null,
        },
        Value::Boolean(b) => Value::Number(if *b { 1.0 } else { 0.0 }, 0),
        _ => Value::Null,
    };
    Ok((value, context))
}

pub fn to_boolean(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Boolean(b) => Value::Boolean(*b),
        Value::String(s) => match s.to_lowercase().as_str() {
            "true" | "t" | "yes" | "y" | "1" | "1.0" => Value::Boolean(true),
            "false" | "f" | "no" | "n" | "0" | "0.0" => Value::Boolean(false),
            _ => Value::Null,
        },
        Value::Number(n, _) => {
            if *n == 1.0 {
                Value::Boolean(true)
            } else if *n == 0.0 {
                Value::Boolean(false)
            } else {
                Value::Null
            }
        }
        _ => Value::Null,
    };
    Ok((value, context))
}

pub fn converts_to_integer(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Number(_, p) => *p == 0,
        Value::String(s) => s.trim().parse::<i64>().is_ok(),
        Value::Boolean(_) => true,
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

pub fn converts_to_decimal(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Number(..) | Value::Boolean(_) => true,
        Value::String(s) => s.trim().parse::<f64>().is_ok(),
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

pub fn converts_to_boolean(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Boolean(_) => true,
        Value::String(s) => {
            matches!(
                s.to_lowercase().as_str(),
                "true" | "t" | "yes" | "y" | "1" | "1.0" | "false" | "f" | "no" | "n" | "0" | "0.0"
            )
        }
        Value::Number(n, _) => *n == 1.0 || *n == 0.0,
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

pub fn converts_to_string(base: &Value, context: InterpreterContext) -> InterpreterResult {
    Ok((Value::Boolean(!matches!(base, Value::Null)), context))
}

pub fn converts_to_date(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Date(..) | Value::DateTime(..) => true,
        Value::String(s) => Value::from_date_str(s).is_some(),
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

pub fn converts_to_date_time(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::DateTime(..) | Value::Date(..) => true,
        Value::String(s) => Value::from_datetime_str(s).is_some(),
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

pub fn converts_to_time(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Time(..) | Value::DateTime(..) => true,
        Value::String(s) => Value::from_time_str(s).is_some(),
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

pub fn converts_to_quantity(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Quantity(..) | Value::Number(..) => true,
        Value::String(s) => {
            matches!(
                parse_quantity_string(s, &context),
                Ok((Value::Quantity(..), _))
            )
        }
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

pub fn to_date(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Date(d, p) => Value::Date(*d, *p),
        Value::DateTime(dt, _, _) => Value::Date(dt.date(), DatePrecision::Day),
        Value::String(s) => Value::from_date_str(s).unwrap_or(Value::collection(vec![])),
        _ => Value::collection(vec![]),
    };
    Ok((value, context))
}

pub fn to_date_time(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::DateTime(dt, p, tz) => Value::DateTime(*dt, *p, *tz),
        Value::Date(d, _) => Value::DateTime(
            d.and_hms_opt(0, 0, 0).unwrap_or_default(),
            DateTimePrecision::Day,
            None,
        ),
        Value::String(s) => {
            Value::from_datetime_str(s).unwrap_or(Value::collection(vec![]))
        }
        _ => Value::collection(vec![]),
    };
    Ok((value, context))
}

pub fn to_time(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Time(t, p) => Value::Time(*t, *p),
        Value::DateTime(dt, _, _) => Value::Time(dt.time(), TimePrecision::Second),
        Value::String(s) => Value::from_time_str(s).unwrap_or(Value::collection(vec![])),
        _ => Value::collection(vec![]),
    };
    Ok((value, context))
}

pub fn to_quantity(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Quantity(v, p, u, t) => Value::Quantity(*v, *p, u.clone(), *t),
        Value::Number(n, p) => {
            let unit = args
                .first()
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .unwrap_or_else(|| "1".to_string());
            Value::Quantity(*n, *p, unit, None)
        }
        Value::String(s) => {
            let (val, _) = parse_quantity_string(s, &context)?;
            val
        }
        _ => Value::collection(vec![]),
    };
    Ok((value, context))
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::match_same_arms
)]
pub fn to_long(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Number(n, p) if *p == 0 => {
            let i = *n as i64;
            if (i as f64 - *n).abs() < f64::EPSILON {
                Value::Number(*n, 0)
            } else {
                Value::collection(vec![])
            }
        }
        Value::Number(..) => Value::collection(vec![]),
        Value::String(s) => match s.trim().parse::<i64>() {
            Ok(i) => Value::Number(i as f64, 0),
            Err(_) => Value::collection(vec![]),
        },
        Value::Boolean(b) => Value::Number(if *b { 1.0 } else { 0.0 }, 0),
        _ => Value::collection(vec![]),
    };
    Ok((value, context))
}

pub fn converts_to_long(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Number(_, p) => *p == 0,
        Value::String(s) => s.trim().parse::<i64>().is_ok(),
        Value::Boolean(_) => true,
        _ => false,
    };
    Ok((Value::Boolean(result), context))
}

fn parse_quantity_string(s: &str, context: &InterpreterContext) -> InterpreterResult {
    let s = s.trim();

    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    #[allow(clippy::expect_used)] // Hardcoded regex, validated by tests
    let re = RE.get_or_init(|| {
        Regex::new(r"^(-?\d+\.?\d*)\s*(?:'([^']+)'|(\S+))?$")
            .expect("Invalid regex for parsing quantity strings")
    });

    if let Some(caps) = re.captures(s) {
        let value: f64 = caps
            .get(1)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0.0);

        let unit = caps
            .get(2)
            .or_else(|| caps.get(3))
            .map_or_else(|| "1".to_string(), |m| m.as_str().to_string());

        Ok((Value::Quantity(value, Value::precision(value), unit, None), context.clone()))
    } else {
        Ok((Value::collection(vec![]), context.clone()))
    }
}
