use crate::InterpreterResult;
use crate::context::InterpreterContext;
use crate::datetime::TimeInterval;
use crate::error::InterpreterError;
use crate::units::{QuantityResult, quantity_add, quantity_cmp, quantity_div, quantity_sub};
use crate::value::Value;
use chrono::{Months, NaiveTime, Timelike};
use parser::{
    AdditiveOp, EqualityOp, InequalityOp, MembershipOp, MultiplicativeOp, OrOp, PolarityOp, TypeOp,
    TypeSpecifier,
};
use std::cmp::Ordering;

pub(crate) fn interpret_indexer(
    base: &Value,
    index: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    let idx = index
        .to_f64()
        .ok_or_else(|| InterpreterError::TypeMismatch("Index must be a number".to_string()))?;

    if idx < 0.0 {
        return Ok((Value::collection(vec![]), context));
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let i = idx.trunc() as usize;

    let value = match base {
        Value::Collection(items) => items
            .get(i)
            .cloned()
            .unwrap_or_else(|| Value::collection(vec![])),
        Value::Null => Value::collection(vec![]),
        other => {
            if i == 0 {
                other.clone()
            } else {
                Value::collection(vec![])
            }
        }
    };
    Ok((value, context))
}

pub(crate) fn interpret_polarity(
    op: &PolarityOp,
    value: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    let result = match op {
        PolarityOp::Plus => value.clone(),
        PolarityOp::Minus => match value {
            Value::Number(n, p) => Value::Number(-n, *p),
            Value::Quantity(v, qp, u, t) => Value::Quantity(-v, *qp, u.clone(), *t),
            _ => {
                return Err(InterpreterError::TypeMismatch(
                    "Cannot apply unary minus to this type".to_string(),
                ));
            }
        },
    };
    Ok((result, context))
}

pub(crate) fn interpret_multiplicative(
    left: &Value,
    op: &MultiplicativeOp,
    right: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    if left.is_null_or_empty() || right.is_null_or_empty() {
        return Ok((Value::collection(vec![]), context));
    }

    let value = match op {
        MultiplicativeOp::Multiply => {
            if let (Value::Quantity(v, _, u, t), Value::Number(n, _)) = (left, right) {
                let result = v * n;
                return Ok((
                    Value::Quantity(result, Value::precision(result), u.clone(), *t),
                    context,
                ));
            }
            if let (Value::Number(n, _), Value::Quantity(v, _, u, t)) = (left, right) {
                let result = n * v;
                return Ok((
                    Value::Quantity(result, Value::precision(result), u.clone(), *t),
                    context,
                ));
            }
            let left_num = left.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Left operand must be a number".to_string())
            })?;
            let right_num = right.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Right operand must be a number".to_string())
            })?;
            let result = left_num * right_num;
            Value::Number(result, Value::precision(result))
        }
        MultiplicativeOp::Divide => {
            if let (Value::Quantity(v, _, u, t), Value::Number(n, _)) = (left, right) {
                if *n == 0.0 {
                    return Ok((Value::collection(vec![]), context));
                }
                let result = v / n;
                return Ok((
                    Value::Quantity(result, Value::precision(result), u.clone(), *t),
                    context,
                ));
            }
            if let (Value::Quantity(..), Value::Quantity(v2, _, _, _)) = (left, right) {
                if *v2 == 0.0 {
                    return Ok((Value::collection(vec![]), context));
                }
                return match quantity_div(left, right) {
                    Some(ratio) => Ok((Value::Number(ratio, Value::precision(ratio)), context)),
                    None => Ok((Value::collection(vec![]), context)),
                };
            }
            let left_num = left.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Left operand must be a number".to_string())
            })?;
            let right_num = right.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Right operand must be a number".to_string())
            })?;
            if right_num == 0.0 {
                return Ok((Value::collection(vec![]), context));
            }
            let result = left_num / right_num;
            Value::Number(result, Value::precision(result))
        }
        MultiplicativeOp::Div => {
            let left_num = left.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Left operand must be a number".to_string())
            })?;
            let right_num = right.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Right operand must be a number".to_string())
            })?;
            if right_num == 0.0 {
                return Ok((Value::collection(vec![]), context));
            }
            Value::Number((left_num / right_num).trunc(), 0)
        }
        MultiplicativeOp::Mod => {
            let left_num = left.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Left operand must be a number".to_string())
            })?;
            let right_num = right.to_f64().ok_or_else(|| {
                InterpreterError::TypeMismatch("Right operand must be a number".to_string())
            })?;
            if right_num == 0.0 {
                return Ok((Value::collection(vec![]), context));
            }
            let result = left_num % right_num;
            Value::Number(result, Value::precision(result))
        }
    };
    Ok((value, context))
}

pub(crate) fn interpret_additive(
    left: &Value,
    op: &AdditiveOp,
    right: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    if !matches!(op, AdditiveOp::Ampersand) && (left.is_null_or_empty() || right.is_null_or_empty())
    {
        return Ok((Value::collection(vec![]), context));
    }

    let left = &left.unwrap_singleton();
    let right = &right.unwrap_singleton();

    let value = match op {
        AdditiveOp::Plus => {
            let interval = if matches!(
                (left, right),
                (
                    Value::Date(..) | Value::DateTime(..) | Value::Time(..),
                    Value::Quantity(..)
                )
            ) {
                Some(right.to_time_interval().ok_or_else(|| {
                    InterpreterError::InvalidOperation(format!(
                        "Unsupported time unit for date arithmetic: {}",
                        right
                    ))
                })?)
            } else {
                right.to_time_interval()
            };
            if let Some(interval) = interval {
                match (left, interval) {
                    (Value::Date(d, p), TimeInterval::Months(m)) => {
                        let result = if m >= 0 {
                            d.checked_add_months(Months::new(m.unsigned_abs()))
                        } else {
                            d.checked_sub_months(Months::new(m.unsigned_abs()))
                        };
                        return result
                            .map(|v| (Value::Date(v, *p), context))
                            .ok_or_else(|| {
                                InterpreterError::InvalidOperation("Date overflow".to_string())
                            });
                    }
                    (Value::Date(d, p), TimeInterval::Duration(dur)) => {
                        return Ok((Value::Date(*d + dur, *p), context));
                    }
                    (Value::DateTime(dt, p, tz), TimeInterval::Months(m)) => {
                        let date = dt.date();
                        let time = dt.time();
                        let new_date = if m >= 0 {
                            date.checked_add_months(Months::new(m.unsigned_abs()))
                        } else {
                            date.checked_sub_months(Months::new(m.unsigned_abs()))
                        };
                        return new_date
                            .map(|d| (Value::DateTime(d.and_time(time), *p, *tz), context))
                            .ok_or_else(|| {
                                InterpreterError::InvalidOperation("DateTime overflow".to_string())
                            });
                    }
                    (Value::DateTime(dt, p, tz), TimeInterval::Duration(dur)) => {
                        return Ok((Value::DateTime(*dt + dur, *p, *tz), context));
                    }
                    (Value::Time(t, p), TimeInterval::Duration(dur)) => {
                        let seconds = t.num_seconds_from_midnight() as i64 + dur.num_seconds();
                        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                        let wrapped = seconds.rem_euclid(86400) as u32;
                        let new_time =
                            NaiveTime::from_num_seconds_from_midnight_opt(wrapped, 0).unwrap_or(*t);
                        return Ok((Value::Time(new_time, *p), context));
                    }
                    _ => {}
                }
            }
            if matches!((left, right), (Value::Quantity(..), Value::Quantity(..))) {
                return match quantity_add(left, right) {
                    QuantityResult::Ok(q) => Ok((q, context)),
                    QuantityResult::Incompatible => Ok((Value::collection(vec![]), context)),
                };
            }
            if let (Some(l), Some(r)) = (left.to_f64(), right.to_f64()) {
                let result = l + r;
                return Ok((Value::Number(result, Value::precision(result)), context));
            }
            if let (Some(l), Some(r)) = (left.as_string(), right.as_string()) {
                return Ok((Value::String(format!("{}{}", l, r)), context));
            }
            return Ok((Value::collection(vec![]), context));
        }
        AdditiveOp::Minus => {
            let interval = if matches!(
                (left, right),
                (
                    Value::Date(..) | Value::DateTime(..) | Value::Time(..),
                    Value::Quantity(..)
                )
            ) {
                Some(right.to_time_interval().ok_or_else(|| {
                    InterpreterError::InvalidOperation(format!(
                        "Unsupported time unit for date arithmetic: {}",
                        right
                    ))
                })?)
            } else {
                right.to_time_interval()
            };
            if let Some(delta) = interval {
                match (left, delta) {
                    (Value::Date(d, p), TimeInterval::Months(m)) => {
                        let result = if m >= 0 {
                            d.checked_sub_months(Months::new(m.unsigned_abs()))
                        } else {
                            d.checked_add_months(Months::new(m.unsigned_abs()))
                        };
                        return result
                            .map(|v| (Value::Date(v, *p), context))
                            .ok_or_else(|| {
                                InterpreterError::InvalidOperation("Date overflow".to_string())
                            });
                    }
                    (Value::Date(d, p), TimeInterval::Duration(dur)) => {
                        return Ok((Value::Date(*d - dur, *p), context));
                    }
                    (Value::DateTime(dt, p, tz), TimeInterval::Months(m)) => {
                        let date = dt.date();
                        let time = dt.time();
                        let new_date = if m >= 0 {
                            date.checked_sub_months(Months::new(m.unsigned_abs()))
                        } else {
                            date.checked_add_months(Months::new(m.unsigned_abs()))
                        };
                        return new_date
                            .map(|d| (Value::DateTime(d.and_time(time), *p, *tz), context))
                            .ok_or_else(|| {
                                InterpreterError::InvalidOperation("DateTime overflow".to_string())
                            });
                    }
                    (Value::DateTime(dt, p, tz), TimeInterval::Duration(dur)) => {
                        return Ok((Value::DateTime(*dt - dur, *p, *tz), context));
                    }
                    (Value::Time(t, p), TimeInterval::Duration(dur)) => {
                        let seconds = t.num_seconds_from_midnight() as i64 - dur.num_seconds();
                        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                        let wrapped = seconds.rem_euclid(86400) as u32;
                        let new_time =
                            NaiveTime::from_num_seconds_from_midnight_opt(wrapped, 0).unwrap_or(*t);
                        return Ok((Value::Time(new_time, *p), context));
                    }
                    _ => {}
                }
            }
            if matches!((left, right), (Value::Quantity(..), Value::Quantity(..))) {
                return match quantity_sub(left, right) {
                    QuantityResult::Ok(q) => Ok((q, context)),
                    QuantityResult::Incompatible => Ok((Value::collection(vec![]), context)),
                };
            }
            if let (Some(l), Some(r)) = (left.to_f64(), right.to_f64()) {
                let result = l - r;
                return Ok((Value::Number(result, Value::precision(result)), context));
            }
            return Err(InterpreterError::TypeMismatch(
                "Cannot subtract these types".to_string(),
            ));
        }
        AdditiveOp::Ampersand => {
            if matches!(left, Value::Collection(items) if items.len() > 1)
                || matches!(right, Value::Collection(items) if items.len() > 1)
            {
                return Err(InterpreterError::InvalidOperation(
                    "& operator requires singleton or empty operands".to_string(),
                ));
            }
            let left_str = if left.is_null_or_empty() {
                String::new()
            } else {
                left.to_string()
            };
            let right_str = if right.is_null_or_empty() {
                String::new()
            } else {
                right.to_string()
            };
            Value::String(format!("{}{}", left_str, right_str))
        }
    };
    Ok((value, context))
}

pub(crate) fn interpret_type(
    value: &Value,
    op: &TypeOp,
    type_spec: &TypeSpecifier,
    context: InterpreterContext,
) -> InterpreterResult {
    let result = match op {
        TypeOp::Is => Value::Boolean(value.is(type_spec)),
        TypeOp::As => value.as_type(type_spec),
    };
    Ok((result, context))
}

pub(crate) fn interpret_union(
    left: &Value,
    right: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    let mut result: Vec<Value> = vec![];

    let mut add_if_unique = |value: Value| {
        if !result.iter().any(|existing| existing.equals(&value)) {
            result.push(value);
        }
    };

    match left {
        Value::Collection(items) => {
            for item in items.iter() {
                add_if_unique(item.clone());
            }
        }
        Value::Null => {}
        _ => add_if_unique(left.clone()),
    }

    match right {
        Value::Collection(items) => {
            for item in items.iter() {
                add_if_unique(item.clone());
            }
        }
        Value::Null => {}
        _ => add_if_unique(right.clone()),
    }

    Ok((Value::collection(result), context))
}

pub(crate) fn interpret_inequality(
    left: &Value,
    op: &InequalityOp,
    right: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    let left = left.unwrap_singleton();
    let right = right.unwrap_singleton();

    if left.is_null_or_empty() || right.is_null_or_empty() {
        return Ok((Value::collection(vec![]), context));
    }

    if let (Value::DateTime(_, _, tz1), Value::DateTime(_, _, tz2)) = (&left, &right)
        && tz1.is_some() != tz2.is_some()
    {
        return Ok((Value::collection(vec![]), context));
    }

    if let (Some(lq), Some(rq)) = (left.as_quantity(), right.as_quantity()) {
        if let Some(ordering) = quantity_cmp(&lq, &rq) {
            let result = match op {
                InequalityOp::Less => ordering == Ordering::Less,
                InequalityOp::LessEqual => ordering != Ordering::Greater,
                InequalityOp::Greater => ordering == Ordering::Greater,
                InequalityOp::GreaterEqual => ordering != Ordering::Less,
            };
            return Ok((Value::Boolean(result), context));
        }
        return Ok((Value::collection(vec![]), context));
    }

    if let Some(ordering) = left.compare_equal(&right) {
        let result = match op {
            InequalityOp::Less => ordering == Ordering::Less,
            InequalityOp::LessEqual => ordering != Ordering::Greater,
            InequalityOp::Greater => ordering == Ordering::Greater,
            InequalityOp::GreaterEqual => ordering != Ordering::Less,
        };
        return Ok((Value::Boolean(result), context));
    }

    if left
        .compare_precision(&right)
        .is_some_and(|ord| ord != Ordering::Equal)
    {
        return Ok((Value::collection(vec![]), context));
    }

    Err(InterpreterError::TypeMismatch(format!(
        "Cannot compare {:?} with {:?}",
        left, right
    )))
}

pub(crate) fn interpret_equality(
    left: &Value,
    op: &EqualityOp,
    right: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    let left = left.unwrap_singleton();
    let right = right.unwrap_singleton();

    if matches!(op, EqualityOp::Equal | EqualityOp::NotEqual)
        && (left.is_null_or_empty() || right.is_null_or_empty())
    {
        return Ok((Value::collection(vec![]), context));
    }

    if matches!(op, EqualityOp::Equal | EqualityOp::NotEqual)
        && left
            .compare_precision(&right)
            .is_some_and(|ord| ord != Ordering::Equal)
    {
        return Ok((Value::collection(vec![]), context));
    }

    if matches!(op, EqualityOp::Equal | EqualityOp::NotEqual)
        && let (Value::DateTime(_, _, tz1), Value::DateTime(_, _, tz2)) = (&left, &right)
        && tz1.is_some() != tz2.is_some()
    {
        return Ok((Value::collection(vec![]), context));
    }

    let left = left.as_quantity().unwrap_or(left);
    let right = right.as_quantity().unwrap_or(right);

    let result = match op {
        EqualityOp::Equal => left.equals(&right),
        EqualityOp::Equivalent => left.equivalent(&right),
        EqualityOp::NotEqual => !left.equals(&right),
        EqualityOp::NotEquivalent => !left.equivalent(&right),
    };

    Ok((Value::Boolean(result), context))
}

pub(crate) fn interpret_membership(
    left: &Value,
    op: &MembershipOp,
    right: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    let result = match op {
        MembershipOp::In => {
            if left.is_null_or_empty() {
                return Ok((Value::collection(vec![]), context));
            }
            if let Value::Collection(items) = left
                && items.len() > 1
            {
                return Err(InterpreterError::InvalidOperation(
                    "in operator requires a singleton on the left side".to_string(),
                ));
            }
            match right {
                Value::Collection(items) => items.iter().any(|item| left.equals(item)),
                _ => left.equals(right),
            }
        }
        MembershipOp::Contains => {
            if right.is_null_or_empty() {
                return Ok((Value::collection(vec![]), context));
            }
            if let Value::Collection(items) = right
                && items.len() > 1
            {
                return Err(InterpreterError::InvalidOperation(
                    "contains operator requires a singleton on the right side".to_string(),
                ));
            }
            match left {
                Value::Collection(items) => items.iter().any(|item| item.equals(right)),
                _ => left.equals(right),
            }
        }
    };

    Ok((Value::Boolean(result), context))
}

pub(crate) fn interpret_or(
    left: &Value,
    op: &OrOp,
    right: &Value,
    context: InterpreterContext,
) -> InterpreterResult {
    let value = match op {
        OrOp::Or => match (left.to_bool(), right.to_bool()) {
            (Some(true), _) | (_, Some(true)) => Value::Boolean(true),
            (Some(false), Some(false)) => Value::Boolean(false),
            _ => Value::collection(vec![]),
        },
        OrOp::Xor => match (left.to_bool(), right.to_bool()) {
            (Some(l), Some(r)) => Value::Boolean(l != r),
            _ => Value::collection(vec![]),
        },
    };
    Ok((value, context))
}
