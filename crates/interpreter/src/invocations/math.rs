use crate::InterpreterResult;
use crate::context::InterpreterContext;
use crate::error::InterpreterError;
use crate::value::Value;

pub fn abs(base: &Value, context: InterpreterContext) -> InterpreterResult {
    if let Value::Quantity(v, p, u, t) = base {
        return Ok((Value::Quantity(v.abs(), *p, u.clone(), *t), context));
    }
    if let Value::Number(n, p) = base {
        return Ok((Value::Number(n.abs(), *p), context));
    }
    Err(InterpreterError::TypeMismatch(
        "abs() requires a numeric value".to_string(),
    ))
}

pub fn ceiling(base: &Value, context: InterpreterContext) -> InterpreterResult {
    if let Value::Quantity(v, _, u, t) = base {
        return Ok((Value::Quantity(v.ceil(), 0, u.clone(), *t), context));
    }
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("ceiling() requires a numeric value".to_string())
    })?;
    Ok((Value::Number(n.ceil(), 0), context))
}

pub fn floor(base: &Value, context: InterpreterContext) -> InterpreterResult {
    if let Value::Quantity(v, _, u, t) = base {
        return Ok((Value::Quantity(v.floor(), 0, u.clone(), *t), context));
    }
    let n = base.to_f64().ok_or_else(|| {
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
    let multiplier = 10_f64.powi(precision);

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let result_precision = if precision <= 0 { 0u8 } else { precision as u8 };

    if let Value::Quantity(v, _, u, t) = base {
        return Ok((
            Value::Quantity(
                (v * multiplier).round() / multiplier,
                result_precision,
                u.clone(),
                *t,
            ),
            context,
        ));
    }
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("round() requires a numeric value".to_string())
    })?;
    Ok((
        Value::Number((n * multiplier).round() / multiplier, result_precision),
        context,
    ))
}

pub fn truncate(base: &Value, context: InterpreterContext) -> InterpreterResult {
    if let Value::Quantity(v, _, u, t) = base {
        return Ok((Value::Quantity(v.trunc(), 0, u.clone(), *t), context));
    }
    let n = base.to_f64().ok_or_else(|| {
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
        let result = n.sqrt();
        Ok((Value::Number(result, Value::precision(result)), context))
    }
}

pub fn exp(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("exp() requires a numeric value".to_string())
    })?;
    let result = n.exp();
    Ok((Value::Number(result, Value::precision(result)), context))
}

pub fn ln(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let n = base.to_f64().ok_or_else(|| {
        InterpreterError::TypeMismatch("ln() requires a numeric value".to_string())
    })?;
    if n <= 0.0 {
        Ok((Value::Null, context))
    } else {
        let result = n.ln();
        Ok((Value::Number(result, Value::precision(result)), context))
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
    let result = n.log(log_base);
    Ok((Value::Number(result, Value::precision(result)), context))
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
    if result.is_nan() || result.is_infinite() {
        Ok((Value::Null, context))
    } else {
        Ok((Value::Number(result, Value::precision(result)), context))
    }
}
