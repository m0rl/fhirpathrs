#![allow(clippy::expect_used)]
use parser::parse;

const DEPTH: usize = 100_000;

#[test]
fn test_deep_additive_chain_drops() {
    let input = format!("1{}", "+1".repeat(DEPTH));
    drop(parse(&input).expect("additive chain should parse"));
}

#[test]
fn test_deep_infix_operator_chains_drop() {
    for link in [
        " * 1",
        " | 1",
        " < 1",
        " = 1",
        " in 1",
        " and 1",
        " or 1",
        " implies 1",
        " is Integer",
    ] {
        let input = format!("1{}", link.repeat(DEPTH));
        drop(parse(&input).expect("infix operator chain should parse"));
    }
}

#[test]
fn test_deep_member_chain_drops() {
    let input = format!("a{}", ".a".repeat(DEPTH));
    drop(parse(&input).expect("member chain should parse"));
}

#[test]
fn test_deep_nested_function_arguments_drop() {
    let input = format!("{}1{}", "f(".repeat(DEPTH), ")".repeat(DEPTH));
    drop(parse(&input).expect("nested function arguments should parse"));
}

#[test]
fn test_deep_nested_method_arguments_drop() {
    let input = format!("{}1{}", "a.f(".repeat(DEPTH), ")".repeat(DEPTH));
    drop(parse(&input).expect("nested method arguments should parse"));
}

#[test]
fn test_deep_nested_parentheses_drop() {
    let input = format!("{}1{}", "(".repeat(DEPTH), ")".repeat(DEPTH));
    drop(parse(&input).expect("nested parentheses should parse"));
}

#[test]
fn test_deep_unary_minus_drops() {
    let input = format!("{}1", "-".repeat(DEPTH));
    drop(parse(&input).expect("unary minus chain should parse"));
}

#[test]
fn test_deep_chained_indexers_drop() {
    let input = format!("a{}", "[0]".repeat(DEPTH));
    drop(parse(&input).expect("chained indexers should parse"));
}

#[test]
fn test_deep_nested_indexers_drop() {
    let input = format!("{}0{}", "a[".repeat(DEPTH), "]".repeat(DEPTH));
    drop(parse(&input).expect("nested indexers should parse"));
}

#[test]
fn test_deep_nested_sort_criteria_drop() {
    let input = format!("{}1{}", "a.sort(".repeat(DEPTH), " desc)".repeat(DEPTH));
    drop(parse(&input).expect("nested sort criteria should parse"));
}

#[test]
fn test_deep_mixed_wrappers_drop() {
    let wrappers = [
        ("(", ")"),
        ("-", ""),
        ("f(", ")"),
        ("a[", "]"),
        ("", ".a"),
        ("", "+1"),
        ("a.sort(", " desc)"),
    ];
    let prefix: String = (0..DEPTH)
        .rev()
        .map(|level| wrappers[level % wrappers.len()].0)
        .collect();
    let suffix: String = (0..DEPTH)
        .map(|level| wrappers[level % wrappers.len()].1)
        .collect();
    drop(parse(&format!("{prefix}1{suffix}")).expect("mixed wrappers should parse"));
}

#[test]
fn test_deep_chain_with_trailing_operator_is_rejected() {
    let input = format!("1{}+", "+1".repeat(DEPTH));
    assert!(parse(&input).is_err());
}

#[test]
fn test_deep_chain_with_trailing_input_is_rejected() {
    let input = format!("a{}.", ".a".repeat(DEPTH));
    assert!(parse(&input).is_err());
}
