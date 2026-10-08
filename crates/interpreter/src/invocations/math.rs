use crate::InterpreterResult;
use crate::context::InterpreterContext;
use crate::decimal::Decimal;
use crate::error::InterpreterError;
use crate::value::Value;

pub fn abs(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Quantity(v, p, u, t) => v.checked_abs().map_or_else(
            || Value::collection(vec![]),
            |v| Value::Quantity(v, *p, u.clone(), *t),
        ),
        Value::Number(n, p) => n
            .checked_abs()
            .map_or_else(|| Value::collection(vec![]), |n| Value::Number(n, *p)),
        _ => {
            return Err(InterpreterError::TypeMismatch(
                "abs() requires a numeric value".to_string(),
            ));
        }
    };
    Ok((value, context))
}

pub fn ceiling(base: &Value, context: InterpreterContext) -> InterpreterResult {
    if let Value::Quantity(v, _, u, t) = base {
        return Ok((
            v.ceil().map_or_else(
                || Value::collection(vec![]),
                |v| Value::Quantity(v, 0, u.clone(), *t),
            ),
            context,
        ));
    }
    let n = base.to_decimal().ok_or_else(|| {
        InterpreterError::TypeMismatch("ceiling() requires a numeric value".to_string())
    })?;
    Ok((
        n.ceil()
            .map_or_else(|| Value::collection(vec![]), |n| Value::Number(n, 0)),
        context,
    ))
}

pub fn floor(base: &Value, context: InterpreterContext) -> InterpreterResult {
    if let Value::Quantity(v, _, u, t) = base {
        return Ok((Value::Quantity(v.floor(), 0, u.clone(), *t), context));
    }
    let n = base.to_decimal().ok_or_else(|| {
        InterpreterError::TypeMismatch("floor() requires a numeric value".to_string())
    })?;
    Ok((Value::Number(n.floor(), 0), context))
}

pub fn round(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let precision = if args.is_empty() {
        0
    } else {
        args[0].to_i32().ok_or_else(|| {
            InterpreterError::TypeMismatch("round() precision must be a number".to_string())
        })?
    };
    let result_precision = if precision <= 0 {
        0
    } else {
        u8::try_from(precision).map_or(Decimal::MAX_PRECISION, |p| p.min(Decimal::MAX_PRECISION))
    };

    if let Value::Quantity(v, _, u, t) = base {
        return Ok((
            v.round_dp(precision).map_or_else(
                || Value::collection(vec![]),
                |v| Value::Quantity(v, result_precision, u.clone(), *t),
            ),
            context,
        ));
    }
    let n = base.to_decimal().ok_or_else(|| {
        InterpreterError::TypeMismatch("round() requires a numeric value".to_string())
    })?;
    Ok((
        n.round_dp(precision).map_or_else(
            || Value::collection(vec![]),
            |n| Value::Number(n, result_precision),
        ),
        context,
    ))
}

pub fn truncate(base: &Value, context: InterpreterContext) -> InterpreterResult {
    if let Value::Quantity(v, _, u, t) = base {
        return Ok((Value::Quantity(v.trunc(), 0, u.clone(), *t), context));
    }
    let n = base.to_decimal().ok_or_else(|| {
        InterpreterError::TypeMismatch("truncate() requires a numeric value".to_string())
    })?;
    Ok((Value::Number(n.trunc(), 0), context))
}

pub fn sqrt(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("sqrt() requires a numeric value".to_string())
    })?;
    if n < 0.0 {
        Ok((Value::Null, context))
    } else {
        Ok((Value::from_f64_result(n.sqrt()), context))
    }
}

pub fn exp(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("exp() requires a numeric value".to_string())
    })?;
    let result = n.exp();
    if result == 0.0 {
        Ok((Value::Null, context))
    } else {
        Ok((Value::from_f64_result(result), context))
    }
}

pub fn ln(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("ln() requires a numeric value".to_string())
    })?;
    if n <= 0.0 {
        Ok((Value::Null, context))
    } else {
        Ok((Value::from_f64_result(n.ln()), context))
    }
}

pub fn log(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("log() requires a numeric value".to_string())
    })?;
    if n <= 0.0 {
        return Ok((Value::Null, context));
    }
    let log_base = if args.is_empty() {
        10.0
    } else {
        args[0].to_f64().ok_or_else(|| {
            InterpreterError::TypeMismatch("log() base must be a number".to_string())
        })?
    };
    if log_base <= 0.0 || log_base == 1.0 {
        return Ok((Value::Null, context));
    }
    Ok((Value::from_f64_result(n.log(log_base)), context))
}

pub fn power(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("power() requires a numeric value".to_string())
    })?;
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "power() requires an exponent argument".to_string(),
        ));
    }
    let exponent = args[0].to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("power() exponent must be a number".to_string())
    })?;
    let result = n.powf(exponent);
    if !result.is_finite() || (result == 0.0 && n != 0.0) {
        Ok((Value::Null, context))
    } else {
        Ok((Value::from_f64_result(result), context))
    }
}
