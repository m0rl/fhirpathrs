#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant,
    clippy::manual_string_new
)]
use interpreter::{InterpreterContext, InterpreterError, Value, interpret};
use parser::parse;
use std::collections::HashMap;

#[test]
fn test_literal_interpretation() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("42").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));

    let expr = parse("true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'hello'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_arithmetic_operations() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 + 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("10 - 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(7.0, 0));

    let expr = parse("4 * 5").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(20.0, 0));

    let expr = parse("15 / 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(5.0, 0));
}

#[test]
fn test_div_operator() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("10 div 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("9 div 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("(-10) div 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-3.0, 0));

    let expr = parse("10 div (-3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-3.0, 0));

    let expr = parse("(-10) div (-3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("2 div 5").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_mod_operator() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("10 mod 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("9 mod 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));

    let expr = parse("(-10) mod 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-1.0, 0));

    let expr = parse("2 mod 5").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_null_propagation_arithmetic() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{} + 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("1 + {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} * 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} - 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_null_propagation_string_concat() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{} & 'text'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("text".to_string()));

    let expr = parse("'text' & {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("text".to_string()));

    let expr = parse("{} & {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_null_propagation_and() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("false and {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("{} and false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("{} and true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("true and {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} and {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_null_propagation_or() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true or {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("{} or true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("{} or false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("false or {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} or {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_null_propagation_xor() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{} xor true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("true xor {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_null_propagation_implies() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("false implies {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("{} implies true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("{} implies false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("true implies {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_logical_operations() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true and false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("true or false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("2 > 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("5 = 5").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_concatenation() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello' & ' world'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_collection_operations() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]));

    let expr = parse("1 | 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.contains(&Value::Number(1.0, 0)));
        assert!(items.contains(&Value::Number(2.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_union_deduplication() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 | 2 | 1 | 3 | 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3, "Union should deduplicate: got {:?}", items);
        assert!(items.contains(&Value::Number(1.0, 0)));
        assert!(items.contains(&Value::Number(2.0, 0)));
        assert!(items.contains(&Value::Number(3.0, 0)));
    } else {
        panic!("Expected collection");
    }

    let expr = parse("'a' | 'a'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(
            items.len(),
            1,
            "Union of identical values should have length 1"
        );
        assert_eq!(items[0], Value::String("a".to_string()));
    } else {
        panic!("Expected collection");
    }

    let expr = parse("(1 | 2) | (2 | 3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(
            items.len(),
            3,
            "Nested union should deduplicate: got {:?}",
            items
        );
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_equality_vs_equivalence_strings() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'Hello' = 'hello'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(false),
        "Equality should be case-sensitive"
    );

    let expr = parse("'Hello' = 'Hello'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(true),
        "Equality should match identical strings"
    );

    let expr = parse("'Hello' ~ 'hello'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(true),
        "Equivalence should be case-insensitive"
    );

    let expr = parse("'HELLO' ~ 'hello'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(true),
        "Equivalence should be case-insensitive"
    );

    let expr = parse("'Hello' != 'hello'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(true),
        "Not equal should be case-sensitive"
    );

    let expr = parse("'Hello' !~ 'hello'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(false),
        "Not equivalent should be case-insensitive"
    );
}

#[test]
fn test_equality_vs_equivalence_numbers() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 = 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1 ~ 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1.0 = 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1.0 ~ 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_equality_vs_equivalence_quantity() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 'cm' = 1 'cm'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true), "Same unit should be equal");

    let expr = parse("1 'cm' = 1 'm'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(false),
        "Different units should not be equal"
    );

    let expr = parse("1 'cm' ~ 1 'CM'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::Boolean(true),
        "Equivalence should be case-insensitive for units"
    );
}

#[test]
fn test_is_operator_integer() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 is Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("42 is Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1.0 is Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("1.5 is Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("3.14 is Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_is_operator_decimal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 is Decimal").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1.5 is Decimal").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("3.14159 is Decimal").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_is_operator_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello' is String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'' is String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("123 is String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_is_operator_boolean() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true is Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("false is Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'true' is Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_is_operator_date_time() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("@2024-01-15 is Date").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("@2024-01-15T10:30:00 is DateTime").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("@T10:30:00 is Time").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("@2024-01-15 is DateTime").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_is_operator_quantity() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("10 'kg' is Quantity").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("3.5 'cm' is Quantity").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("10 is Quantity").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_as_operator_string_to_integer() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'123' as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(123.0, 0));

    let expr = parse("'12.7' as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(12.0, 0));

    let expr = parse("'abc' as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("'  42  ' as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_as_operator_string_to_decimal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'3.14' as Decimal").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.14, 0));

    let expr = parse("'42' as Decimal").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));

    let expr = parse("'invalid' as Decimal").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_as_operator_string_to_boolean() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'true' as Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'false' as Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'yes' as Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'no' as Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'maybe' as Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_as_operator_number_to_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("123 as String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("123".to_string()));

    let expr = parse("3.14 as String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("3.14".to_string()));
}

#[test]
fn test_as_operator_decimal_to_integer() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("3.7 as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("3.2 as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let expr = parse("(-3.7) as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-3.0, 0));
}

#[test]
fn test_as_operator_boolean_to_string() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true as String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("true".to_string()));

    let expr = parse("false as String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("false".to_string()));
}

#[test]
fn test_as_operator_boolean_to_integer() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));

    let expr = parse("false as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_as_operator_same_type() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello' as String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("hello".to_string()));

    let expr = parse("42 as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));

    let expr = parse("true as Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_as_operator_quantity_conversions() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("10 'kg' as String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("10 'kg'".to_string()));

    let expr = parse("10 'kg' as Decimal").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));

    let expr = parse("10.5 'kg' as Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));
}

#[test]
fn test_type_operators_with_qualified_names() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello' is System.String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("42 is System.Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_indexer_valid_index() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(10 | 20 | 30)[0]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));

    let expr = parse("(10 | 20 | 30)[1]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(20.0, 0));

    let expr = parse("(10 | 20 | 30)[2]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(30.0, 0));
}

#[test]
fn test_indexer_out_of_bounds_returns_empty() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(1 | 2)[5]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("(1 | 2)[100]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_indexer_negative_returns_empty() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(1 | 2)[-1]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_indexer_on_singleton() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("42[0]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));

    let expr = parse("42[1]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_indexer_on_empty_returns_empty() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("({}).first()[0]").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_string_comparison_less_than() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'apple' < 'banana'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'banana' < 'apple'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("'apple' < 'apple'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_string_comparison_greater_than() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'zebra' > 'apple'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'apple' > 'zebra'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_string_comparison_less_equal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'apple' <= 'banana'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'apple' <= 'apple'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'banana' <= 'apple'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_string_comparison_greater_equal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'zebra' >= 'apple'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'apple' >= 'apple'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'apple' >= 'zebra'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_string_comparison_case_sensitive() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'A' < 'a'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'Z' < 'a'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_string_comparison_empty_strings() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'' < 'a'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("'' <= ''").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_is_fhir_resource_type() {
    use std::collections::HashMap;

    let patient = HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Patient".to_string()),
        ),
        ("id".to_string(), Value::String("123".to_string())),
    ]);
    let context = InterpreterContext::new(Value::object(patient));

    let expr = parse("$this is Patient").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("$this is Observation").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_is_fhir_qualified_name() {
    use std::collections::HashMap;

    let patient = HashMap::from([(
        "resourceType".to_string(),
        Value::String("Patient".to_string()),
    )]);
    let context = InterpreterContext::new(Value::object(patient));

    let expr = parse("$this is FHIR.Patient").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("$this is FHIR.Observation").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_is_fhir_vs_system_namespace() {
    use std::collections::HashMap;

    let patient = HashMap::from([(
        "resourceType".to_string(),
        Value::String("Patient".to_string()),
    )]);
    let context = InterpreterContext::new(Value::object(patient));

    let expr = parse("$this is System.String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("$this is String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let context = InterpreterContext::new(Value::Null);

    let expr = parse("'hello' is FHIR.String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("42 is FHIR.Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_as_fhir_resource_type() {
    use std::collections::HashMap;

    let patient = HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Patient".to_string()),
        ),
        ("id".to_string(), Value::String("123".to_string())),
    ]);
    let context = InterpreterContext::new(Value::object(patient.clone()));

    let expr = parse("$this as Patient").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::object(patient));

    let expr = parse("$this as Observation").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_object_without_resource_type() {
    use std::collections::HashMap;

    let obj = HashMap::from([
        ("name".to_string(), Value::String("John".to_string())),
        ("age".to_string(), Value::Number(30.0, 0)),
    ]);
    let context = InterpreterContext::new(Value::object(obj));

    let expr = parse("$this is Patient").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("$this is FHIR.Patient").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_null_equality_propagation() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{} = {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} != {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} = 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("1 = {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_null_equivalence_still_works() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{} ~ {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("{} !~ 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_null_inequality_propagation() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{} < 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("1 > {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} <= {}").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));

    let expr = parse("{} >= 5").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_deeply_nested_expression() {
    let context = InterpreterContext::new(Value::Null);

    let depth = 1000;
    let input = "(".repeat(depth) + "1 + 2" + &")".repeat(depth);
    let expr = parse(&input).expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));
}

#[test]
fn test_numeric_less_than() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 < 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("2 < 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("1 < 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_numeric_less_equal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 <= 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1 <= 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("2 <= 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_numeric_greater_equal() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("2 >= 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1 >= 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1 >= 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_membership_in() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]));

    let expr = parse("2 in $this").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("5 in $this").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_membership_contains() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]));

    let expr = parse("$this contains 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("$this contains 5").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_membership_in_singleton() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 in 1").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1 in 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_unary_plus() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("+42").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_unary_minus() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("-42").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-42.0, 0));

    let expr = parse("-3.14").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(-3.14, 2));
}

#[test]
fn test_unary_minus_quantity() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("-(5 'mg')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(-5.0, 0, "mg".to_string(), None));
}

#[test]
fn test_unary_minus_type_error() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("-true").expect("parse failed");
    let result = interpret(&expr, context.clone());
    assert!(result.is_err());
}

#[test]
fn test_division_by_zero() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("1 / 0").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret");
    assert_eq!(result.to_vec(), vec![]);

    let expr = parse("10 div 0").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret");
    assert_eq!(result.to_vec(), vec![]);

    let expr = parse("10 mod 0").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret");
    assert_eq!(result.to_vec(), vec![]);
}

#[test]
fn test_unknown_function_error() {
    let context = InterpreterContext::new(Value::Number(1.0, 0));

    let expr = parse("$this.nonExistentFunc()").expect("parse failed");
    let result = interpret(&expr, context);
    assert!(matches!(result, Err(InterpreterError::UnknownFunction(_))));
}

#[test]
fn test_unknown_constant_error() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("%undefinedConstant").expect("parse failed");
    let result = interpret(&expr, context);
    assert!(matches!(result, Err(InterpreterError::UnknownConstant(_))));
}

#[test]
fn test_external_constant() {
    let context = InterpreterContext::new(Value::Null)
        .with_constant("myConst".to_string(), Value::Number(42.0, 0));

    let expr = parse("%myConst").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_external_constant_in_expression() {
    let context = InterpreterContext::new(Value::Null)
        .with_constant("threshold".to_string(), Value::Number(100.0, 0));

    let expr = parse("50 < %threshold").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("150 < %threshold").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_external_constant_valueset_membership() {
    let allowed_codes = Value::collection(vec![
        Value::String("male".to_string()),
        Value::String("female".to_string()),
        Value::String("other".to_string()),
        Value::String("unknown".to_string()),
    ]);
    let context = InterpreterContext::new(Value::String("male".to_string()))
        .with_constant("vs-gender".to_string(), allowed_codes);

    let expr = parse("$this in %'vs-gender'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let context = InterpreterContext::new(Value::String("invalid".to_string())).with_constant(
        "vs-gender".to_string(),
        Value::collection(vec![
            Value::String("male".to_string()),
            Value::String("female".to_string()),
        ]),
    );

    let expr = parse("$this in %'vs-gender'").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_quantity_multiply_by_scalar() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(5 'mg') * 3").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(15.0, 0, "mg".to_string(), None));

    let expr = parse("3 * (5 'mg')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(15.0, 0, "mg".to_string(), None));
}

#[test]
fn test_quantity_divide_by_scalar() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(10 'mg') / 2").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(5.0, 0, "mg".to_string(), None));
}

#[test]
fn test_quantity_divide_by_zero() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(10 'mg') / 0").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret");
    assert_eq!(result.to_vec(), vec![]);
}

#[test]
fn test_is_null_returns_false() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{} is Integer").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("{} is String").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("{} is Boolean").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_member_access_on_object() {
    let patient = HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Patient".to_string()),
        ),
        ("id".to_string(), Value::String("123".to_string())),
        ("gender".to_string(), Value::String("male".to_string())),
        ("active".to_string(), Value::Boolean(true)),
    ]);
    let context = InterpreterContext::new(Value::object(patient));

    let expr = parse("gender").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("male".to_string()));

    let expr = parse("active").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_nested_member_access() {
    let name = HashMap::from([
        ("family".to_string(), Value::String("Smith".to_string())),
        (
            "given".to_string(),
            Value::collection(vec![Value::String("John".to_string())]),
        ),
    ]);
    let patient = HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Patient".to_string()),
        ),
        (
            "name".to_string(),
            Value::collection(vec![Value::object(name)]),
        ),
    ]);
    let context = InterpreterContext::new(Value::object(patient));

    let expr = parse("name.family").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![Value::String("Smith".to_string())])
    );
}

#[test]
fn test_not_function() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true.not()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("false.not()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_empty_and_exists() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{}.empty()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("1.empty()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("{}.exists()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("1.exists()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_count_function() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]));

    let expr = parse("$this.count()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));

    let context = InterpreterContext::new(Value::Null);

    let expr = parse("{}.count()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_clinical_age_check() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("today() - 30 years").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert!(matches!(result, Value::Date(..)));
}

#[test]
fn test_clinical_where_filter_on_fhir_data() {
    let obs1 = Value::object(HashMap::from([
        ("code".to_string(), Value::String("glucose".to_string())),
        ("value".to_string(), Value::Number(120.0, 0)),
    ]));
    let obs2 = Value::object(HashMap::from([
        ("code".to_string(), Value::String("cholesterol".to_string())),
        ("value".to_string(), Value::Number(200.0, 0)),
    ]));
    let obs3 = Value::object(HashMap::from([
        ("code".to_string(), Value::String("glucose".to_string())),
        ("value".to_string(), Value::Number(95.0, 0)),
    ]));
    let context = InterpreterContext::new(Value::collection(vec![obs1, obs2, obs3]));

    let expr = parse("$this.where(code = 'glucose').count()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_clinical_where_filter_value_range() {
    let obs1 = Value::object(HashMap::from([
        ("code".to_string(), Value::String("glucose".to_string())),
        ("value".to_string(), Value::Number(120.0, 0)),
    ]));
    let obs2 = Value::object(HashMap::from([
        ("code".to_string(), Value::String("glucose".to_string())),
        ("value".to_string(), Value::Number(95.0, 0)),
    ]));
    let obs3 = Value::object(HashMap::from([
        ("code".to_string(), Value::String("glucose".to_string())),
        ("value".to_string(), Value::Number(250.0, 0)),
    ]));
    let context = InterpreterContext::new(Value::collection(vec![obs1, obs2, obs3]));

    let expr = parse("$this.where(value > 100).count()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));

    let expr = parse("$this.where(value >= 100 and value <= 200).count()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));
}

#[test]
fn test_clinical_iif_expression() {
    let context = InterpreterContext::new(Value::Number(250.0, 0));

    let expr = parse("iif($this > 200, 'high', 'normal')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("high".to_string()));

    let context = InterpreterContext::new(Value::Number(150.0, 0));
    let expr = parse("iif($this > 200, 'high', 'normal')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("normal".to_string()));
}

#[test]
fn test_parenthesized_expression() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("(2 + 3) * 4").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(20.0, 0));

    let expr = parse("2 + 3 * 4").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(14.0, 0));
}

#[test]
fn test_chained_boolean_logic() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true and true and true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("true and true and false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("false or false or true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_implies_truth_table() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true implies true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("true implies false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("false implies true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("false implies false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_xor_truth_table() {
    let context = InterpreterContext::new(Value::Null);

    let expr = parse("true xor true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let expr = parse("true xor false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("false xor true").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("false xor false").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}
