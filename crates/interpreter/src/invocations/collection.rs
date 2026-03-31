use crate::InterpreterResult;
use crate::context::InterpreterContext;
use crate::error::InterpreterError;
use crate::value::Value;

pub fn empty(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = Value::Boolean(match base {
        Value::Null => true,
        Value::Collection(v) => v.is_empty(),
        _ => false,
    });
    Ok((value, context))
}

pub fn exists(base: &Value, context: InterpreterContext) -> InterpreterResult {
    Ok((Value::Boolean(!base.is_null_or_empty()), context))
}

#[allow(clippy::cast_precision_loss)]
pub fn count(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = Value::Number(match base {
        Value::Collection(v) => v.len() as f64,
        Value::Null => 0.0,
        _ => 1.0,
    }, 0);
    Ok((value, context))
}

pub fn first(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Collection(v) => v.first().cloned().unwrap_or(Value::Null),
        _ => base.clone(),
    };
    Ok((value, context))
}

pub fn last(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let value = match base {
        Value::Collection(v) => v.last().cloned().unwrap_or(Value::Null),
        _ => base.clone(),
    };
    Ok((value, context))
}

pub fn combine(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Ok((base.clone(), context));
    }
    let mut result = vec![];
    if let Value::Collection(items) = base {
        result.extend(items.iter().cloned());
    } else if !matches!(base, Value::Null) {
        result.push(base.clone());
    }

    for arg in args {
        if let Value::Collection(items) = arg {
            result.extend(items.iter().cloned());
        } else if !matches!(arg, Value::Null) {
            result.push(arg.clone());
        }
    }
    Ok((Value::collection(result), context))
}

pub fn single(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    if items.len() == 1 {
        Ok((items.into_iter().next().unwrap_or(Value::Null), context))
    } else if items.is_empty() {
        Ok((Value::Null, context))
    } else {
        Err(InterpreterError::InvalidOperation(
            "single() requires collection to have exactly one item".to_string(),
        ))
    }
}

pub fn tail(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    let value = if items.len() <= 1 {
        Value::collection(vec![])
    } else {
        Value::collection(items.into_iter().skip(1).collect())
    };
    Ok((value, context))
}

pub fn take(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "take() requires a count argument".to_string(),
        ));
    }
    let count = args[0]
        .to_usize()
        .ok_or_else(|| InterpreterError::TypeMismatch("take count must be a number".to_string()))?;
    let items = base.to_vec();
    Ok((
        Value::collection(items.into_iter().take(count).collect()),
        context,
    ))
}

pub fn skip(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "skip() requires a count argument".to_string(),
        ));
    }
    let count = args[0]
        .to_usize()
        .ok_or_else(|| InterpreterError::TypeMismatch("skip count must be a number".to_string()))?;
    let items = base.to_vec();
    Ok((
        Value::collection(items.into_iter().skip(count).collect()),
        context,
    ))
}

pub fn distinct(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    let mut result: Vec<Value> = Vec::new();
    for item in items.iter() {
        if !result.iter().any(|existing| existing.equals(item)) {
            result.push(item.clone());
        }
    }
    Ok((Value::collection(result), context))
}

pub fn is_distinct(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let items = base.to_vec();
    let mut seen: Vec<Value> = Vec::new();
    for item in items.iter() {
        if seen.iter().any(|existing| existing.equals(item)) {
            return Ok((Value::Boolean(false), context));
        }
        seen.push(item.clone());
    }
    Ok((Value::Boolean(true), context))
}

pub fn intersect(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "intersect() requires a collection argument".to_string(),
        ));
    }
    let left_items = base.to_vec();
    let right_items = args[0].to_vec();
    let mut result: Vec<Value> = Vec::new();
    for item in left_items {
        if right_items.iter().any(|r| r.equals(&item))
            && !result.iter().any(|existing| existing.equals(&item))
        {
            result.push(item);
        }
    }
    Ok((Value::collection(result), context))
}

pub fn exclude(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "exclude() requires a collection argument".to_string(),
        ));
    }
    let left_items = base.to_vec();
    let right_items = args[0].to_vec();
    let result: Vec<Value> = left_items
        .into_iter()
        .filter(|item| !right_items.iter().any(|r| r.equals(item)))
        .collect();
    Ok((Value::collection(result), context))
}

pub fn subset_of(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "subsetOf() requires a collection argument".to_string(),
        ));
    }
    let left_items = base.to_vec();
    let right_items = args[0].to_vec();
    let result = left_items
        .iter()
        .all(|item| right_items.iter().any(|r| r.equals(item)));
    Ok((Value::Boolean(result), context))
}

pub fn superset_of(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "supersetOf() requires a collection argument".to_string(),
        ));
    }
    let left_items = base.to_vec();
    let right_items = args[0].to_vec();
    let result = right_items
        .iter()
        .all(|item| left_items.iter().any(|l| l.equals(item)));
    Ok((Value::Boolean(result), context))
}

pub fn not(base: &Value, context: InterpreterContext) -> InterpreterResult {
    match base.to_vec().as_slice() {
        [] => Ok((Value::collection(vec![]), context)),
        [Value::Boolean(b)] => Ok((Value::Boolean(!*b), context)),
        [_] => Ok((Value::Boolean(false), context)),
        _ => Err(InterpreterError::InvalidOperation(
            "not() requires a collection with at most one item".to_string(),
        )),
    }
}

pub fn has_value(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let result = match base {
        Value::Null => false,
        Value::Collection(items) => items.len() == 1 && !matches!(items.first(), Some(Value::Null)),
        _ => true,
    };
    Ok((Value::Boolean(result), context))
}

pub fn union(base: &Value, args: &[Value], context: InterpreterContext) -> InterpreterResult {
    if args.is_empty() {
        return Err(InterpreterError::InvalidOperation(
            "union() requires a collection argument".to_string(),
        ));
    }
    let mut result: Vec<Value> = Vec::new();
    for item in base.to_vec() {
        if !result.iter().any(|existing| existing.equals(&item)) {
            result.push(item);
        }
    }
    for item in args[0].to_vec() {
        if !result.iter().any(|existing| existing.equals(&item)) {
            result.push(item);
        }
    }
    Ok((Value::collection(result), context))
}

fn collect_children(value: &Value) -> Vec<Value> {
    use std::collections::VecDeque;

    let mut results = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(value);

    while let Some(v) = queue.pop_front() {
        match v {
            Value::Object(obj) => {
                let mut keys: Vec<&String> = obj.keys().collect();
                keys.sort();
                for k in keys {
                    if let Some(child) = obj.get(k) {
                        match child {
                            Value::Collection(items) => {
                                results.extend(items.iter().cloned());
                            }
                            _ => results.push(child.clone()),
                        }
                    }
                }
            }
            Value::Collection(items) => {
                for item in items.iter() {
                    queue.push_back(item);
                }
            }
            _ => {}
        }
    }

    results
}

pub fn children(base: &Value, context: InterpreterContext) -> InterpreterResult {
    Ok((Value::collection(collect_children(base)), context))
}

pub fn descendants(base: &Value, context: InterpreterContext) -> InterpreterResult {
    let mut result: Vec<Value> = Vec::new();
    let mut to_process = collect_children(base);

    while !to_process.is_empty() {
        let mut next_level: Vec<Value> = Vec::new();
        for item in to_process {
            next_level.extend(collect_children(&item));
            result.push(item);
        }
        to_process = next_level;
    }

    Ok((Value::collection(result), context))
}
