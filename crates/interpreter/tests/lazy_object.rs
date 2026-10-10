#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod nested_node;

use interpreter::{InterpreterContext, Value, interpret};
use nested_node::NestedNodeSource;
use parser::parse;

#[test]
fn member_access_loads_only_visited_fields() {
    let (node, field_loads) = NestedNodeSource::nested(10);
    let expr = parse("item.item.value").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(node)).expect("interpret failed");
    assert_eq!(result, Value::number(8.0, 0));
    assert_eq!(field_loads.get(), 3);
}

#[test]
fn type_check_reads_only_resource_type() {
    let (node, field_loads) = NestedNodeSource::nested(10);
    let expr = parse("item.is(Node)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(node)).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
    assert_eq!(field_loads.get(), 2);
}

#[test]
fn missing_field_loads_one_level_only() {
    let (node, field_loads) = NestedNodeSource::nested(10);
    let expr = parse("absent").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(node)).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
    assert_eq!(field_loads.get(), 3);
}

#[test]
fn equal_lazy_objects_compare_equal() {
    let (left, _) = NestedNodeSource::nested(3);
    let (right, _) = NestedNodeSource::nested(3);
    assert!(left.compare_equal(&right).is_equal());
}

#[test]
fn deep_lazy_chain_traverses_and_drops_without_stack_overflow() {
    let depth = 100_000;
    let (node, _) = NestedNodeSource::nested(depth);
    let expr = parse("repeatAll(item).count()").expect("parse failed");
    let (result, context) =
        interpret(&expr, InterpreterContext::new(node)).expect("interpret failed");
    assert_eq!(result, Value::number(f64::from(depth), 0));
    drop(context);
}
