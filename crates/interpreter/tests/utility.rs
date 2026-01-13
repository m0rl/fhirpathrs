#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use chrono::{Datelike, Timelike};
use interpreter::{CollectingTraceHandler, InterpreterContext, Value, interpret};
use parser::parse;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn test_iif_true_branch() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("iif(true, 'yes', 'no')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("yes".to_string()));
}

#[test]
fn test_iif_false_branch() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("iif(false, 'yes', 'no')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("no".to_string()));
}

#[test]
fn test_iif_without_else() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("iif(false, 'yes')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_iif_with_expression_condition() {
    let context = InterpreterContext::new(Value::Number(5.0));
    let expr = parse("iif($this > 3, 'first', 'second')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("first".to_string()));
}

#[test]
fn test_iif_lazy_evaluation_true() {
    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("iif(true, 'ok', single())").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("ok".to_string()));
}

#[test]
fn test_iif_lazy_evaluation_false() {
    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("iif(false, single(), 'ok')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("ok".to_string()));
}

#[test]
fn test_iif_nested() {
    let context = InterpreterContext::new(Value::Number(15.0));
    let expr = parse("iif($this < 10, 'first', iif($this < 20, 'second', 'third'))")
        .expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("second".to_string()));
}

#[test]
fn test_iif_with_collection_this_refers_to_whole_collection() {
    let data = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("iif($this.count() > 2, 'many', 'few')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("many".to_string()));
}

#[test]
fn test_iif_this_is_collection_can_use_first() {
    let data = Value::collection(vec![
        Value::String("alpha".to_string()),
        Value::String("beta".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr =
        parse("iif($this.first() = 'alpha', 'starts with alpha', 'other')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("starts with alpha".to_string()));
}

#[test]
fn test_iif_vs_select_iif_for_itemwise() {
    let data = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(5.0),
        Value::Number(10.0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("select(iif($this > 3, 'many', 'few'))").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(
        result,
        Value::collection(vec![
            Value::String("few".to_string()),
            Value::String("many".to_string()),
            Value::String("many".to_string()),
        ])
    );
}

#[test]
fn test_trace_returns_input_unchanged() {
    let data = Value::Number(42.0);
    let context = InterpreterContext::new(data.clone());
    let expr = parse("trace('test')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(result, data);
}

#[test]
fn test_trace_returns_collection_unchanged() {
    let data = Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]);
    let context = InterpreterContext::new(data.clone());
    let expr = parse("trace('nums')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(result, data);
}

#[test]
fn test_trace_with_projection_returns_input_unchanged() {
    let data = Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]);
    let context = InterpreterContext::new(data.clone());
    let expr = parse("trace('doubled', $this * 2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(result, data);
}

#[test]
fn test_trace_calls_handler() {
    let data = Value::Number(42.0);
    let handler = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext::new(data.clone()).with_trace_handler(handler.clone());

    let expr = parse("trace('myTrace')").expect("parse failed");
    let _ = interpret(&expr, context.clone()).expect("interpret failed");

    let events = handler.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "myTrace");
    assert_eq!(events[0].value, Value::Number(42.0));
}

#[test]
fn test_trace_handler_receives_collection_items() {
    let data = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let handler = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext::new(data).with_trace_handler(handler.clone());

    let expr = parse("trace('nums')").expect("parse failed");
    let _ = interpret(&expr, context.clone()).expect("interpret failed");

    let events = handler.events();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].value, Value::Number(1.0));
    assert_eq!(events[1].value, Value::Number(2.0));
    assert_eq!(events[2].value, Value::Number(3.0));
}

#[test]
fn test_trace_with_projection_traces_projected_values() {
    let data = Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]);
    let handler = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext::new(data).with_trace_handler(handler.clone());

    let expr = parse("trace('doubled', $this * 2)").expect("parse failed");
    let _ = interpret(&expr, context.clone()).expect("interpret failed");

    let events = handler.events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].name, "doubled");
    assert_eq!(events[0].value, Value::Number(2.0));
    assert_eq!(events[1].value, Value::Number(4.0));
}

#[test]
fn test_trace_with_total_in_projection() {
    let data = Value::collection(vec![
        Value::Number(10.0),
        Value::Number(20.0),
        Value::Number(30.0),
    ]);
    let handler = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext::new(data).with_trace_handler(handler.clone());

    let expr = parse("trace('progress', $index + 1)").expect("parse failed");
    let _ = interpret(&expr, context.clone()).expect("interpret failed");

    let events = handler.events();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].value, Value::Number(1.0));
    assert_eq!(events[1].value, Value::Number(2.0));
    assert_eq!(events[2].value, Value::Number(3.0));

    handler.clear();

    let expr = parse("trace('of-total', $total)").expect("parse failed");
    let _ = interpret(&expr, context.clone()).expect("interpret failed");

    let events = handler.events();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].value, Value::Number(3.0));
    assert_eq!(events[1].value, Value::Number(3.0));
    assert_eq!(events[2].value, Value::Number(3.0));
}

#[test]
fn test_trace_chainable() {
    let data = Value::Number(5.0);
    let context = InterpreterContext::new(data);

    let expr = parse("trace('a').trace('b') + 10").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(result, Value::Number(15.0));
}

#[test]
fn test_trace_without_handler_works() {
    let data = Value::String("test".to_string());
    let context = InterpreterContext::new(data.clone());

    let expr = parse("trace('label')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(result, data);
}

#[test]
fn test_trace_with_dynamic_name_expression() {
    use std::collections::HashMap;

    let obj = HashMap::from([
        ("id".to_string(), Value::String("identifier".to_string())),
        ("name".to_string(), Value::String("HumanName".to_string())),
    ]);
    let data = Value::object(obj);

    let handler = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext::new(data.clone()).with_trace_handler(handler.clone());

    let expr = parse("trace(id)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(result, data);

    let events = handler.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "identifier");
}

#[test]
fn test_trace_with_this_in_name() {
    let data = Value::String("value".to_string());

    let handler = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext::new(data.clone()).with_trace_handler(handler.clone());

    let expr = parse("trace($this)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    assert_eq!(result, data);

    let events = handler.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "value");
}

#[test]
fn test_now_returns_datetime() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("now()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::DateTime(dt, ..) = result {
        assert!(dt.date().year() >= 2020, "Should have a valid year");
        assert!(dt.time().hour() <= 23, "Should have a valid hour");
    } else {
        panic!("Expected DateTime, got {:?}", result);
    }
}

#[test]
fn test_now_deterministic_within_context() {
    let context = InterpreterContext::new(Value::Null);
    let expr1 = parse("now()").expect("parse failed");
    let expr2 = parse("now()").expect("parse failed");
    let (result1, _) = interpret(&expr1, context.clone()).expect("interpret failed");
    let (result2, _) = interpret(&expr2, context.clone()).expect("interpret failed");
    assert_eq!(
        result1, result2,
        "now() should be deterministic within same context"
    );
}

#[test]
fn test_today_returns_date() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("today()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Date(d, _) = result {
        let expected = context.evaluation_timestamp.date_naive();
        assert_eq!(d.year(), expected.year(), "Should have a valid year");
        assert_eq!(d.month(), expected.month(), "Should have a valid month");
        assert_eq!(d.day(), expected.day(), "Should have a valid day");
    } else {
        panic!("Expected Date, got {:?}", result);
    }
}

#[test]
fn test_today_deterministic_within_context() {
    let context = InterpreterContext::new(Value::Null);
    let expr1 = parse("today()").expect("parse failed");
    let expr2 = parse("today()").expect("parse failed");
    let (result1, _) = interpret(&expr1, context.clone()).expect("interpret failed");
    let (result2, _) = interpret(&expr2, context.clone()).expect("interpret failed");
    assert_eq!(
        result1, result2,
        "today() should be deterministic within same context"
    );
}

#[test]
fn test_today_from_now() {
    let context = InterpreterContext::new(Value::Null);
    let now_expr = parse("now()").expect("parse failed");
    let today_expr = parse("today()").expect("parse failed");

    let (now_result, _) = interpret(&now_expr, context.clone()).expect("interpret failed");
    let (today_result, _) = interpret(&today_expr, context.clone()).expect("interpret failed");

    if let (Value::DateTime(dt, ..), Value::Date(d, _)) = (now_result, today_result) {
        assert_eq!(dt.date(), d, "today() should match date portion of now()");
    } else {
        panic!("Expected DateTime and Date");
    }
}

#[test]
fn test_time_of_day_returns_time() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("timeOfDay()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Time(t, _) = result {
        assert!(t.hour() <= 23, "Should have a valid hour");
        assert!(t.minute() <= 59, "Should have a valid minute");
    } else {
        panic!("Expected Time, got {:?}", result);
    }
}

#[test]
fn test_time_of_day_deterministic_within_context() {
    let context = InterpreterContext::new(Value::Null);
    let expr1 = parse("timeOfDay()").expect("parse failed");
    let expr2 = parse("timeOfDay()").expect("parse failed");
    let (result1, _) = interpret(&expr1, context.clone()).expect("interpret failed");
    let (result2, _) = interpret(&expr2, context.clone()).expect("interpret failed");
    assert_eq!(
        result1, result2,
        "timeOfDay() should be deterministic within same context"
    );
}

#[test]
fn test_time_of_day_from_now() {
    let context = InterpreterContext::new(Value::Null);
    let now_expr = parse("now()").expect("parse failed");
    let time_expr = parse("timeOfDay()").expect("parse failed");

    let (now_result, _) = interpret(&now_expr, context.clone()).expect("interpret failed");
    let (time_result, _) = interpret(&time_expr, context.clone()).expect("interpret failed");

    if let (Value::DateTime(dt, ..), Value::Time(t, _)) = (now_result, time_result) {
        assert_eq!(
            dt.time(),
            t,
            "timeOfDay() should match time portion of now()"
        );
    } else {
        panic!("Expected DateTime and Time");
    }
}

#[test]
fn test_define_variable_basic() {
    let data = Value::Number(42.0);
    let context = InterpreterContext::new(data);

    let expr = parse("defineVariable('x')").expect("parse failed");
    let (result, ctx) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0));
    assert_eq!(ctx.external_constants.get("x"), Some(&Value::Number(42.0)));
}

#[test]
fn test_define_variable_with_expression() {
    let data = Value::Number(10.0);
    let context = InterpreterContext::new(data);

    let expr = parse("defineVariable('doubled', $this * 2)").expect("parse failed");
    let (result, ctx) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0));
    assert_eq!(
        ctx.external_constants.get("doubled"),
        Some(&Value::Number(20.0))
    );
}

#[test]
fn test_define_variable_accessible_downstream() {
    let data = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let context = InterpreterContext::new(data);

    let expr =
        parse("defineVariable('total', count()).select($this + %total)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![
            Value::Number(4.0),
            Value::Number(5.0),
            Value::Number(6.0),
        ])
    );
}

#[test]
fn test_define_variable_in_where() {
    let data = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);
    let context = InterpreterContext::new(data);

    let expr =
        parse("defineVariable('threshold', 3).where($this > %threshold)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![Value::Number(4.0), Value::Number(5.0)])
    );
}

#[test]
fn test_define_variable_returns_base_unchanged() {
    let data = Value::String("hello".to_string());
    let context = InterpreterContext::new(data);

    let expr = parse("defineVariable('x', 99)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_define_variable_on_collection() {
    let data = Value::collection(vec![Value::Number(10.0), Value::Number(20.0)]);
    let context = InterpreterContext::new(data.clone());

    let expr = parse("defineVariable('items')").expect("parse failed");
    let (result, ctx) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, data);
    assert_eq!(ctx.external_constants.get("items"), Some(&data));
}

#[test]
fn test_define_variable_overrides_existing_constant() {
    let context = InterpreterContext::new(Value::Number(42.0))
        .with_constant("x".to_string(), Value::Number(1.0));

    let expr = parse("defineVariable('x', 99)").expect("parse failed");
    let (_, ctx) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(ctx.external_constants.get("x"), Some(&Value::Number(99.0)));
}

#[test]
fn test_define_variable_wrong_arg_count() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("defineVariable()").expect("parse failed");
    let result = interpret(&expr, context.clone());
    assert!(result.is_err());
}

#[test]
fn test_define_variable_chained() {
    let data = Value::Number(5.0);
    let context = InterpreterContext::new(data);

    let expr = parse("defineVariable('a', 10).defineVariable('b', 20)").expect("parse failed");
    let (result, ctx) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0));
    assert_eq!(ctx.external_constants.get("a"), Some(&Value::Number(10.0)));
    assert_eq!(ctx.external_constants.get("b"), Some(&Value::Number(20.0)));
}

#[test]
fn test_define_variable_not_leaked_across_and() {
    let context = InterpreterContext::new(Value::Number(5.0))
        .with_constant("x".to_string(), Value::Number(0.0));

    let expr = parse("defineVariable('x', 99).exists() and (%x = 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_type_boolean() {
    let context = InterpreterContext::new(Value::Boolean(true));
    let expr = parse("type().name").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result.to_vec(), vec![Value::String("Boolean".to_string())]);
}

#[test]
fn test_type_integer() {
    let context = InterpreterContext::new(Value::Number(1.0));
    let expr = parse("type().name").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result.to_vec(), vec![Value::String("Integer".to_string())]);
}

#[test]
fn test_type_decimal() {
    let context = InterpreterContext::new(Value::Number(1.5));
    let expr = parse("type().name").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result.to_vec(), vec![Value::String("Decimal".to_string())]);
}

#[test]
fn test_type_string() {
    let context = InterpreterContext::new(Value::String("hello".to_string()));
    let expr = parse("type().name").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result.to_vec(), vec![Value::String("String".to_string())]);
}

#[test]
fn test_type_quantity() {
    let context = InterpreterContext::new(Value::Quantity(1.0, "kg".to_string(), None));
    let expr = parse("type().name").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result.to_vec(), vec![Value::String("Quantity".to_string())]);
}

#[test]
fn test_type_empty() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("type()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_type_fhir_resource() {
    let patient = Value::object(HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Patient".to_string()),
        ),
        ("id".to_string(), Value::String("123".to_string())),
    ]));
    let context = InterpreterContext::new(patient);
    let expr = parse("type().name").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result.to_vec(), vec![Value::String("Patient".to_string())]);
}
