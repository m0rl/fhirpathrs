mod context;
pub mod datetime;
mod error;
mod invocations;
mod operators;
mod stack;
mod trace;
mod units;
mod value;

pub use crate::context::InterpreterContext;
pub use crate::error::InterpreterError;
pub use crate::trace::{CollectingTraceHandler, SharedTraceHandler, TraceEvent, TraceHandler};
pub use crate::value::{QuantityType, Value};
pub use datetime::DatePrecision;
pub use datetime::DateTimePrecision;
pub use datetime::TimePrecision;

use crate::invocations::{
    Continuation, dispatch_function, interpret_function, interpret_member_access,
};
use crate::operators::{
    interpret_additive, interpret_equality, interpret_indexer, interpret_inequality,
    interpret_membership, interpret_multiplicative, interpret_or, interpret_polarity,
    interpret_type, interpret_union,
};
use crate::stack::{BinOp, Frame};
use parser::{Expression, Invocation, Literal, Term};

pub type InterpreterResult = Result<(Value, InterpreterContext), InterpreterError>;

pub fn interpret(expression: &Expression, context: InterpreterContext) -> InterpreterResult {
    let mut stack: Vec<Frame> = Vec::new();
    let mut ctx = context;
    let mut current: &Expression = expression;

    'dispatch: loop {
        let mut expr = current;
        while let Expression::Term(Term::Parenthesized(inner)) = expr {
            expr = inner;
        }

        let mut val = match expr {
            Expression::Term(Term::Invocation(inv)) => {
                stack.push(Frame::Invocation(inv));
                ctx.data.clone()
            }
            Expression::Term(term) => {
                let (v, c) = interpret_term(term, ctx)?;
                ctx = c;
                v
            }
            Expression::Invocation(base, inv) => {
                stack.push(Frame::Invocation(inv));
                current = base;
                continue 'dispatch;
            }
            Expression::Indexer(base, index) => {
                stack.push(Frame::IndexerEvalIndex(index));
                current = base;
                continue 'dispatch;
            }
            Expression::Polarity(op, inner) => {
                stack.push(Frame::Polarity(op));
                current = inner;
                continue 'dispatch;
            }
            Expression::Multiplicative(left, op, right) => {
                stack.push(Frame::BinaryEvalRight(right, BinOp::Multiplicative(op)));
                current = left;
                continue 'dispatch;
            }
            Expression::Additive(left, op, right) => {
                stack.push(Frame::BinaryEvalRight(right, BinOp::Additive(op)));
                current = left;
                continue 'dispatch;
            }
            Expression::Type(inner, op, spec) => {
                stack.push(Frame::TypeApply(op, spec));
                current = inner;
                continue 'dispatch;
            }
            Expression::Union(left, right) => {
                stack.push(Frame::BinaryEvalRight(right, BinOp::Union));
                current = left;
                continue 'dispatch;
            }
            Expression::Inequality(left, op, right) => {
                stack.push(Frame::BinaryEvalRight(right, BinOp::Inequality(op)));
                current = left;
                continue 'dispatch;
            }
            Expression::Equality(left, op, right) => {
                stack.push(Frame::BinaryEvalRight(right, BinOp::Equality(op)));
                current = left;
                continue 'dispatch;
            }
            Expression::Membership(left, op, right) => {
                stack.push(Frame::BinaryEvalRight(right, BinOp::Membership(op)));
                current = left;
                continue 'dispatch;
            }
            Expression::And(left, right) => {
                stack.push(Frame::AndAfterLeft(right, ctx.clone()));
                current = left;
                continue 'dispatch;
            }
            Expression::Or(left, op, right) => {
                stack.push(Frame::OrAfterLeft(right, op, ctx.clone()));
                current = left;
                continue 'dispatch;
            }
            Expression::Implies(left, right) => {
                stack.push(Frame::ImpliesAfterLeft(right, ctx.clone()));
                current = left;
                continue 'dispatch;
            }
        };

        loop {
            match stack.pop() {
                None => return Ok((val, ctx)),
                Some(Frame::Invocation(inv)) => {
                    let base = val;
                    match inv {
                        Invocation::Member(name) => {
                            let (v, c) = interpret_member_access(&base, name, ctx)?;
                            val = v;
                            ctx = c;
                        }
                        Invocation::This => {
                            val = ctx.this_context.clone().unwrap_or_else(|| ctx.data.clone());
                        }
                        Invocation::Index => {
                            #[allow(clippy::cast_precision_loss)]
                            {
                                val = ctx
                                    .index_context
                                    .map_or(Value::Null, |i| Value::Number(i as f64, 0));
                            }
                        }
                        Invocation::Total => {
                            val = ctx.total_context.clone().unwrap_or(Value::Null);
                        }
                        Invocation::Function(name, args) => {
                            match dispatch_function(&base, name, args, ctx)? {
                                Continuation::Resolved(v, context) => {
                                    val = v;
                                    ctx = context;
                                }
                                Continuation::Chain(exp, frame, context) => {
                                    stack.push(frame);
                                    ctx = context;
                                    current = exp;
                                    continue 'dispatch;
                                }
                            }
                        }
                    }
                }
                Some(Frame::IndexerEvalIndex(index_expr)) => {
                    stack.push(Frame::IndexerCombine(val));
                    current = index_expr;
                    continue 'dispatch;
                }
                Some(Frame::IndexerCombine(base_val)) => {
                    let (v, c) = interpret_indexer(&base_val, &val, ctx)?;
                    val = v;
                    ctx = c;
                }
                Some(Frame::Polarity(op)) => {
                    let (v, c) = interpret_polarity(op, &val, ctx)?;
                    val = v;
                    ctx = c;
                }
                Some(Frame::BinaryEvalRight(right_expr, op)) => {
                    stack.push(Frame::BinaryCombine(val, op));
                    current = right_expr;
                    continue 'dispatch;
                }
                Some(Frame::BinaryCombine(left_val, op)) => {
                    let (v, c) = match op {
                        BinOp::Multiplicative(mop) => {
                            interpret_multiplicative(&left_val, mop, &val, ctx)?
                        }
                        BinOp::Additive(aop) => interpret_additive(&left_val, aop, &val, ctx)?,
                        BinOp::Union => interpret_union(&left_val, &val, ctx)?,
                        BinOp::Inequality(iop) => interpret_inequality(&left_val, iop, &val, ctx)?,
                        BinOp::Equality(eop) => interpret_equality(&left_val, eop, &val, ctx)?,
                        BinOp::Membership(mop) => interpret_membership(&left_val, mop, &val, ctx)?,
                    };
                    val = v;
                    ctx = c;
                }
                Some(Frame::TypeApply(type_op, spec)) => {
                    let (v, c) = interpret_type(&val, type_op, spec, ctx)?;
                    val = v;
                    ctx = c;
                }
                Some(Frame::AndAfterLeft(right_expr, saved_ctx)) => {
                    if val.to_bool() == Some(false) {
                        val = Value::Boolean(false);
                        ctx = saved_ctx;
                    } else {
                        let left_val = val;
                        stack.push(Frame::AndAfterRight(left_val, saved_ctx.clone()));
                        ctx = saved_ctx;
                        current = right_expr;
                        continue 'dispatch;
                    }
                }
                Some(Frame::AndAfterRight(left_val, saved_ctx)) => {
                    val = match (left_val.to_bool(), val.to_bool()) {
                        (_, Some(false)) => Value::Boolean(false),
                        (Some(true), Some(true)) => Value::Boolean(true),
                        _ => Value::collection(vec![]),
                    };
                    ctx = saved_ctx;
                }
                Some(Frame::OrAfterLeft(right_expr, op, saved_ctx)) => {
                    if matches!(op, parser::OrOp::Or) && val.to_bool() == Some(true) {
                        val = Value::Boolean(true);
                        ctx = saved_ctx;
                    } else {
                        let left_val = val;
                        stack.push(Frame::OrAfterRight(left_val, op, saved_ctx.clone()));
                        ctx = saved_ctx;
                        current = right_expr;
                        continue 'dispatch;
                    }
                }
                Some(Frame::OrAfterRight(left_val, op, saved_ctx)) => {
                    let (v, c) = interpret_or(&left_val, op, &val, saved_ctx)?;
                    val = v;
                    ctx = c;
                }
                Some(Frame::ImpliesAfterLeft(right_expr, saved_ctx)) => {
                    if val.to_bool() == Some(false) {
                        val = Value::Boolean(true);
                        ctx = saved_ctx;
                    } else {
                        let left_val = val;
                        stack.push(Frame::ImpliesAfterRight(left_val, saved_ctx.clone()));
                        ctx = saved_ctx;
                        current = right_expr;
                        continue 'dispatch;
                    }
                }
                Some(Frame::ImpliesAfterRight(left_val, saved_ctx)) => {
                    val = match (left_val.to_bool(), val.to_bool()) {
                        (Some(true), Some(r)) => Value::Boolean(r),
                        (None, Some(true)) => Value::Boolean(true),
                        _ => Value::collection(vec![]),
                    };
                    ctx = saved_ctx;
                }
                Some(Frame::WhereApply {
                    items,
                    expr: criteria,
                    index,
                    mut results,
                    saved_ctx,
                }) => {
                    if val.is_truthy() {
                        results.push(items[index].clone());
                    }
                    let next = index + 1;
                    if next < items.len() {
                        let item_ctx = saved_ctx
                            .clone()
                            .with_this(items[next].clone())
                            .with_index(next)
                            .with_total_count(items.len());
                        stack.push(Frame::WhereApply {
                            items,
                            expr: criteria,
                            index: next,
                            results,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = criteria;
                        continue 'dispatch;
                    }
                    val = Value::collection(results);
                    ctx = saved_ctx;
                }
                Some(Frame::SelectApply {
                    items,
                    expr: projection,
                    index,
                    mut results,
                    saved_ctx,
                }) => {
                    match val {
                        Value::Collection(ref inner) => results.extend(inner.iter().cloned()),
                        Value::Null => {}
                        other => results.push(other),
                    }
                    let next = index + 1;
                    if next < items.len() {
                        let item_ctx = saved_ctx
                            .clone()
                            .with_this(items[next].clone())
                            .with_index(next)
                            .with_total_count(items.len());
                        stack.push(Frame::SelectApply {
                            items,
                            expr: projection,
                            index: next,
                            results,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = projection;
                        continue 'dispatch;
                    }
                    val = Value::collection(results);
                    ctx = saved_ctx;
                }
                Some(Frame::AllApply {
                    items,
                    expr: criteria,
                    index,
                    saved_ctx,
                }) => {
                    if !val.is_truthy() {
                        val = Value::Boolean(false);
                        ctx = saved_ctx;
                    } else {
                        let next = index + 1;
                        if next < items.len() {
                            let item_ctx = saved_ctx
                                .clone()
                                .with_this(items[next].clone())
                                .with_index(next)
                                .with_total_count(items.len());
                            stack.push(Frame::AllApply {
                                items,
                                expr: criteria,
                                index: next,
                                saved_ctx,
                            });
                            ctx = item_ctx;
                            current = criteria;
                            continue 'dispatch;
                        }
                        val = Value::Boolean(true);
                        ctx = saved_ctx;
                    }
                }
                Some(Frame::ExistsApply {
                    items,
                    expr: criteria,
                    index,
                    saved_ctx,
                }) => {
                    if val.is_truthy() {
                        val = Value::Boolean(true);
                        ctx = saved_ctx;
                    } else {
                        let next = index + 1;
                        if next < items.len() {
                            let item_ctx = saved_ctx
                                .clone()
                                .with_this(items[next].clone())
                                .with_index(next)
                                .with_total_count(items.len());
                            stack.push(Frame::ExistsApply {
                                items,
                                expr: criteria,
                                index: next,
                                saved_ctx,
                            });
                            ctx = item_ctx;
                            current = criteria;
                            continue 'dispatch;
                        }
                        val = Value::Boolean(false);
                        ctx = saved_ctx;
                    }
                }
                Some(Frame::FuncArgs {
                    base,
                    name,
                    args,
                    index,
                    mut evaluated,
                    saved_ctx,
                }) => {
                    evaluated.push(val);
                    let next = index + 1;
                    if next < args.len() {
                        stack.push(Frame::FuncArgs {
                            base,
                            name,
                            args,
                            index: next,
                            evaluated,
                            saved_ctx: saved_ctx.clone(),
                        });
                        ctx = saved_ctx;
                        current = &args[next];
                        continue 'dispatch;
                    }
                    let (v, c) = interpret_function(&base, name, &evaluated, saved_ctx)?;
                    val = v;
                    ctx = c;
                }
                Some(Frame::RepeatApply {
                    dedup,
                    to_process,
                    index,
                    expr: projection,
                    mut result,
                    mut new_items,
                    saved_ctx,
                }) => {
                    let projected = val.to_vec();
                    for proj_item in projected {
                        if dedup {
                            if !result.iter().any(|existing| existing.equals(&proj_item)) {
                                result.push(proj_item.clone());
                                new_items.push(proj_item);
                            }
                        } else {
                            result.push(proj_item.clone());
                            new_items.push(proj_item);
                        }
                    }
                    let next = index + 1;
                    if next < to_process.len() {
                        let item_ctx = saved_ctx.clone().with_this(to_process[next].clone());
                        stack.push(Frame::RepeatApply {
                            dedup,
                            to_process,
                            index: next,
                            expr: projection,
                            result,
                            new_items,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = projection;
                        continue 'dispatch;
                    }
                    if new_items.is_empty() {
                        val = Value::collection(result);
                        ctx = saved_ctx;
                    } else {
                        let item_ctx = saved_ctx.clone().with_this(new_items[0].clone());
                        stack.push(Frame::RepeatApply {
                            dedup,
                            to_process: new_items,
                            index: 0,
                            expr: projection,
                            result,
                            new_items: Vec::new(),
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = projection;
                        continue 'dispatch;
                    }
                }
                Some(Frame::IifBranch {
                    args,
                    eval_ctx,
                    saved_ctx,
                }) => {
                    if !val.is_multi_item_collection() && val.is_truthy() {
                        stack.push(Frame::IifRestore { saved_ctx });
                        ctx = eval_ctx;
                        current = &args[1];
                        continue 'dispatch;
                    } else if !val.is_multi_item_collection() && args.len() >= 3 {
                        stack.push(Frame::IifRestore { saved_ctx });
                        ctx = eval_ctx;
                        current = &args[2];
                        continue 'dispatch;
                    }
                    val = Value::collection(vec![]);
                    ctx = saved_ctx;
                }
                Some(Frame::IifRestore { saved_ctx }) => {
                    ctx = saved_ctx;
                }
                Some(Frame::TraceAfterName {
                    base,
                    args,
                    saved_ctx,
                }) => {
                    let name = val.to_str().unwrap_or_else(|_| "trace".to_string());
                    if let Some(handler) = &saved_ctx.trace_handler {
                        let items = base.to_vec();
                        if args.len() >= 2 {
                            let total = items.len();
                            if items.is_empty() {
                                val = base;
                                ctx = saved_ctx;
                            } else {
                                let item_ctx = saved_ctx
                                    .clone()
                                    .with_this(items[0].clone())
                                    .with_index(0)
                                    .with_total_count(total);
                                stack.push(Frame::TraceLoop {
                                    base,
                                    name,
                                    items,
                                    index: 0,
                                    projection: &args[1],
                                    saved_ctx,
                                });
                                ctx = item_ctx;
                                current = &args[1];
                                continue 'dispatch;
                            }
                        } else {
                            for item in &items {
                                handler.on_trace(&trace::TraceEvent {
                                    name: name.clone(),
                                    value: item.clone(),
                                });
                            }
                            val = base;
                            ctx = saved_ctx;
                        }
                    } else {
                        val = base;
                        ctx = saved_ctx;
                    }
                }
                Some(Frame::TraceLoop {
                    base,
                    name,
                    items,
                    index,
                    projection,
                    saved_ctx,
                }) => {
                    if let Some(handler) = &saved_ctx.trace_handler {
                        handler.on_trace(&trace::TraceEvent {
                            name: name.clone(),
                            value: val,
                        });
                    }
                    let next = index + 1;
                    if next < items.len() {
                        let total = items.len();
                        let item_ctx = saved_ctx
                            .clone()
                            .with_this(items[next].clone())
                            .with_index(next)
                            .with_total_count(total);
                        stack.push(Frame::TraceLoop {
                            base,
                            name,
                            items,
                            index: next,
                            projection,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = projection;
                        continue 'dispatch;
                    }
                    val = base;
                    ctx = saved_ctx;
                }
                Some(Frame::AggregateInit {
                    items,
                    aggregator,
                    saved_ctx,
                }) => {
                    let accumulated = val;
                    if items.is_empty() {
                        val = accumulated;
                        ctx = saved_ctx;
                    } else {
                        let item_ctx = saved_ctx
                            .clone()
                            .with_this(items[0].clone())
                            .with_index(0)
                            .with_total(accumulated);
                        stack.push(Frame::AggregateLoop {
                            items,
                            aggregator,
                            index: 0,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = aggregator;
                        continue 'dispatch;
                    }
                }
                Some(Frame::AggregateLoop {
                    items,
                    aggregator,
                    index,
                    saved_ctx,
                }) => {
                    let accumulated = val;
                    let next = index + 1;
                    if next >= items.len() {
                        val = accumulated;
                        ctx = saved_ctx;
                    } else {
                        let item_ctx = saved_ctx
                            .clone()
                            .with_this(items[next].clone())
                            .with_index(next)
                            .with_total(accumulated);
                        stack.push(Frame::AggregateLoop {
                            items,
                            aggregator,
                            index: next,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = aggregator;
                        continue 'dispatch;
                    }
                }
                Some(Frame::SortEval {
                    items,
                    criteria,
                    index,
                    mut keyed,
                    descending,
                    saved_ctx,
                }) => {
                    keyed.push((items[index].clone(), val));
                    let next = index + 1;
                    if next < items.len() {
                        let item_ctx = saved_ctx.clone().with_this(items[next].clone());
                        stack.push(Frame::SortEval {
                            items,
                            criteria,
                            index: next,
                            keyed,
                            descending,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = criteria;
                        continue 'dispatch;
                    }
                    keyed.sort_by(|(_, a_key), (_, b_key)| {
                        let ord = a_key
                            .compare_equal(b_key)
                            .unwrap_or(std::cmp::Ordering::Equal);
                        if descending { ord.reverse() } else { ord }
                    });
                    val = Value::collection(keyed.into_iter().map(|(item, _)| item).collect());
                    ctx = saved_ctx;
                }
                Some(Frame::DefineVarEvalName {
                    base,
                    args,
                    saved_ctx,
                }) => {
                    let name = val.to_str().map_err(|_| {
                        InterpreterError::InvalidOperation(
                            "defineVariable() first argument must evaluate to a string".to_string(),
                        )
                    })?;
                    if args.len() == 2 {
                        let item_ctx = saved_ctx.clone().with_this(base.clone());
                        stack.push(Frame::DefineVarEval {
                            base,
                            name,
                            saved_ctx,
                        });
                        ctx = item_ctx;
                        current = &args[1];
                        continue 'dispatch;
                    }
                    ctx = saved_ctx.with_constant(name, base.clone());
                    val = base;
                }
                Some(Frame::DefineVarEval {
                    base,
                    name,
                    saved_ctx,
                }) => {
                    ctx = saved_ctx.with_constant(name, val);
                    val = base;
                }
                Some(Frame::CoalesceArgs {
                    args,
                    index,
                    saved_ctx,
                }) => {
                    if !val.is_null_or_empty() {
                        ctx = saved_ctx;
                    } else {
                        let next = index + 1;
                        if next < args.len() {
                            stack.push(Frame::CoalesceArgs {
                                args,
                                index: next,
                                saved_ctx: saved_ctx.clone(),
                            });
                            ctx = saved_ctx;
                            current = &args[next];
                            continue 'dispatch;
                        }
                        val = Value::collection(vec![]);
                        ctx = saved_ctx;
                    }
                }
            }
        }
    }
}

fn resolve_predefined_constant(name: &str, context: &InterpreterContext) -> Option<Value> {
    match name {
        "context" | "resource" | "rootResource" => Some(context.root_resource.clone()),
        "ucum" => Some(Value::String("http://unitsofmeasure.org".to_string())),
        "sct" => Some(Value::String("http://snomed.info/sct".to_string())),
        "loinc" => Some(Value::String("http://loinc.org".to_string())),
        _ => None,
    }
}

fn interpret_term(term: &Term, context: InterpreterContext) -> InterpreterResult {
    match term {
        Term::Literal(literal) => {
            let value = match literal {
                Literal::Null => Value::Null,
                Literal::Boolean(b) => Value::Boolean(*b),
                Literal::String(s) => Value::String(s.clone()),
                Literal::Number(n, p) => Value::Number(*n, *p),
                Literal::Date(d) => Value::from_date_str(d).ok_or_else(|| {
                    InterpreterError::InvalidOperation(format!("Invalid date: {}", d))
                })?,
                Literal::DateTime(dt) => Value::from_datetime_str(dt).ok_or_else(|| {
                    InterpreterError::InvalidOperation(format!("Invalid datetime: {}", dt))
                })?,
                Literal::Time(t) => Value::from_time_str(t).ok_or_else(|| {
                    InterpreterError::InvalidOperation(format!("Invalid time: {}", t))
                })?,
                Literal::Quantity(q) => Value::Quantity(q.value, q.precision, q.unit.clone(), None),
            };
            Ok((value, context))
        }
        Term::ExternalConstant(constant) => {
            let name = &constant.value;
            let value = context
                .external_constants
                .get(name)
                .cloned()
                .or_else(|| resolve_predefined_constant(name, &context))
                .or_else(|| {
                    if let Value::Object(ref map) = context.root_resource {
                        map.get(name).cloned()
                    } else {
                        None
                    }
                })
                .ok_or_else(|| InterpreterError::UnknownConstant(name.clone()))?;
            Ok((value, context))
        }
        Term::Invocation(_) | Term::Parenthesized(_) => Err(InterpreterError::InvalidOperation(
            "unexpected term type in interpret_term".to_string(),
        )),
    }
}
