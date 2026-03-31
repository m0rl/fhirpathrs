use crate::InterpreterResult;
use crate::context::InterpreterContext;
use crate::error::InterpreterError;
use crate::units::{QuantityResult, quantity_add, quantity_cmp};
use crate::value::Value;

pub fn all_true(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    if items.is_empty() {
        return Ok((Value::Boolean(true), context));
    }
    for item in items {
        match item {
            Value::Boolean(false) => return Ok((Value::Boolean(false), context)),
            Value::Boolean(true) => {}
            _ => return Err(InterpreterError::InvalidOperation(
                "allTrue() requires a collection of boolean values".to_string(),
            )),
        }
    }
    Ok((Value::Boolean(true), context))
}

pub fn any_true(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    for item in items {
        match item {
            Value::Boolean(true) => return Ok((Value::Boolean(true), context)),
            Value::Boolean(false) => {}
            _ => return Err(InterpreterError::InvalidOperation(
                "anyTrue() requires a collection of boolean values".to_string(),
            )),
        }
    }
    Ok((Value::Boolean(false), context))
}

pub fn all_false(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    if items.is_empty() {
        return Ok((Value::Boolean(true), context));
    }
    for item in items {
        match item {
            Value::Boolean(true) => return Ok((Value::Boolean(false), context)),
            Value::Boolean(false) => {}
            _ => return Err(InterpreterError::InvalidOperation(
                "allFalse() requires a collection of boolean values".to_string(),
            )),
        }
    }
    Ok((Value::Boolean(true), context))
}

pub fn any_false(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    for item in items {
        match item {
            Value::Boolean(false) => return Ok((Value::Boolean(true), context)),
            Value::Boolean(true) => {}
            _ => return Err(InterpreterError::InvalidOperation(
                "anyFalse() requires a collection of boolean values".to_string(),
            )),
        }
    }
    Ok((Value::Boolean(false), context))
}

pub fn sum(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    if items.is_empty() {
        return Ok((Value::Number(0.0, 0), context));
    }

    let first_quantity = items
        .iter()
        .find(|item| matches!(item, Value::Quantity(..)));

    match first_quantity {
        Some(quantity) => {
            let mut acc = quantity.clone();
            let mut found_first = false;

            for item in &items {
                match item {
                    Value::Quantity(..) => {
                        if !found_first {
                            found_first = true;
                            continue;
                        }
                        match quantity_add(&acc, item) {
                            QuantityResult::Ok(q) => {
                                acc = q;
                            }
                            QuantityResult::Incompatible => {
                                return Ok((Value::collection(vec![]), context));
                            }
                        }
                    }
                    Value::Null => {}
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "sum() requires all values to be quantities when quantities are present"
                                .to_string(),
                        ));
                    }
                }
            }
            Ok((acc, context))
        }
        None => {
            let mut total = 0.0;
            for item in items {
                match item {
                    Value::Number(n, _) => {
                        total += n;
                    }
                    Value::Null => {}
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "sum() requires numeric values".to_string(),
                        ));
                    }
                }
            }
            Ok((Value::Number(total, Value::precision(total)), context))
        }
    }
}

pub fn avg(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    if items.is_empty() {
        return Ok((Value::Null, context));
    }

    let first_quantity = items
        .iter()
        .find(|item| matches!(item, Value::Quantity(..)));

    match first_quantity {
        Some(quantity) => {
            let mut acc = quantity.clone();
            let mut count = 1;
            let mut found_first = false;

            for item in &items {
                match item {
                    Value::Quantity(..) => {
                        if !found_first {
                            found_first = true;
                            continue;
                        }
                        match quantity_add(&acc, item) {
                            QuantityResult::Ok(q) => {
                                acc = q;
                                count += 1;
                            }
                            QuantityResult::Incompatible => {
                                return Ok((Value::collection(vec![]), context));
                            }
                        }
                    }
                    Value::Null => {}
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "avg() requires all values to be quantities when quantities are present"
                                .to_string(),
                        ));
                    }
                }
            }
            if let Value::Quantity(v, _, ref u, t) = acc {
                let result = v / count as f64;
                let p = Value::precision(result);
                Ok((Value::Quantity(result, p, u.clone(), t), context))
            } else {
                Ok((Value::Null, context))
            }
        }
        None => {
            let mut total = 0.0;
            let mut count = 0;
            for item in items {
                match item {
                    Value::Number(n, _) => {
                        total += n;
                        count += 1;
                    }
                    Value::Null => {}
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "avg() requires numeric values".to_string(),
                        ));
                    }
                }
            }
            if count == 0 {
                return Ok((Value::Null, context));
            }
            let result = total / count as f64;
            let p = Value::precision(result);
            Ok((Value::Number(result, p), context))
        }
    }
}

pub fn min(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    if items.is_empty() {
        return Ok((Value::Null, context));
    }

    let first_quantity = items
        .iter()
        .find(|item| matches!(item, Value::Quantity(..)));

    match first_quantity {
        Some(quantity) => {
            let mut min_q = quantity.clone();
            let mut found_first = false;

            for item in &items {
                match item {
                    Value::Quantity(..) => {
                        if !found_first {
                            found_first = true;
                            continue;
                        }
                        match quantity_cmp(item, &min_q) {
                            Some(std::cmp::Ordering::Less) => {
                                min_q = item.clone();
                            }
                            Some(_) => {}
                            None => {
                                return Ok((Value::collection(vec![]), context));
                            }
                        }
                    }
                    Value::Null => {}
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "min() requires all values to be quantities when quantities are present"
                                .to_string(),
                        ));
                    }
                }
            }
            Ok((min_q, context))
        }
        None => {
            let mut min_val: Option<(f64, u8)> = None;
            for item in items {
                let n = match &item {
                    Value::Number(n, p) => Some((*n, *p)),
                    Value::Null => None,
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "min() requires numeric values".to_string(),
                        ));
                    }
                };
                if let Some((n, p)) = n {
                    min_val = Some(min_val.map_or((n, p), |(m, mp)| {
                        if n < m { (n, p) } else { (m, mp) }
                    }));
                }
            }
            Ok((min_val.map_or(Value::Null, |(n, p)| Value::Number(n, p)), context))
        }
    }
}

pub fn max(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    if items.is_empty() {
        return Ok((Value::Null, context));
    }

    let first_quantity = items
        .iter()
        .find(|item| matches!(item, Value::Quantity(..)));

    match first_quantity {
        Some(quantity) => {
            let mut max_q = quantity.clone();
            let mut found_first = false;

            for item in &items {
                match item {
                    Value::Quantity(..) => {
                        if !found_first {
                            found_first = true;
                            continue;
                        }
                        match quantity_cmp(item, &max_q) {
                            Some(std::cmp::Ordering::Greater) => {
                                max_q = item.clone();
                            }
                            Some(_) => {}
                            None => {
                                return Ok((Value::collection(vec![]), context));
                            }
                        }
                    }
                    Value::Null => {}
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "max() requires all values to be quantities when quantities are present"
                                .to_string(),
                        ));
                    }
                }
            }
            Ok((max_q, context))
        }
        None => {
            let mut max_val: Option<(f64, u8)> = None;
            for item in items {
                let n = match &item {
                    Value::Number(n, p) => Some((*n, *p)),
                    Value::Null => None,
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "max() requires numeric values".to_string(),
                        ));
                    }
                };
                if let Some((n, p)) = n {
                    max_val = Some(max_val.map_or((n, p), |(m, mp)| {
                        if n > m { (n, p) } else { (m, mp) }
                    }));
                }
            }
            Ok((max_val.map_or(Value::Null, |(n, p)| Value::Number(n, p)), context))
        }
    }
}
