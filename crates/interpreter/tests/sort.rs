#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use interpreter::{CollectingTraceHandler, InterpreterContext, Value, interpret};
use parser::{Expression, SortDirection, parse};
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn no_key_numeric_ascending() {
    let data = Value::collection(vec![
        Value::number(3.0, 0),
        Value::number(1.0, 0),
        Value::number(2.0, 0),
    ]);
    let expr = parse("$this.sort()").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(1.0, 0),
            Value::number(2.0, 0),
            Value::number(3.0, 0),
        ]
    );
}

#[test]
fn no_key_string_ascending() {
    let data = Value::collection(vec![
        Value::String("c".to_string()),
        Value::String("a".to_string()),
        Value::String("b".to_string()),
    ]);
    let expr = parse("$this.sort()").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
        ]
    );
}

#[test]
fn desc_suffix_sorts_numbers_descending() {
    let data = Value::collection(vec![
        Value::number(1.0, 0),
        Value::number(3.0, 0),
        Value::number(2.0, 0),
    ]);
    let expr = parse("$this.sort($this desc)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(3.0, 0),
            Value::number(2.0, 0),
            Value::number(1.0, 0),
        ]
    );
}

#[test]
fn desc_suffix_sorts_strings_descending() {
    let data = Value::collection(vec![
        Value::String("a".to_string()),
        Value::String("c".to_string()),
        Value::String("b".to_string()),
    ]);
    let expr = parse("$this.sort($this desc)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::String("c".to_string()),
            Value::String("b".to_string()),
            Value::String("a".to_string()),
        ]
    );
}

#[test]
fn single_key_this_ascending() {
    let data = Value::collection(vec![
        Value::number(3.0, 0),
        Value::number(1.0, 0),
        Value::number(2.0, 0),
    ]);
    let expr = parse("$this.sort($this)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(1.0, 0),
            Value::number(2.0, 0),
            Value::number(3.0, 0),
        ]
    );
}

#[test]
fn single_key_this_negated_descending() {
    let data = Value::collection(vec![
        Value::number(1.0, 0),
        Value::number(3.0, 0),
        Value::number(2.0, 0),
    ]);
    let expr = parse("$this.sort(-$this)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(3.0, 0),
            Value::number(2.0, 0),
            Value::number(1.0, 0),
        ]
    );
}

#[test]
fn single_key_field_ascending() {
    let mut o1 = HashMap::new();
    o1.insert("v".to_string(), Value::number(30.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("v".to_string(), Value::number(10.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("v".to_string(), Value::number(20.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(v).v").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(10.0, 0),
            Value::number(20.0, 0),
            Value::number(30.0, 0),
        ]
    );
}

#[test]
fn single_key_field_negated_descending() {
    let mut o1 = HashMap::new();
    o1.insert("v".to_string(), Value::number(10.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("v".to_string(), Value::number(30.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("v".to_string(), Value::number(20.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(-v).v").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(30.0, 0),
            Value::number(20.0, 0),
            Value::number(10.0, 0),
        ]
    );
}

#[test]
fn negated_key_with_desc_suffix_cancels_to_ascending() {
    let data = Value::collection(vec![
        Value::number(3.0, 0),
        Value::number(1.0, 0),
        Value::number(2.0, 0),
    ]);
    let expr = parse("$this.sort(-$this desc)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(1.0, 0),
            Value::number(2.0, 0),
            Value::number(3.0, 0),
        ]
    );
}

#[test]
fn multi_key_both_ascending() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(20.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(1.0, 0));
    o2.insert("b".to_string(), Value::number(10.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(5.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(a, b).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(10.0, 0),
            Value::number(20.0, 0),
            Value::number(5.0, 0),
        ]
    );
}

#[test]
fn multi_key_both_descending_via_negation() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(10.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(1.0, 0));
    o2.insert("b".to_string(), Value::number(20.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(5.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(-a, -b).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(5.0, 0),
            Value::number(20.0, 0),
            Value::number(10.0, 0),
        ]
    );
}

#[test]
fn multi_key_mixed_primary_desc_secondary_asc() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(20.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(2.0, 0));
    o2.insert("b".to_string(), Value::number(10.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(20.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(-a, b).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(10.0, 0),
            Value::number(20.0, 0),
            Value::number(20.0, 0),
        ]
    );
}

#[test]
fn multi_key_mixed_primary_asc_secondary_desc() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(10.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(1.0, 0));
    o2.insert("b".to_string(), Value::number(20.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(5.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(a, -b).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(20.0, 0),
            Value::number(10.0, 0),
            Value::number(5.0, 0),
        ]
    );
}

#[test]
fn multi_key_both_desc_suffix() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(10.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(1.0, 0));
    o2.insert("b".to_string(), Value::number(20.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(5.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(a desc, b desc).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(5.0, 0),
            Value::number(20.0, 0),
            Value::number(10.0, 0),
        ]
    );
}

#[test]
fn multi_key_negation_combined_with_desc_suffix() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(10.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(1.0, 0));
    o2.insert("b".to_string(), Value::number(20.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(5.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(a desc, -b desc).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(5.0, 0),
            Value::number(10.0, 0),
            Value::number(20.0, 0),
        ]
    );
}

#[test]
fn empty_collection_returns_empty() {
    let expr = parse("$this.sort()").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(Value::collection(vec![])))
        .expect("interpret failed");
    assert!(result.to_vec().is_empty());
}

#[test]
fn empty_collection_with_key_returns_empty() {
    let expr = parse("$this.sort(-$this)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(Value::collection(vec![])))
        .expect("interpret failed");
    assert!(result.to_vec().is_empty());
}

#[test]
fn missing_key_sorts_first_in_ascending() {
    let mut o1 = HashMap::new();
    o1.insert("family".to_string(), Value::String("Chalmers".to_string()));
    o1.insert("tag".to_string(), Value::String("a".to_string()));
    let mut o2 = HashMap::new();
    o2.insert("tag".to_string(), Value::String("b".to_string()));
    let mut o3 = HashMap::new();
    o3.insert("family".to_string(), Value::String("Windsor".to_string()));
    o3.insert("tag".to_string(), Value::String("c".to_string()));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(family).tag").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::String("b".to_string()),
            Value::String("a".to_string()),
            Value::String("c".to_string()),
        ]
    );
}

#[test]
fn missing_key_sorts_last_in_descending() {
    let mut o1 = HashMap::new();
    o1.insert("family".to_string(), Value::String("Chalmers".to_string()));
    o1.insert("tag".to_string(), Value::String("a".to_string()));
    let mut o2 = HashMap::new();
    o2.insert("tag".to_string(), Value::String("b".to_string()));
    let mut o3 = HashMap::new();
    o3.insert("family".to_string(), Value::String("Windsor".to_string()));
    o3.insert("tag".to_string(), Value::String("c".to_string()));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(family desc).tag").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::String("c".to_string()),
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]
    );
}

#[test]
fn primary_ties_broken_by_secondary() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(3.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(1.0, 0));
    o2.insert("b".to_string(), Value::number(1.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(1.0, 0));
    o3.insert("b".to_string(), Value::number(2.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(a, b).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(1.0, 0),
            Value::number(2.0, 0),
            Value::number(3.0, 0),
        ]
    );
}

#[test]
fn secondary_selector_is_not_evaluated_without_primary_ties() {
    let expr = parse("(1 | 2).sort($this, 1 / 0)").expect("parse failed");
    let (result, _) =
        interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");

    assert_eq!(
        result.to_vec(),
        vec![Value::number(1.0, 0), Value::number(2.0, 0)]
    );
}

#[test]
fn tertiary_selector_is_not_evaluated_without_secondary_ties() {
    let mut first = HashMap::new();
    first.insert("a".to_string(), Value::number(1.0, 0));
    first.insert("b".to_string(), Value::number(2.0, 0));
    let mut second = HashMap::new();
    second.insert("a".to_string(), Value::number(1.0, 0));
    second.insert("b".to_string(), Value::number(1.0, 0));
    let data = Value::collection(vec![Value::object(first), Value::object(second)]);
    let expr = parse("$this.sort(a, b, 1 / 0).b").expect("parse failed");

    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");

    assert_eq!(
        result.to_vec(),
        vec![Value::number(1.0, 0), Value::number(2.0, 0)]
    );
}

#[test]
fn secondary_selector_is_evaluated_only_for_primary_ties() {
    let mut first = HashMap::new();
    first.insert("a".to_string(), Value::number(1.0, 0));
    first.insert("b".to_string(), Value::number(3.0, 0));
    let mut second = HashMap::new();
    second.insert("a".to_string(), Value::number(1.0, 0));
    second.insert("b".to_string(), Value::number(1.0, 0));
    let mut third = HashMap::new();
    third.insert("a".to_string(), Value::number(2.0, 0));
    third.insert("b".to_string(), Value::number(2.0, 0));
    let handler = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext::new(Value::collection(vec![
        Value::object(first),
        Value::object(second),
        Value::object(third),
    ]))
    .with_trace_handler(handler.clone());
    let expr = parse("$this.sort(a, b.trace('secondary')).b").expect("parse failed");

    let (result, _) = interpret(&expr, context).expect("interpret failed");

    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(1.0, 0),
            Value::number(3.0, 0),
            Value::number(2.0, 0),
        ]
    );
    let events = handler.events();
    assert_eq!(events.len(), 2);
    let traced: Vec<&Value> = events.iter().map(|e| &e.value).collect();
    assert!(traced.contains(&&Value::number(1.0, 0)));
    assert!(traced.contains(&&Value::number(3.0, 0)));
}

#[test]
fn single_key_asc_suffix_is_explicit_ascending() {
    let data = Value::collection(vec![
        Value::number(3.0, 0),
        Value::number(1.0, 0),
        Value::number(2.0, 0),
    ]);
    let expr = parse("$this.sort($this asc)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(1.0, 0),
            Value::number(2.0, 0),
            Value::number(3.0, 0),
        ]
    );
}

#[test]
fn single_key_desc_suffix_on_field() {
    let mut o1 = HashMap::new();
    o1.insert("v".to_string(), Value::number(10.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("v".to_string(), Value::number(30.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("v".to_string(), Value::number(20.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(v desc).v").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(30.0, 0),
            Value::number(20.0, 0),
            Value::number(10.0, 0),
        ]
    );
}

#[test]
fn multi_key_default_asc_plus_explicit_desc() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(10.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(1.0, 0));
    o2.insert("b".to_string(), Value::number(20.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(5.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(a, b desc).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(20.0, 0),
            Value::number(10.0, 0),
            Value::number(5.0, 0),
        ]
    );
}

#[test]
fn multi_key_explicit_desc_and_default_asc() {
    let mut o1 = HashMap::new();
    o1.insert("a".to_string(), Value::number(1.0, 0));
    o1.insert("b".to_string(), Value::number(20.0, 0));
    let mut o2 = HashMap::new();
    o2.insert("a".to_string(), Value::number(2.0, 0));
    o2.insert("b".to_string(), Value::number(10.0, 0));
    let mut o3 = HashMap::new();
    o3.insert("a".to_string(), Value::number(2.0, 0));
    o3.insert("b".to_string(), Value::number(20.0, 0));
    let data = Value::collection(vec![
        Value::object(o1),
        Value::object(o2),
        Value::object(o3),
    ]);
    let expr = parse("$this.sort(a desc, b).b").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(10.0, 0),
            Value::number(20.0, 0),
            Value::number(20.0, 0),
        ]
    );
}

#[test]
fn negated_key_with_asc_suffix_is_descending() {
    let data = Value::collection(vec![
        Value::number(1.0, 0),
        Value::number(3.0, 0),
        Value::number(2.0, 0),
    ]);
    let expr = parse("$this.sort(-$this asc)").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(3.0, 0),
            Value::number(2.0, 0),
            Value::number(1.0, 0),
        ]
    );
}

#[test]
fn spec_example_family_desc_given_first() {
    let mut n1 = HashMap::new();
    n1.insert("family".to_string(), Value::String("Chalmers".to_string()));
    n1.insert(
        "given".to_string(),
        Value::collection(vec![
            Value::String("Peter".to_string()),
            Value::String("James".to_string()),
        ]),
    );
    let mut n2 = HashMap::new();
    n2.insert("family".to_string(), Value::String("Windsor".to_string()));
    n2.insert(
        "given".to_string(),
        Value::collection(vec![
            Value::String("Peter".to_string()),
            Value::String("James".to_string()),
        ]),
    );
    let mut patient = HashMap::new();
    patient.insert(
        "resourceType".to_string(),
        Value::String("Patient".to_string()),
    );
    patient.insert(
        "name".to_string(),
        Value::collection(vec![Value::object(n1), Value::object(n2)]),
    );
    let data = Value::object(patient);
    let expr = parse("Patient.name.sort(family desc, given.first()).family").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::String("Windsor".to_string()),
            Value::String("Chalmers".to_string()),
        ]
    );
}

#[test]
fn negated_string_key_is_error() {
    let expr = parse("('a' | 'b').sort(-$this)").expect("parse failed");
    assert!(interpret(&expr, InterpreterContext::new(Value::Null)).is_err());
}

#[test]
fn multi_item_key_is_error() {
    let data = Value::collection(vec![
        Value::object(HashMap::from([(
            "g".to_string(),
            Value::collection(vec![Value::number(1.0, 0), Value::number(2.0, 0)]),
        )])),
        Value::object(HashMap::from([(
            "g".to_string(),
            Value::collection(vec![Value::number(3.0, 0), Value::number(4.0, 0)]),
        )])),
    ]);
    let expr = parse("$this.sort(g)").expect("parse failed");
    assert!(interpret(&expr, InterpreterContext::new(data)).is_err());
}

#[test]
fn multi_item_secondary_key_is_error_on_ties() {
    let data = Value::collection(vec![
        Value::object(HashMap::from([
            ("a".to_string(), Value::number(1.0, 0)),
            (
                "g".to_string(),
                Value::collection(vec![Value::number(1.0, 0), Value::number(2.0, 0)]),
            ),
        ])),
        Value::object(HashMap::from([
            ("a".to_string(), Value::number(1.0, 0)),
            (
                "g".to_string(),
                Value::collection(vec![Value::number(3.0, 0), Value::number(4.0, 0)]),
            ),
        ])),
    ]);
    let expr = parse("$this.sort(a, g)").expect("parse failed");
    assert!(interpret(&expr, InterpreterContext::new(data)).is_err());
}

#[test]
fn incompatible_key_types_is_error() {
    let expr = parse("(1 | true | 2).sort($this)").expect("parse failed");
    assert!(interpret(&expr, InterpreterContext::new(Value::Null)).is_err());
}

#[test]
fn incompatible_item_types_is_error_without_key() {
    let expr = parse("(1 | 'a' | 2).sort()").expect("parse failed");
    assert!(interpret(&expr, InterpreterContext::new(Value::Null)).is_err());
}

#[test]
fn member_named_asc_or_desc_is_a_key_selector() {
    let data = Value::collection(vec![
        Value::object(HashMap::from([
            ("asc".to_string(), Value::number(2.0, 0)),
            ("desc".to_string(), Value::String("b".to_string())),
            ("id".to_string(), Value::String("second".to_string())),
        ])),
        Value::object(HashMap::from([
            ("asc".to_string(), Value::number(1.0, 0)),
            ("desc".to_string(), Value::String("a".to_string())),
            ("id".to_string(), Value::String("first".to_string())),
        ])),
    ]);
    let first_second = vec![
        Value::String("first".to_string()),
        Value::String("second".to_string()),
    ];
    let second_first = vec![
        Value::String("second".to_string()),
        Value::String("first".to_string()),
    ];
    for (expr_str, expected) in [
        ("$this.sort(asc).id", &first_second),
        ("$this.sort(asc asc).id", &first_second),
        ("$this.sort(asc desc).id", &second_first),
        ("$this.sort(desc).id", &first_second),
        ("$this.sort(desc desc).id", &second_first),
        ("$this.sort(desc, asc).id", &first_second),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(data.clone())).expect("interpret failed");
        assert_eq!(result.to_vec(), *expected, "{expr_str}");
    }
}

#[test]
fn missing_member_named_desc_is_an_empty_key() {
    let expr = parse("(2 | 1).sort(desc)").expect("parse failed");
    let (result, _) =
        interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![Value::number(2.0, 0), Value::number(1.0, 0)]
    );
}

#[test]
fn number_and_dimensionless_quantity_keys_are_comparable() {
    let one = Value::quantity(1.0, 0, "1".to_string(), None);
    for expr_str in ["(2 | 1 '1' | 3).sort()", "(2 | 1 '1' | 3).sort($this)"] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(
            result.to_vec(),
            vec![one.clone(), Value::number(2.0, 0), Value::number(3.0, 0)],
            "{expr_str}"
        );
    }
    let expr = parse("(2 | 1 '1' | 3).sort($this desc)").expect("parse failed");
    let (result, _) =
        interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![Value::number(3.0, 0), Value::number(2.0, 0), one]
    );
}

#[test]
fn number_and_dimensioned_quantity_is_error() {
    let expr = parse("(2 | 1 'mg').sort()").expect("parse failed");
    assert!(interpret(&expr, InterpreterContext::new(Value::Null)).is_err());
    let expr = parse("(2 | 1 'mg').sort($this)").expect("parse failed");
    assert!(interpret(&expr, InterpreterContext::new(Value::Null)).is_err());
}

#[test]
fn numbers_differing_in_the_last_digit_sort_exactly() {
    let data = Value::collection(vec![
        Value::number(1.00000002, 8),
        Value::number(1.0, 0),
        Value::number(1.00000001, 8),
    ]);
    let expr = parse("$this.sort()").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(1.0, 0),
            Value::number(1.00000001, 8),
            Value::number(1.00000002, 8),
        ]
    );
}

#[test]
fn numbers_beyond_max_precision_collapse_at_the_boundary_and_sort_stably() {
    let data = Value::collection(vec![
        Value::from_f64(3e-16),
        Value::from_f64(1.0),
        Value::from_f64(0.0),
        Value::from_f64(1.5e-16),
        Value::from_f64(1.00000001),
    ]);
    let expr = parse("$this.sort()").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        vec![
            Value::number(0.0, 0),
            Value::number(0.0, 0),
            Value::number(0.0, 0),
            Value::number(1.0, 0),
            Value::number(1.00000001, 8),
        ]
    );
}

#[test]
fn sorts_large_collection() {
    let data = Value::collection(
        (0..1000)
            .rev()
            .map(|i| Value::number(f64::from(i), 0))
            .collect(),
    );
    let expr = parse("$this.sort()").expect("parse failed");
    let (result, _) = interpret(&expr, InterpreterContext::new(data)).expect("interpret failed");
    assert_eq!(
        result.to_vec(),
        (0..1000)
            .map(|i| Value::number(f64::from(i), 0))
            .collect::<Vec<_>>()
    );
}

#[test]
fn ordered_by_outside_sort_is_error() {
    let expr = Expression::OrderedBy(
        Box::new(parse("1").expect("parse failed")),
        SortDirection::Asc,
    );
    assert!(interpret(&expr, InterpreterContext::new(Value::Null)).is_err());
}
