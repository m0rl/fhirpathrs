#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;
use std::collections::HashMap;

#[test]
fn test_where_with_this_context() {
    let mut obj1 = HashMap::new();
    obj1.insert("value".to_string(), Value::Number(10.0, 0));
    let mut obj2 = HashMap::new();
    obj2.insert("value".to_string(), Value::Number(5.0, 0));
    let mut obj3 = HashMap::new();
    obj3.insert("value".to_string(), Value::Number(15.0, 0));

    let data = Value::collection(vec![
        Value::object(obj1),
        Value::object(obj2),
        Value::object(obj3),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("where($this.value > 8)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Collection(ref items) = result {
        assert_eq!(
            items.len(),
            2,
            "Should filter to 2 items where value > 8, got {:?}",
            items
        );
    } else {
        panic!("Expected collection, got {:?}", result);
    }
}

#[test]
fn test_select_with_this_context() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("select($this * 2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::Number(2.0, 0), "First should be 2.0");
        assert_eq!(items[1], Value::Number(4.0, 0), "Second should be 4.0");
        assert_eq!(items[2], Value::Number(6.0, 0), "Third should be 6.0");
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_where_with_index_context() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
        Value::String("d".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("where($index mod 2 = 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2, "Should have 2 items at even indices");
        assert_eq!(items[0], Value::String("a".to_string()));
        assert_eq!(items[1], Value::String("c".to_string()));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_select_with_index_context() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("select($index)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::Number(0.0, 0));
        assert_eq!(items[1], Value::Number(1.0, 0));
        assert_eq!(items[2], Value::Number(2.0, 0));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_where_with_total_context() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
        Value::String("d".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("where($index < $total - 1)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3, "Should have 3 items (all but last)");
        assert_eq!(items[0], Value::String("a".to_string()));
        assert_eq!(items[1], Value::String("b".to_string()));
        assert_eq!(items[2], Value::String("c".to_string()));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_select_with_total_context() {
    let data = Value::collection(vec![
        Value::Number(10.0, 0),
        Value::Number(20.0, 0),
        Value::Number(30.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("select($total)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");

    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::Number(3.0, 0));
        assert_eq!(items[1], Value::Number(3.0, 0));
        assert_eq!(items[2], Value::Number(3.0, 0));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_all_with_total_context() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("all($total = 3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_exists_with_total_context() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("exists($index = $total - 1)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_collection_single() {
    let context = InterpreterContext::new(Value::collection(vec![Value::Number(42.0, 0)]));

    let expr = parse("single()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("single()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Null);

    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
    ]));
    let expr = parse("single()").expect("parse failed");
    let result = interpret(&expr, context.clone());
    assert!(result.is_err());
}

#[test]
fn test_collection_tail() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("tail()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], Value::Number(2.0, 0));
        assert_eq!(items[1], Value::Number(3.0, 0));
    } else {
        panic!("Expected collection");
    }

    let context = InterpreterContext::new(Value::collection(vec![Value::Number(1.0, 0)]));
    let expr = parse("tail()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert!(items.is_empty());
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_collection_take() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
        Value::Number(4.0, 0),
        Value::Number(5.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("take(3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::Number(1.0, 0));
        assert_eq!(items[1], Value::Number(2.0, 0));
        assert_eq!(items[2], Value::Number(3.0, 0));
    } else {
        panic!("Expected collection");
    }

    let expr = parse("take(0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert!(items.is_empty());
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_collection_skip() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
        Value::Number(4.0, 0),
        Value::Number(5.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("skip(2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::Number(3.0, 0));
        assert_eq!(items[1], Value::Number(4.0, 0));
        assert_eq!(items[2], Value::Number(5.0, 0));
    } else {
        panic!("Expected collection");
    }

    let expr = parse("skip(10)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert!(items.is_empty());
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_collection_distinct() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(1.0, 0),
        Value::Number(3.0, 0),
        Value::Number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("distinct()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert!(items.contains(&Value::Number(1.0, 0)));
        assert!(items.contains(&Value::Number(2.0, 0)));
        assert!(items.contains(&Value::Number(3.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_collection_is_distinct() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]));

    let expr = parse("isDistinct()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(1.0, 0),
    ]));

    let expr = parse("isDistinct()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_collection_intersect() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("intersect(2 | 3 | 4)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.contains(&Value::Number(2.0, 0)));
        assert!(items.contains(&Value::Number(3.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_collection_exclude() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
        Value::Number(4.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("exclude(2 | 4)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.contains(&Value::Number(1.0, 0)));
        assert!(items.contains(&Value::Number(3.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_collection_all_true() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(true),
        Value::Boolean(true),
        Value::Boolean(true),
    ]));

    let expr = parse("allTrue()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(true),
        Value::Boolean(false),
        Value::Boolean(true),
    ]));

    let expr = parse("allTrue()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("allTrue()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_collection_any_true() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(false),
        Value::Boolean(false),
        Value::Boolean(true),
    ]));

    let expr = parse("anyTrue()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(false),
        Value::Boolean(false),
    ]));

    let expr = parse("anyTrue()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_collection_all_false() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(false),
        Value::Boolean(false),
    ]));

    let expr = parse("allFalse()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(false),
        Value::Boolean(true),
    ]));

    let expr = parse("allFalse()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_collection_any_false() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(true),
        Value::Boolean(false),
    ]));

    let expr = parse("anyFalse()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let context = InterpreterContext::new(Value::collection(vec![
        Value::Boolean(true),
        Value::Boolean(true),
    ]));

    let expr = parse("anyFalse()").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_collection_subset_of() {
    let data = Value::collection(vec![Value::Number(1.0, 0), Value::Number(2.0, 0)]);
    let context = InterpreterContext::new(data);

    let expr = parse("subsetOf(1 | 2 | 3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("subsetOf(1 | 3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_collection_superset_of() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("supersetOf(1 | 2)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("supersetOf(1 | 4)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_collection_all_with_criteria() {
    let data = Value::collection(vec![
        Value::Number(2.0, 0),
        Value::Number(4.0, 0),
        Value::Number(6.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("all($this > 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("all($this > 3)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));

    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("all($this > 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_collection_exists_with_criteria() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(5.0, 0),
        Value::Number(3.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("exists($this > 4)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));

    let expr = parse("exists($this > 10)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_collection_repeat() {
    let item3 = Value::object(HashMap::from([
        ("name".to_string(), Value::String("item3".to_string())),
        ("item".to_string(), Value::collection(vec![])),
    ]));
    let item2 = Value::object(HashMap::from([
        ("name".to_string(), Value::String("item2".to_string())),
        ("item".to_string(), Value::collection(vec![])),
    ]));
    let item1 = Value::object(HashMap::from([
        ("name".to_string(), Value::String("item1".to_string())),
        ("item".to_string(), Value::collection(vec![item3.clone()])),
    ]));

    let root = HashMap::from([
        ("name".to_string(), Value::String("root".to_string())),
        (
            "item".to_string(),
            Value::collection(vec![item1.clone(), item2.clone()]),
        ),
    ]);

    let context = InterpreterContext::new(Value::object(root));

    let expr = parse("repeat(item)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3, "Should find item1, item2, and item3");
        assert_eq!(items[0], item1);
        assert_eq!(items[1], item2);
        assert_eq!(items[2], item3);
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_filters_by_string() {
    let data = Value::collection(vec![
        Value::String("hello".to_string()),
        Value::Number(42.0, 0),
        Value::String("world".to_string()),
        Value::Boolean(true),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(String)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], Value::String("hello".to_string()));
        assert_eq!(items[1], Value::String("world".to_string()));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_filters_by_integer() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.5, 1),
        Value::Number(3.0, 0),
        Value::String("four".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(Integer)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2, "Should only include whole numbers");
        assert_eq!(items[0], Value::Number(1.0, 0));
        assert_eq!(items[1], Value::Number(3.0, 0));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_filters_by_decimal() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.5, 1),
        Value::String("three".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(Decimal)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2, "All numbers are Decimal");
        assert_eq!(items[0], Value::Number(1.0, 0));
        assert_eq!(items[1], Value::Number(2.5, 1));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_filters_by_boolean() {
    let data = Value::collection(vec![
        Value::Boolean(true),
        Value::Number(1.0, 0),
        Value::Boolean(false),
        Value::String("true".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(Boolean)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], Value::Boolean(true));
        assert_eq!(items[1], Value::Boolean(false));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_with_qualified_name() {
    let data = Value::collection(vec![
        Value::String("hello".to_string()),
        Value::Number(42.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(System.String)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], Value::String("hello".to_string()));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_returns_empty_when_no_match() {
    let data = Value::collection(vec![Value::Number(1.0, 0), Value::Number(2.0, 0)]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(String)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert!(items.is_empty());
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_on_empty_collection() {
    let context = InterpreterContext::new(Value::collection(vec![]));

    let expr = parse("ofType(String)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert!(items.is_empty());
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_fhir_resources() {
    let patient = Value::object(HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Patient".to_string()),
        ),
        ("id".to_string(), Value::String("p1".to_string())),
    ]));
    let observation = Value::object(HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Observation".to_string()),
        ),
        ("id".to_string(), Value::String("o1".to_string())),
    ]));

    let data = Value::collection(vec![patient.clone(), observation.clone()]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(Patient)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], patient);
    } else {
        panic!("Expected collection");
    }

    let expr = parse("ofType(Observation)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], observation);
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_fhir_qualified_name() {
    let patient = Value::object(HashMap::from([(
        "resourceType".to_string(),
        Value::String("Patient".to_string()),
    )]));

    let data = Value::collection(vec![patient.clone()]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(FHIR.Patient)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], patient);
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_of_type_mixed_primitives_and_resources() {
    let patient = Value::object(HashMap::from([(
        "resourceType".to_string(),
        Value::String("Patient".to_string()),
    )]));

    let data = Value::collection(vec![
        Value::String("hello".to_string()),
        patient.clone(),
        Value::Number(42.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("ofType(String)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], Value::String("hello".to_string()));
    } else {
        panic!("Expected collection");
    }

    let expr = parse("ofType(Patient)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], patient);
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_aggregate_sum() {
    let data = Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
        Value::Number(4.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("aggregate($total + $this, 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(10.0, 0));
}

#[test]
fn test_aggregate_product() {
    let data = Value::collection(vec![
        Value::Number(2.0, 0),
        Value::Number(3.0, 0),
        Value::Number(4.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("aggregate($total * $this, 1)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(24.0, 0));
}

#[test]
fn test_aggregate_string_concat() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("aggregate($total & $this, '')").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::String("abc".to_string()));
}

#[test]
fn test_aggregate_without_init() {
    let data = Value::collection(vec![Value::Number(1.0, 0), Value::Number(2.0, 0)]);
    let context = InterpreterContext::new(data);

    let expr =
        parse("aggregate(iif($total.empty(), $this, $total + $this))").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));
}

#[test]
fn test_aggregate_on_empty_collection() {
    let data = Value::collection(vec![]);
    let context = InterpreterContext::new(data);

    let expr = parse("aggregate($total + $this, 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(0.0, 0));
}

#[test]
fn test_aggregate_on_singleton() {
    let data = Value::Number(42.0, 0);
    let context = InterpreterContext::new(data);

    let expr = parse("aggregate($total + $this, 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_aggregate_max() {
    let data = Value::collection(vec![
        Value::Number(5.0, 0),
        Value::Number(2.0, 0),
        Value::Number(8.0, 0),
        Value::Number(1.0, 0),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("aggregate(iif($total < $this, $this, $total), 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(8.0, 0));
}

#[test]
fn test_aggregate_count() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("aggregate($total + 1, 0)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));
}

#[test]
fn test_not_true() {
    let context = InterpreterContext::new(Value::Boolean(true));
    let expr = parse("not()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_not_false() {
    let context = InterpreterContext::new(Value::Boolean(false));
    let expr = parse("not()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_not_empty() {
    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("not()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_not_chained() {
    let context = InterpreterContext::new(Value::Boolean(true));
    let expr = parse("not().not()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_has_value_primitive() {
    let context = InterpreterContext::new(Value::Number(42.0, 0));
    let expr = parse("hasValue()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_has_value_string() {
    let context = InterpreterContext::new(Value::String("hello".to_string()));
    let expr = parse("hasValue()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_has_value_null() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("hasValue()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_has_value_empty_collection() {
    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("hasValue()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_has_value_multi_item_collection() {
    let context = InterpreterContext::new(Value::collection(vec![
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
    ]));
    let expr = parse("hasValue()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_union_function_dedup() {
    let data = Value::collection(vec![Value::Number(1.0, 0), Value::Number(2.0, 0)]);
    let context = InterpreterContext::new(data);
    let expr = parse("union(2 | 3)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Value::Number(1.0, 0));
        assert_eq!(items[1], Value::Number(2.0, 0));
        assert_eq!(items[2], Value::Number(3.0, 0));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_union_function_empty() {
    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("union(1 | 2)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_children_of_object() {
    let obj = Value::object(HashMap::from([
        ("a".to_string(), Value::Number(1.0, 0)),
        ("b".to_string(), Value::Number(2.0, 0)),
    ]));
    let context = InterpreterContext::new(obj);
    let expr = parse("children()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.contains(&Value::Number(1.0, 0)));
        assert!(items.contains(&Value::Number(2.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_children_of_primitive() {
    let context = InterpreterContext::new(Value::Number(42.0, 0));
    let expr = parse("children()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_children_nested_returns_only_direct() {
    let obj = Value::object(HashMap::from([
        (
            "a".to_string(),
            Value::object(HashMap::from([("x".to_string(), Value::Number(10.0, 0))])),
        ),
        ("b".to_string(), Value::Number(2.0, 0)),
    ]));
    let context = InterpreterContext::new(obj);
    let expr = parse("children()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.contains(&Value::Number(2.0, 0)));
        assert!(items.iter().any(|v| matches!(v, Value::Object(_))));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_descendants_flat_object() {
    let obj = Value::object(HashMap::from([
        ("a".to_string(), Value::Number(1.0, 0)),
        ("b".to_string(), Value::Number(2.0, 0)),
    ]));
    let context = InterpreterContext::new(obj);
    let expr = parse("descendants()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2);
        assert!(items.contains(&Value::Number(1.0, 0)));
        assert!(items.contains(&Value::Number(2.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_descendants_nested_object() {
    let inner = Value::object(HashMap::from([
        ("x".to_string(), Value::Number(10.0, 0)),
        ("y".to_string(), Value::Number(20.0, 0)),
    ]));
    let obj = Value::object(HashMap::from([
        ("a".to_string(), inner.clone()),
        ("b".to_string(), Value::Number(3.0, 0)),
    ]));
    let context = InterpreterContext::new(obj);
    let expr = parse("descendants()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 4);
        assert!(items.contains(&inner));
        assert!(items.contains(&Value::Number(3.0, 0)));
        assert!(items.contains(&Value::Number(10.0, 0)));
        assert!(items.contains(&Value::Number(20.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_descendants_primitive() {
    let context = InterpreterContext::new(Value::Number(42.0, 0));
    let expr = parse("descendants()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_descendants_deeply_nested() {
    let level3 = Value::object(HashMap::from([(
        "three".to_string(),
        Value::Number(99.0, 0),
    )]));
    let level2 = Value::object(HashMap::from([("two".to_string(), level3.clone())]));
    let level1 = Value::object(HashMap::from([("one".to_string(), level2.clone())]));
    let context = InterpreterContext::new(level1);
    let expr = parse("descendants()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        assert!(items.contains(&level2));
        assert!(items.contains(&level3));
        assert!(items.contains(&Value::Number(99.0, 0)));
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_sort_numbers() {
    let data = Value::collection(vec![
        Value::Number(3.0, 0),
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![
            Value::Number(1.0, 0),
            Value::Number(2.0, 0),
            Value::Number(3.0, 0)
        ])
    );
}

#[test]
fn test_sort_strings() {
    let data = Value::collection(vec![
        Value::String("c".to_string()),
        Value::String("a".to_string()),
        Value::String("b".to_string()),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
        ])
    );
}

#[test]
fn test_sort_empty() {
    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("sort()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_sort_with_criteria() {
    let data = Value::collection(vec![
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Charlie".to_string())),
            ("age".to_string(), Value::Number(30.0, 0)),
        ])),
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Alice".to_string())),
            ("age".to_string(), Value::Number(25.0, 0)),
        ])),
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Bob".to_string())),
            ("age".to_string(), Value::Number(35.0, 0)),
        ])),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort(age)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        if let Value::Object(first) = &items[0] {
            assert_eq!(first.get("name"), Some(&Value::String("Alice".to_string())));
        } else {
            panic!("Expected object");
        }
        if let Value::Object(last) = &items[2] {
            assert_eq!(last.get("name"), Some(&Value::String("Bob".to_string())));
        } else {
            panic!("Expected object");
        }
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_coalesce_returns_first_nonempty() {
    let data = Value::collection(vec![
        Value::Null,
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("coalesce()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(1.0, 0));
}

#[test]
fn test_coalesce_all_empty() {
    let data = Value::collection(vec![Value::Null]);
    let context = InterpreterContext::new(data);
    let expr = parse("coalesce()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_coalesce_with_default() {
    let context = InterpreterContext::new(Value::Null);
    let expr = parse("coalesce(42)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::Number(42.0, 0));
}

#[test]
fn test_coalesce_skips_empty_collections() {
    let data = Value::collection(vec![
        Value::collection(vec![]),
        Value::String("found".to_string()),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("coalesce()").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::String("found".to_string()));
}

#[test]
fn test_repeat_all_allows_duplicates() {
    let obj = Value::object(HashMap::from([
        ("value".to_string(), Value::Number(1.0, 0)),
        (
            "item".to_string(),
            Value::collection(vec![
                Value::object(HashMap::from([
                    ("value".to_string(), Value::Number(2.0, 0)),
                    ("item".to_string(), Value::collection(vec![])),
                ])),
                Value::object(HashMap::from([
                    ("value".to_string(), Value::Number(2.0, 0)),
                    ("item".to_string(), Value::collection(vec![])),
                ])),
            ]),
        ),
    ]));
    let context = InterpreterContext::new(obj);

    let expr = parse("repeatAll(item)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 2, "repeatAll should include duplicate items");
    } else {
        panic!("Expected collection");
    }

    let expr = parse("repeat(item)").expect("parse failed");
    let (result, _) = interpret(&expr, context.clone()).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 1, "repeat should deduplicate items");
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_repeat_all_empty() {
    let context = InterpreterContext::new(Value::collection(vec![]));
    let expr = parse("repeatAll(item)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_sort_asc_numbers() {
    let data = Value::collection(vec![
        Value::Number(3.0, 0),
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort(asc)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![
            Value::Number(1.0, 0),
            Value::Number(2.0, 0),
            Value::Number(3.0, 0)
        ])
    );
}

#[test]
fn test_sort_desc_numbers() {
    let data = Value::collection(vec![
        Value::Number(3.0, 0),
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort(desc)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![
            Value::Number(3.0, 0),
            Value::Number(2.0, 0),
            Value::Number(1.0, 0)
        ])
    );
}

#[test]
fn test_sort_desc_strings() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("c".to_string()),
        Value::String("b".to_string()),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort(desc)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![
            Value::String("c".to_string()),
            Value::String("b".to_string()),
            Value::String("a".to_string()),
        ])
    );
}

#[test]
fn test_sort_with_criteria_desc() {
    let data = Value::collection(vec![
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Alice".to_string())),
            ("age".to_string(), Value::Number(25.0, 0)),
        ])),
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Charlie".to_string())),
            ("age".to_string(), Value::Number(30.0, 0)),
        ])),
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Bob".to_string())),
            ("age".to_string(), Value::Number(35.0, 0)),
        ])),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort(age, desc)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        if let Value::Object(first) = &items[0] {
            assert_eq!(first.get("name"), Some(&Value::String("Bob".to_string())));
        } else {
            panic!("Expected object");
        }
        if let Value::Object(last) = &items[2] {
            assert_eq!(last.get("name"), Some(&Value::String("Alice".to_string())));
        } else {
            panic!("Expected object");
        }
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_sort_with_criteria_asc() {
    let data = Value::collection(vec![
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Charlie".to_string())),
            ("age".to_string(), Value::Number(30.0, 0)),
        ])),
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Alice".to_string())),
            ("age".to_string(), Value::Number(25.0, 0)),
        ])),
        Value::object(HashMap::from([
            ("name".to_string(), Value::String("Bob".to_string())),
            ("age".to_string(), Value::Number(35.0, 0)),
        ])),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort(age, asc)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(items.len(), 3);
        if let Value::Object(first) = &items[0] {
            assert_eq!(first.get("name"), Some(&Value::String("Alice".to_string())));
        } else {
            panic!("Expected object");
        }
        if let Value::Object(last) = &items[2] {
            assert_eq!(last.get("name"), Some(&Value::String("Bob".to_string())));
        } else {
            panic!("Expected object");
        }
    } else {
        panic!("Expected collection");
    }
}

#[test]
fn test_sort_with_this_desc() {
    let data = Value::collection(vec![
        Value::Number(3.0, 0),
        Value::Number(1.0, 0),
        Value::Number(2.0, 0),
    ]);
    let context = InterpreterContext::new(data);
    let expr = parse("sort($this, desc)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    assert_eq!(
        result,
        Value::collection(vec![
            Value::Number(3.0, 0),
            Value::Number(2.0, 0),
            Value::Number(1.0, 0)
        ])
    );
}

#[test]
fn test_repeat_all_flat_values() {
    let data = Value::collection(vec![
        Value::object(HashMap::from([(
            "item".to_string(),
            Value::collection(vec![Value::Number(1.0, 0)]),
        )])),
        Value::object(HashMap::from([(
            "item".to_string(),
            Value::collection(vec![Value::Number(1.0, 0)]),
        )])),
    ]);
    let context = InterpreterContext::new(data);

    let expr = parse("repeatAll(item)").expect("parse failed");
    let (result, _) = interpret(&expr, context).expect("interpret failed");
    if let Value::Collection(ref items) = result {
        assert_eq!(
            items.len(),
            2,
            "repeatAll should keep both identical Number(1.0) results"
        );
    } else {
        panic!("Expected collection");
    }
}
