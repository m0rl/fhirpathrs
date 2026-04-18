use crate::InterpreterResult;
use crate::context::{ContextConstant, InterpreterContext};
use crate::error::InterpreterError;
use crate::stack::Frame;
use crate::value::Value;
use parser::{Expression, Invocation, Literal, Term, TypeSpecifier};

mod aggregate;
mod collection;
mod math;
mod string;
mod type_conv;
mod utility;

pub(crate) fn interpret_member_access(
    base: &Value,
    member: &str,
    context: InterpreterContext,
) -> InterpreterResult {
    use std::collections::VecDeque;

    let value = match base {
        Value::Object(obj) => {
            if let Some(v) = utility::resolve_field(obj, member) {
                match v {
                    Value::Collection(ref inner) => Value::collection(inner.to_vec()),
                    other => other,
                }
            } else if member.starts_with(|c: char| c.is_ascii_uppercase())
                && let Some(Value::String(rt)) = obj.get("resourceType")
                && rt == member
            {
                return Ok((base.clone(), context));
            } else {
                Value::collection(vec![])
            }
        }
        Value::Collection(items) => {
            let mut results = Vec::new();
            let mut queue: VecDeque<&Value> = items.iter().collect();
            while let Some(item) = queue.pop_front() {
                match item {
                    Value::Object(obj) => {
                        if let Some(v) = utility::resolve_field(obj, member) {
                            match v {
                                Value::Collection(ref inner) => {
                                    results.extend(inner.iter().cloned());
                                }
                                other => results.push(other),
                            }
                        } else if member.starts_with(|c: char| c.is_ascii_uppercase())
                            && let Some(Value::String(rt)) = obj.get("resourceType")
                            && rt == member
                        {
                            results.push(item.clone());
                        }
                    }
                    Value::Collection(inner) => {
                        for v in inner.iter() {
                            queue.push_back(v);
                        }
                    }
                    _ => {}
                }
            }
            Value::collection(results)
        }
        Value::Quantity(n, p, ..) if member == "value" => Value::Number(*n, *p),
        Value::Quantity(_, _, code, _) if member == "code" => Value::String(code.clone()),
        _ => Value::Null,
    };
    Ok((value, context))
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum Continuation<'a> {
    Resolved(Value, InterpreterContext),
    Chain(&'a Expression, Frame<'a>, InterpreterContext),
}

pub(crate) fn dispatch_function<'a>(
    base: &Value,
    name: &'a str,
    args: &'a [Expression],
    ctx: InterpreterContext,
) -> Result<Continuation<'a>, InterpreterError> {
    match name {
        "where" => {
            if args.len() != 1 {
                return Err(InterpreterError::InvalidOperation(
                    "where() requires exactly one argument".to_string(),
                ));
            }
            let items = base.to_vec();
            if items.is_empty() {
                return Ok(Continuation::Resolved(Value::collection(vec![]), ctx));
            }
            let total = items.len();
            let item_ctx = ctx
                .clone()
                .with_this(items[0].clone())
                .with_index(0)
                .with_total_count(total);
            Ok(Continuation::Chain(
                &args[0],
                Frame::WhereApply {
                    items,
                    expr: &args[0],
                    index: 0,
                    results: Vec::new(),
                    saved_ctx: ctx.clone(),
                },
                item_ctx,
            ))
        }
        "select" => {
            if args.len() != 1 {
                return Err(InterpreterError::InvalidOperation(
                    "select() requires exactly one argument".to_string(),
                ));
            }
            let items = base.to_vec();
            if items.is_empty() {
                return Ok(Continuation::Resolved(Value::collection(vec![]), ctx));
            }
            let total = items.len();
            let item_ctx = ctx
                .clone()
                .with_this(items[0].clone())
                .with_index(0)
                .with_total_count(total);
            Ok(Continuation::Chain(
                &args[0],
                Frame::SelectApply {
                    items,
                    expr: &args[0],
                    index: 0,
                    results: Vec::new(),
                    saved_ctx: ctx.clone(),
                },
                item_ctx,
            ))
        }
        "all" => {
            if args.len() != 1 {
                return Err(InterpreterError::InvalidOperation(
                    "all() requires exactly one argument".to_string(),
                ));
            }
            let items = base.to_vec();
            if items.is_empty() {
                return Ok(Continuation::Resolved(Value::Boolean(true), ctx));
            }
            let total = items.len();
            let item_ctx = ctx
                .clone()
                .with_this(items[0].clone())
                .with_index(0)
                .with_total_count(total);
            Ok(Continuation::Chain(
                &args[0],
                Frame::AllApply {
                    items,
                    expr: &args[0],
                    index: 0,
                    saved_ctx: ctx.clone(),
                },
                item_ctx,
            ))
        }
        "exists" if !args.is_empty() => {
            if args.len() != 1 {
                return Err(InterpreterError::InvalidOperation(
                    "exists() with criteria requires exactly one argument".to_string(),
                ));
            }
            let items = base.to_vec();
            if items.is_empty() {
                return Ok(Continuation::Resolved(Value::Boolean(false), ctx));
            }
            let total = items.len();
            let item_ctx = ctx
                .clone()
                .with_this(items[0].clone())
                .with_index(0)
                .with_total_count(total);
            Ok(Continuation::Chain(
                &args[0],
                Frame::ExistsApply {
                    items,
                    expr: &args[0],
                    index: 0,
                    saved_ctx: ctx.clone(),
                },
                item_ctx,
            ))
        }
        "repeat" | "repeatAll" => {
            if args.len() != 1 {
                return Err(InterpreterError::InvalidOperation(format!(
                    "{}() requires exactly one argument",
                    name
                )));
            }
            let dedup = name == "repeat";
            let to_process = base.to_vec();
            if to_process.is_empty() {
                return Ok(Continuation::Resolved(Value::collection(vec![]), ctx));
            }
            let item_ctx = ctx.clone().with_this(to_process[0].clone());
            Ok(Continuation::Chain(
                &args[0],
                Frame::RepeatApply {
                    dedup,
                    to_process,
                    index: 0,
                    expr: &args[0],
                    result: Vec::new(),
                    new_items: Vec::new(),
                    saved_ctx: ctx.clone(),
                },
                item_ctx,
            ))
        }
        "iif" => {
            if args.len() < 2 || args.len() > 3 {
                return Err(InterpreterError::InvalidOperation(
                    "iif() requires 2 or 3 arguments: condition, trueResult[, falseResult]"
                        .to_string(),
                ));
            }
            let eval_ctx = if ctx.this_context.is_some() {
                ctx.clone()
            } else {
                ctx.clone().with_this(base.clone())
            };
            Ok(Continuation::Chain(
                &args[0],
                Frame::IifBranch {
                    args,
                    eval_ctx: eval_ctx.clone(),
                    saved_ctx: ctx.clone(),
                },
                eval_ctx,
            ))
        }
        "trace" => {
            if args.is_empty() || args.len() > 2 {
                return Err(InterpreterError::InvalidOperation(
                    "trace() requires 1 or 2 arguments: name[, projection]".to_string(),
                ));
            }
            let name_ctx = ctx.clone().with_this(base.clone());
            Ok(Continuation::Chain(
                &args[0],
                Frame::TraceAfterName {
                    base: base.clone(),
                    args,
                    saved_ctx: ctx.clone(),
                },
                name_ctx,
            ))
        }
        "aggregate" => {
            if args.is_empty() || args.len() > 2 {
                return Err(InterpreterError::InvalidOperation(
                    "aggregate() requires 1 or 2 arguments: aggregator expression and optional init value"
                        .to_string(),
                ));
            }
            let items = match base {
                Value::Collection(items) => Vec::clone(items),
                Value::Null => {
                    if args.len() > 1 {
                        return Ok(Continuation::Chain(
                            &args[1],
                            Frame::AggregateInit {
                                items: vec![],
                                aggregator: &args[0],
                                saved_ctx: ctx.clone(),
                            },
                            ctx.clone(),
                        ));
                    }
                    return Ok(Continuation::Resolved(Value::collection(vec![]), ctx));
                }
                other => vec![other.clone()],
            };
            if args.len() > 1 {
                Ok(Continuation::Chain(
                    &args[1],
                    Frame::AggregateInit {
                        items,
                        aggregator: &args[0],
                        saved_ctx: ctx.clone(),
                    },
                    ctx.clone(),
                ))
            } else if items.is_empty() {
                Ok(Continuation::Resolved(Value::collection(vec![]), ctx))
            } else {
                let accumulated = Value::collection(vec![]);
                let item_ctx = ctx
                    .clone()
                    .with_this(items[0].clone())
                    .with_index(0)
                    .with_total(accumulated);
                Ok(Continuation::Chain(
                    &args[0],
                    Frame::AggregateLoop {
                        items,
                        aggregator: &args[0],
                        index: 0,
                        saved_ctx: ctx.clone(),
                    },
                    item_ctx,
                ))
            }
        }
        "defineVariable" => {
            if args.is_empty() || args.len() > 2 {
                return Err(InterpreterError::InvalidOperation(
                    "defineVariable() requires 1 or 2 arguments: name and optional expression"
                        .to_string(),
                ));
            }
            if let Expression::Term(Term::Literal(Literal::String(var_name))) = &args[0] {
                if matches!(
                    ctx.constants.get(var_name),
                    Some(ContextConstant::System(_) | ContextConstant::Runtime(_))
                ) {
                    return Ok(Continuation::Resolved(
                        Value::collection(vec![]),
                        ctx.clone(),
                    ));
                }
                if args.len() == 2 {
                    let item_ctx = ctx.clone().with_this(base.clone());
                    Ok(Continuation::Chain(
                        &args[1],
                        Frame::DefineVarEval {
                            base: base.clone(),
                            name: var_name.clone(),
                            saved_ctx: ctx.clone(),
                        },
                        item_ctx,
                    ))
                } else {
                    Ok(Continuation::Resolved(
                        base.clone(),
                        ctx.clone().define_variable(var_name.clone(), base.clone()),
                    ))
                }
            } else {
                let eval_ctx = ctx.clone().with_this(base.clone());
                Ok(Continuation::Chain(
                    &args[0],
                    Frame::DefineVarEvalName {
                        base: base.clone(),
                        args,
                        saved_ctx: ctx.clone(),
                    },
                    eval_ctx,
                ))
            }
        }
        "ofType" => {
            let (v, c) = interpret_of_type(base, args, ctx.clone())?;
            Ok(Continuation::Resolved(v, c))
        }
        "is" => {
            if args.len() != 1 {
                return Err(InterpreterError::InvalidOperation(
                    "is() requires exactly one argument: type name".to_string(),
                ));
            }
            let type_spec = TypeSpecifier::QualifiedIdentifier(collect_identifier_parts(&args[0])?);
            Ok(Continuation::Resolved(
                Value::Boolean(base.is(&type_spec)),
                ctx,
            ))
        }
        "as" => {
            if args.len() != 1 {
                return Err(InterpreterError::InvalidOperation(
                    "as() requires exactly one argument: type name".to_string(),
                ));
            }
            let type_spec = TypeSpecifier::QualifiedIdentifier(collect_identifier_parts(&args[0])?);
            Ok(Continuation::Resolved(base.as_type(&type_spec), ctx))
        }
        "sort" => {
            let (criteria_args, descending) = match args.last().and_then(|e| {
                if let Expression::Term(Term::Invocation(Invocation::Member(n))) = e {
                    match n.as_str() {
                        "asc" => Some(false),
                        "desc" => Some(true),
                        _ => None,
                    }
                } else {
                    None
                }
            }) {
                Some(desc) => (&args[..args.len() - 1], desc),
                None => (args, false),
            };
            if criteria_args.len() > 1 {
                return Err(InterpreterError::InvalidOperation(
                    "sort() requires zero or one criteria expression".to_string(),
                ));
            }
            let items = base.to_vec();
            if items.is_empty() {
                return Ok(Continuation::Resolved(Value::collection(vec![]), ctx));
            }
            if criteria_args.is_empty() {
                let mut sorted = items;
                sorted.sort_by(|a, b| {
                    let ord = a.compare_equal(b).unwrap_or(std::cmp::Ordering::Equal);
                    if descending { ord.reverse() } else { ord }
                });
                return Ok(Continuation::Resolved(Value::collection(sorted), ctx));
            }
            let criteria = &criteria_args[0];
            let item_ctx = ctx.clone().with_this(items[0].clone());
            Ok(Continuation::Chain(
                criteria,
                Frame::SortEval {
                    items,
                    criteria,
                    index: 0,
                    keyed: Vec::new(),
                    descending,
                    saved_ctx: ctx.clone(),
                },
                item_ctx,
            ))
        }
        "coalesce" => {
            let items = base.to_vec();
            for item in items {
                if !item.is_null_or_empty() {
                    return Ok(Continuation::Resolved(item, ctx));
                }
            }
            if args.is_empty() {
                return Ok(Continuation::Resolved(Value::collection(vec![]), ctx));
            }
            Ok(Continuation::Chain(
                &args[0],
                Frame::CoalesceArgs {
                    args,
                    index: 0,
                    saved_ctx: ctx.clone(),
                },
                ctx,
            ))
        }
        _ => {
            if args.is_empty() {
                let (v, c) = interpret_function(base, name, &[], ctx.clone())?;
                return Ok(Continuation::Resolved(v, c));
            }
            Ok(Continuation::Chain(
                &args[0],
                Frame::FuncArgs {
                    base: base.clone(),
                    name,
                    args,
                    index: 0,
                    evaluated: Vec::new(),
                    saved_ctx: ctx.clone(),
                },
                ctx,
            ))
        }
    }
}

pub(crate) fn interpret_function(
    base: &Value,
    name: &str,
    args: &[Value],
    context: InterpreterContext,
) -> InterpreterResult {
    if propagates_empty(name) {
        if base.is_null_or_empty() {
            return Ok((Value::collection(vec![]), context));
        }
        if args.iter().any(Value::is_null_or_empty) {
            return Ok((Value::collection(vec![]), context));
        }
    }
    match name {
        "empty" => collection::empty(base, context),
        "exists" => collection::exists(base, context),
        "count" => collection::count(base, context),
        "first" => collection::first(base, context),
        "last" => collection::last(base, context),
        "combine" => collection::combine(base, args, context),
        "single" => collection::single(base, context),
        "tail" => collection::tail(base, context),
        "take" => collection::take(base, args, context),
        "skip" => collection::skip(base, args, context),
        "distinct" => collection::distinct(base, context),
        "isDistinct" => collection::is_distinct(base, context),
        "intersect" => collection::intersect(base, args, context),
        "exclude" => collection::exclude(base, args, context),
        "subsetOf" => collection::subset_of(base, args, context),
        "supersetOf" => collection::superset_of(base, args, context),
        "not" => collection::not(base, context),
        "hasValue" => collection::has_value(base, context),
        "union" => collection::union(base, args, context),
        "children" => collection::children(base, context),
        "descendants" => collection::descendants(base, context),

        "allTrue" => aggregate::all_true(base, context),
        "anyTrue" => aggregate::any_true(base, context),
        "allFalse" => aggregate::all_false(base, context),
        "anyFalse" => aggregate::any_false(base, context),
        "sum" => aggregate::sum(base, context),
        "avg" => aggregate::avg(base, context),
        "min" => aggregate::min(base, context),
        "max" => aggregate::max(base, context),

        "abs" => math::abs(base, context),
        "ceiling" => math::ceiling(base, context),
        "floor" => math::floor(base, context),
        "round" => math::round(base, args, context),
        "truncate" => math::truncate(base, context),
        "sqrt" => math::sqrt(base, context),
        "exp" => math::exp(base, context),
        "ln" => math::ln(base, context),
        "log" => math::log(base, args, context),
        "power" => math::power(base, args, context),

        "indexOf" => string::index_of(base, args, context),
        "substring" => string::substring(base, args, context),
        "startsWith" => string::starts_with(base, args, context),
        "endsWith" => string::ends_with(base, args, context),
        "contains" => string::contains(base, args, context),
        "upper" => string::upper(base, context),
        "lower" => string::lower(base, context),
        "replace" => string::replace(base, args, context),
        "matches" => string::matches(base, args, context),
        "replaceMatches" => string::replace_matches(base, args, context),
        "length" => string::length(base, context),
        "toChars" => string::to_chars(base, context),
        "split" => string::split(base, args, context),
        "join" => string::join(base, args, context),
        "trim" => string::trim(base, context),
        "lastIndexOf" => string::last_index_of(base, args, context),
        "matchesFull" => string::matches_full(base, args, context),
        "encode" => string::encode(base, args, context),
        "decode" => string::decode(base, args, context),
        "escape" => string::escape(base, args, context),
        "unescape" => string::unescape(base, args, context),

        "toString" => type_conv::to_string(base, context),
        "toInteger" => type_conv::to_integer(base, context),
        "toDecimal" => type_conv::to_decimal(base, context),
        "toBoolean" => type_conv::to_boolean(base, context),
        "toDate" => type_conv::to_date(base, context),
        "toDateTime" => type_conv::to_date_time(base, context),
        "toTime" => type_conv::to_time(base, context),
        "toQuantity" => type_conv::to_quantity(base, args, context),
        "convertsToInteger" => type_conv::converts_to_integer(base, context),
        "convertsToDecimal" => type_conv::converts_to_decimal(base, context),
        "convertsToBoolean" => type_conv::converts_to_boolean(base, context),
        "convertsToString" => type_conv::converts_to_string(base, context),
        "convertsToDate" => type_conv::converts_to_date(base, context),
        "convertsToDateTime" => type_conv::converts_to_date_time(base, context),
        "convertsToTime" => type_conv::converts_to_time(base, context),
        "convertsToQuantity" => type_conv::converts_to_quantity(base, context),
        "toLong" => type_conv::to_long(base, context),
        "convertsToLong" => type_conv::converts_to_long(base, context),

        "now" => utility::now(base, context),
        "today" => utility::today(base, context),
        "timeOfDay" => utility::time_of_day(base, context),
        "type" => utility::value_type(base, context),
        "precision" => utility::precision(base, context),
        "lowBoundary" => utility::low_boundary(base, args, context),
        "highBoundary" => utility::high_boundary(base, args, context),
        "year" => utility::year(base, context),
        "month" => utility::month(base, context),
        "day" => utility::day(base, context),
        "hour" => utility::hour(base, context),
        "minute" => utility::minute(base, context),
        "second" => utility::second(base, context),
        "millisecond" => utility::millisecond(base, context),
        "timezone" => utility::timezone(base, context),
        "duration" => utility::duration(base, args, context),
        "difference" => utility::difference(base, args, context),
        "comparable" => utility::comparable(base, args, context),

        _ => Err(InterpreterError::UnknownFunction(name.to_string())),
    }
}

pub(crate) fn interpret_of_type(
    base: &Value,
    args: &[Expression],
    context: InterpreterContext,
) -> InterpreterResult {
    if args.len() != 1 {
        return Err(InterpreterError::InvalidOperation(
            "ofType() requires exactly one argument: type name".to_string(),
        ));
    }

    let type_spec = TypeSpecifier::QualifiedIdentifier(collect_identifier_parts(&args[0])?);

    let items = base.to_vec();
    let result: Vec<Value> = items
        .into_iter()
        .filter(|item| item.is(&type_spec))
        .collect();

    Ok((Value::collection(result), context))
}

pub fn collect_identifier_parts(expr: &Expression) -> Result<Vec<String>, InterpreterError> {
    let mut current = expr;
    let mut collected = Vec::new();
    loop {
        match current {
            Expression::Term(Term::Invocation(Invocation::Member(name))) => {
                collected.push(name.clone());
                collected.reverse();
                return Ok(collected);
            }
            Expression::Invocation(base_expr, Invocation::Member(name)) => {
                collected.push(name.clone());
                current = base_expr;
            }
            _ => {
                return Err(InterpreterError::InvalidOperation(
                    "ofType() argument must be a type name".to_string(),
                ));
            }
        }
    }
}

pub fn propagates_empty(func_name: &str) -> bool {
    matches!(
        func_name,
        "indexOf"
            | "substring"
            | "startsWith"
            | "endsWith"
            | "contains"
            | "upper"
            | "lower"
            | "replace"
            | "matches"
            | "replaceMatches"
            | "length"
            | "toChars"
            | "split"
            | "trim"
            | "lastIndexOf"
            | "matchesFull"
            | "encode"
            | "decode"
            | "escape"
            | "unescape"
            | "abs"
            | "ceiling"
            | "floor"
            | "round"
            | "truncate"
            | "sqrt"
            | "exp"
            | "ln"
            | "log"
            | "power"
            | "toString"
            | "toInteger"
            | "toDecimal"
            | "toBoolean"
            | "toDate"
            | "toDateTime"
            | "toTime"
            | "toQuantity"
            | "convertsToInteger"
            | "convertsToDecimal"
            | "convertsToBoolean"
            | "convertsToString"
            | "convertsToDate"
            | "convertsToDateTime"
            | "convertsToTime"
            | "convertsToQuantity"
            | "toLong"
            | "convertsToLong"
    )
}
