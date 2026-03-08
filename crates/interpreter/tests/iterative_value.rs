#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use interpreter::Value;
use std::collections::HashMap;

fn nested_singleton(val: Value, depth: usize) -> Value {
    let mut v = val;
    for _ in 0..depth {
        v = Value::collection(vec![v]);
    }
    v
}

#[test]
fn test_to_f64_deep_singleton() {
    let val = nested_singleton(Value::Number(42.0), 10_000);
    assert_eq!(val.to_f64(), Some(42.0));
}

#[test]
fn test_to_f64_singleton_wrapping_empty() {
    let val = nested_singleton(Value::collection(vec![]), 100);
    assert_eq!(val.to_f64(), None);
}

#[test]
fn test_to_f64_singleton_wrapping_multi() {
    let inner = Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]);
    let val = nested_singleton(inner, 100);
    assert_eq!(val.to_f64(), None);
}

#[test]
fn test_to_str_deep_singleton() {
    let val = nested_singleton(Value::String("hello".to_string()), 10_000);
    assert_eq!(val.to_str().unwrap(), "hello");
}

#[test]
fn test_to_str_singleton_wrapping_empty() {
    let val = nested_singleton(Value::collection(vec![]), 100);
    assert_eq!(val.to_str().unwrap(), "[]");
}

#[test]
fn test_to_time_interval_deep_singleton() {
    let val = nested_singleton(
        Value::Quantity(3.0, "days".to_string(), None),
        10_000,
    );
    assert_eq!(
        val.to_time_interval(),
        Some(interpreter::datetime::TimeInterval::Duration(
            chrono::TimeDelta::try_days(3).unwrap()
        ))
    );
}

#[test]
fn test_to_time_interval_singleton_wrapping_empty() {
    let val = nested_singleton(Value::collection(vec![]), 100);
    assert_eq!(val.to_time_interval(), None);
}

#[test]
fn test_compare_to_deep_singleton_both_sides() {
    let left = nested_singleton(Value::Number(1.0), 5_000);
    let right = nested_singleton(Value::Number(2.0), 5_000);
    assert_eq!(left.compare_equal(&right), Some(std::cmp::Ordering::Less));
}

#[test]
fn test_compare_to_deep_singleton_left_only() {
    let left = nested_singleton(Value::Number(5.0), 10_000);
    let right = Value::Number(5.0);
    assert_eq!(left.compare_equal(&right), Some(std::cmp::Ordering::Equal));
}

#[test]
fn test_compare_to_singleton_wrapping_multi() {
    let inner = Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]);
    let left = nested_singleton(inner, 100);
    assert_eq!(left.compare_equal(&Value::Number(1.0)), None);
}

#[test]
fn test_compare_precision_deep_singleton() {
    use interpreter::DatePrecision;
    let left = nested_singleton(
        Value::Date(
            chrono::NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            DatePrecision::Day,
        ),
        10_000,
    );
    let right = nested_singleton(
        Value::Date(
            chrono::NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            DatePrecision::Month,
        ),
        10_000,
    );
    assert_eq!(
        left.compare_precision(&right),
        Some(std::cmp::Ordering::Greater)
    );
}

#[test]
fn test_equals_nested_collections() {
    let a = Value::collection(vec![
        Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::collection(vec![Value::Number(3.0), Value::Number(4.0)]),
    ]);
    let b = Value::collection(vec![
        Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::collection(vec![Value::Number(3.0), Value::Number(4.0)]),
    ]);
    assert!(a.equals(&b));
}

#[test]
fn test_equals_nested_collections_mismatch() {
    let a = Value::collection(vec![Value::collection(vec![
        Value::Number(1.0),
        Value::Number(2.0),
    ])]);
    let b = Value::collection(vec![Value::collection(vec![
        Value::Number(1.0),
        Value::Number(9.0),
    ])]);
    assert!(!a.equals(&b));
}

#[test]
fn test_equals_nested_objects() {
    let make = |x: f64| {
        Value::object(HashMap::from([(
            "item".to_string(),
            Value::object(HashMap::from([(
                "x".to_string(),
                Value::Number(x),
            )])),
        )]))
    };
    assert!(make(1.0).equals(&make(1.0)));
    assert!(!make(1.0).equals(&make(999.0)));
}

#[test]
fn test_equivalent_different_order() {
    let a = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let b = Value::collection(vec![
        Value::Number(3.0),
        Value::Number(1.0),
        Value::Number(2.0),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_nested_collections_different_order() {
    let a = Value::collection(vec![
        Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::collection(vec![Value::Number(3.0), Value::Number(4.0)]),
    ]);
    let b = Value::collection(vec![
        Value::collection(vec![Value::Number(3.0), Value::Number(4.0)]),
        Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_case_insensitive_different_order() {
    let a = Value::collection(vec![
        Value::String("Alpha".to_string()),
        Value::String("BETA".to_string()),
    ]);
    let b = Value::collection(vec![
        Value::String("beta".to_string()),
        Value::String("alpha".to_string()),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_different_lengths() {
    let a = Value::collection(vec![Value::Number(1.0)]);
    let b = Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]);
    assert!(!a.equivalent(&b));
}

#[test]
fn test_equivalent_duplicates_match() {
    let a = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(1.0),
        Value::Number(2.0),
    ]);
    let b = Value::collection(vec![
        Value::Number(2.0),
        Value::Number(1.0),
        Value::Number(1.0),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_duplicates_mismatch() {
    let a = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(1.0),
        Value::Number(2.0),
    ]);
    let b = Value::collection(vec![
        Value::Number(2.0),
        Value::Number(2.0),
        Value::Number(1.0),
    ]);
    assert!(!a.equivalent(&b));
}

#[test]
fn test_equivalent_mixed_types_sorted_correctly() {
    let a = Value::collection(vec![
        Value::Number(1.0),
        Value::String("hello".to_string()),
        Value::Boolean(true),
    ]);
    let b = Value::collection(vec![
        Value::Boolean(true),
        Value::Number(1.0),
        Value::String("HELLO".to_string()),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_mixed_types_not_swapped() {
    let a = Value::collection(vec![
        Value::Number(1.0),
        Value::String("2".to_string()),
    ]);
    let b = Value::collection(vec![
        Value::Number(2.0),
        Value::String("1".to_string()),
    ]);
    assert!(!a.equivalent(&b));
}

#[test]
fn test_equivalent_all_types_reordered() {
    let null = Value::Null;
    let bool_val = Value::Boolean(false);
    let num = Value::Number(3.125);
    let string = Value::String("test".to_string());
    let quantity = Value::Quantity(10.0, "mg".to_string(), None);

    let a = Value::collection(vec![
        null.clone(),
        bool_val.clone(),
        num.clone(),
        string.clone(),
        quantity.clone(),
    ]);
    let b = Value::collection(vec![quantity, string, null, num, bool_val]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_same_type_different_values_not_confused() {
    let a = Value::collection(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::String("a".to_string()),
        Value::String("b".to_string()),
    ]);
    let b = Value::collection(vec![
        Value::String("a".to_string()),
        Value::Number(2.0),
        Value::String("b".to_string()),
        Value::Number(1.0),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_same_type_wrong_values_after_sort() {
    let a = Value::collection(vec![
        Value::Number(1.0),
        Value::String("b".to_string()),
    ]);
    let b = Value::collection(vec![
        Value::Number(1.0),
        Value::String("c".to_string()),
    ]);
    assert!(!a.equivalent(&b));
}

#[test]
fn test_equivalent_nested_objects() {
    let make = |s: &str| {
        Value::object(HashMap::from([(
            "key".to_string(),
            Value::String(s.to_string()),
        )]))
    };
    assert!(make("Value").equivalent(&make("value")));
    assert!(!make("one").equivalent(&make("two")));
}

#[test]
fn test_equivalent_objects_different_keys_reordered() {
    let a = Value::collection(vec![
        Value::object(HashMap::from([("a".to_string(), Value::Number(1.0))])),
        Value::object(HashMap::from([("b".to_string(), Value::Number(2.0))])),
    ]);
    let b = Value::collection(vec![
        Value::object(HashMap::from([("b".to_string(), Value::Number(2.0))])),
        Value::object(HashMap::from([("a".to_string(), Value::Number(1.0))])),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_objects_different_keys_not_equivalent() {
    let a = Value::collection(vec![
        Value::object(HashMap::from([("a".to_string(), Value::Number(1.0))])),
        Value::object(HashMap::from([("b".to_string(), Value::Number(2.0))])),
    ]);
    let b = Value::collection(vec![
        Value::object(HashMap::from([("a".to_string(), Value::Number(1.0))])),
        Value::object(HashMap::from([("c".to_string(), Value::Number(2.0))])),
    ]);
    assert!(!a.equivalent(&b));
}

#[test]
fn test_equivalent_objects_with_nested_collection_values_reordered() {
    let a = Value::object(HashMap::from([(
        "items".to_string(),
        Value::collection(vec![Value::Number(2.0), Value::Number(1.0)]),
    )]));
    let b = Value::object(HashMap::from([(
        "items".to_string(),
        Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]),
    )]));
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_mixed_depth_reordered() {
    let a = Value::collection(vec![
        Value::Number(42.0),
        Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]),
    ]);
    let b = Value::collection(vec![
        Value::collection(vec![Value::Number(2.0), Value::Number(1.0)]),
        Value::Number(42.0),
    ]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_deep_objects_in_collections_reordered() {
    let make_patient = |name: &str, age: f64| {
        Value::object(HashMap::from([
            ("name".to_string(), Value::String(name.to_string())),
            (
                "address".to_string(),
                Value::object(HashMap::from([(
                    "city".to_string(),
                    Value::String("NYC".to_string()),
                )])),
            ),
            ("age".to_string(), Value::Number(age)),
        ]))
    };
    let a = Value::collection(vec![make_patient("Alice", 30.0), make_patient("Bob", 25.0)]);
    let b = Value::collection(vec![make_patient("bob", 25.0), make_patient("alice", 30.0)]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_case_insensitive_deep_in_nested_structure() {
    let a = Value::collection(vec![Value::object(HashMap::from([(
        "tags".to_string(),
        Value::collection(vec![
            Value::String("URGENT".to_string()),
            Value::String("Review".to_string()),
        ]),
    )]))]);
    let b = Value::collection(vec![Value::object(HashMap::from([(
        "tags".to_string(),
        Value::collection(vec![
            Value::String("review".to_string()),
            Value::String("urgent".to_string()),
        ]),
    )]))]);
    assert!(a.equivalent(&b));
}

#[test]
fn test_equivalent_nested_sets_reordered() {
    let a = Value::collection(vec![
        Value::collection(vec![Value::Number(2.0), Value::Number(1.0)]),
        Value::collection(vec![Value::Number(1.0), Value::Number(3.0)]),
    ]);
    let b = Value::collection(vec![
        Value::collection(vec![Value::Number(1.0), Value::Number(2.0)]),
        Value::collection(vec![Value::Number(3.0), Value::Number(1.0)]),
    ]);
    assert!(a.equivalent(&b));
}
