#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::parse;
use std::time::{Duration, Instant};

fn must_not_hang(input: &str) {
    let start = Instant::now();
    let _ = parse(input);
    assert!(
        start.elapsed() < Duration::from_secs(2),
        "parse({input:?}) took over 2 seconds — possible infinite loop"
    );
}

fn must_fail(input: &str) {
    let start = Instant::now();
    let result = parse(input);
    assert!(
        start.elapsed() < Duration::from_secs(2),
        "parse({input:?}) took over 2 seconds — possible infinite loop"
    );
    assert!(
        result.is_err(),
        "parse({input:?}) should fail but got: {result:?}"
    );
}

#[test]
fn test_empty_input() {
    must_fail("");
}

#[test]
fn test_whitespace_only() {
    must_fail("   ");
}

#[test]
fn test_tab_only() {
    must_fail("\t\n");
}

#[test]
fn test_unmatched_open_paren() {
    must_fail("(");
}

#[test]
fn test_unmatched_close_paren() {
    must_fail(")");
}

#[test]
fn test_unmatched_open_paren_with_expr() {
    must_fail("(1 + 2");
}

#[test]
fn test_trailing_close_paren() {
    must_fail("1 + 2)");
}

#[test]
fn test_many_unmatched_open_parens() {
    must_fail("((((((");
}

#[test]
fn test_many_unmatched_close_parens() {
    must_fail("))))))");
}

#[test]
fn test_mismatched_paren_count() {
    must_fail("((1 + 2)");
}

#[test]
fn test_extra_close_parens() {
    must_fail("(1 + 2))");
}

#[test]
fn test_empty_parens() {
    must_fail("()");
}

#[test]
fn test_nested_empty_parens() {
    must_fail("(())");
}

#[test]
fn test_unmatched_open_bracket() {
    must_fail("a[");
}

#[test]
fn test_unmatched_close_bracket() {
    must_fail("]");
}

#[test]
fn test_unmatched_bracket_with_expr() {
    must_fail("a[0");
}

#[test]
fn test_trailing_close_bracket() {
    must_fail("a]");
}

#[test]
fn test_empty_brackets() {
    must_fail("a[]");
}

#[test]
fn test_bracket_without_base() {
    must_fail("[0]");
}

#[test]
fn test_paren_bracket_mismatch() {
    must_fail("(1]");
}

#[test]
fn test_bracket_paren_mismatch() {
    must_fail("a[0)");
}

#[test]
fn test_interleaved_delimiters() {
    must_fail("([)]");
}

#[test]
fn test_trailing_plus() {
    must_fail("1 +");
}

#[test]
fn test_trailing_minus() {
    must_fail("1 -");
}

#[test]
fn test_trailing_multiply() {
    must_fail("1 *");
}

#[test]
fn test_trailing_divide() {
    must_fail("1 /");
}

#[test]
fn test_trailing_and() {
    must_fail("true and");
}

#[test]
fn test_trailing_or() {
    must_fail("true or");
}

#[test]
fn test_trailing_xor() {
    must_fail("true xor");
}

#[test]
fn test_trailing_implies() {
    must_fail("true implies");
}

#[test]
fn test_trailing_equals() {
    must_fail("1 =");
}

#[test]
fn test_trailing_not_equals() {
    must_fail("1 !=");
}

#[test]
fn test_trailing_less_than() {
    must_fail("1 <");
}

#[test]
fn test_trailing_greater_than() {
    must_fail("1 >");
}

#[test]
fn test_trailing_union() {
    must_fail("1 |");
}

#[test]
fn test_trailing_ampersand() {
    must_fail("'a' &");
}

#[test]
fn test_trailing_in() {
    must_fail("1 in");
}

#[test]
fn test_trailing_contains() {
    must_fail("1 contains");
}

#[test]
fn test_trailing_is() {
    must_fail("x is");
}

#[test]
fn test_trailing_as() {
    must_fail("x as");
}

#[test]
fn test_trailing_div() {
    must_fail("1 div");
}

#[test]
fn test_trailing_mod() {
    must_fail("1 mod");
}

#[test]
fn test_leading_multiply() {
    must_fail("* 1");
}

#[test]
fn test_leading_divide() {
    must_fail("/ 1");
}

#[test]
fn test_leading_equals() {
    must_fail("= 1");
}

#[test]
fn test_leading_pipe() {
    must_fail("| 1");
}

#[test]
fn test_leading_ampersand() {
    must_fail("& 1");
}

#[test]
fn test_leading_less_than() {
    must_fail("< 1");
}

#[test]
fn test_leading_greater_than() {
    must_fail("> 1");
}

#[test]
fn test_double_multiply() {
    must_fail("1 * * 2");
}

#[test]
fn test_double_divide() {
    must_fail("1 / / 2");
}

#[test]
fn test_double_equals() {
    must_fail("1 = = 2");
}

#[test]
fn test_double_and() {
    must_fail("true and and false");
}

#[test]
fn test_double_or() {
    must_fail("true or or false");
}

#[test]
fn test_multiply_divide() {
    must_fail("1 * / 2");
}

#[test]
fn test_just_plus() {
    must_fail("+");
}

#[test]
fn test_just_minus() {
    must_fail("-");
}

#[test]
fn test_just_multiply() {
    must_fail("*");
}

#[test]
fn test_just_dot() {
    must_fail(".");
}

#[test]
fn test_just_pipe() {
    must_fail("|");
}

#[test]
fn test_two_numbers() {
    must_fail("1 2");
}

#[test]
fn test_two_strings() {
    must_fail("'a' 'b'");
}

#[test]
fn test_two_booleans() {
    must_fail("true false");
}

#[test]
fn test_number_then_identifier() {
    must_fail("42 x");
}

#[test]
fn test_unclosed_function_call() {
    must_fail("where(");
}

#[test]
fn test_function_unclosed_with_arg() {
    must_fail("where(x");
}

#[test]
fn test_function_trailing_comma() {
    must_fail("iif(true,");
}

#[test]
fn test_function_double_comma() {
    must_fail("iif(true,,false)");
}

#[test]
fn test_unclosed_string() {
    must_fail("'hello");
}

#[test]
fn test_unclosed_string_in_expr() {
    must_fail("x = 'hello");
}

#[test]
fn test_trailing_dot() {
    must_fail("a.");
}

#[test]
fn test_double_dot() {
    must_fail("a..b");
}

#[test]
fn test_leading_dot() {
    must_fail(".a");
}

#[test]
fn test_is_without_type() {
    must_fail("x is +");
}

#[test]
fn test_as_without_type() {
    must_fail("x as *");
}

#[test]
fn test_at_sign_alone() {
    must_fail("@");
}

#[test]
fn test_hash_sign() {
    must_fail("#");
}

#[test]
fn test_dollar_sign() {
    must_fail("$");
}

#[test]
fn test_semicolon() {
    must_fail(";");
}

#[test]
fn test_backslash() {
    must_fail("\\");
}

#[test]
fn test_curly_brace_open() {
    must_fail("{");
}

#[test]
fn test_thousand_open_parens() {
    let input = "(".repeat(1000);
    must_not_hang(&input);
}

#[test]
fn test_thousand_close_parens() {
    let input = ")".repeat(1000);
    must_not_hang(&input);
}

#[test]
fn test_thousand_open_brackets() {
    let input = "[".repeat(1000);
    must_not_hang(&input);
}

#[test]
fn test_thousand_close_brackets() {
    let input = "]".repeat(1000);
    must_not_hang(&input);
}

#[test]
fn test_thousand_operators() {
    let input = "+ ".repeat(1000);
    must_not_hang(&input);
}

#[test]
fn test_thousand_dots() {
    let input = ".".repeat(1000);
    must_not_hang(&input);
}

#[test]
fn test_alternating_parens_brackets() {
    let input = "([".repeat(500);
    must_not_hang(&input);
}

#[test]
fn test_many_trailing_operators() {
    let input = format!("1{}", " + ".repeat(1000));
    must_not_hang(&input);
}

#[test]
fn test_valid_unary_plus() {
    assert!(parse("+1").is_ok());
}

#[test]
fn test_valid_unary_minus() {
    assert!(parse("-1").is_ok());
}

#[test]
fn test_valid_double_negation() {
    assert!(parse("--1").is_ok());
}

#[test]
fn test_valid_parenthesized() {
    assert!(parse("(1)").is_ok());
}

#[test]
fn test_valid_nested_parens() {
    assert!(parse("((1))").is_ok());
}

#[test]
fn test_valid_indexer() {
    assert!(parse("a[0]").is_ok());
}

#[test]
fn test_valid_chained_indexer() {
    assert!(parse("a[0][1]").is_ok());
}

#[test]
fn test_valid_paren_then_indexer() {
    assert!(parse("(a)[0]").is_ok());
}

#[test]
fn test_valid_negated_paren() {
    assert!(parse("-(1 + 2)").is_ok());
}

#[test]
fn test_valid_paren_invocation() {
    assert!(parse("(a).where(true)").is_ok());
}

#[test]
fn test_unclosed_backtick() {
    must_fail("`hello");
}

#[test]
fn test_empty_backtick() {
    must_fail("``");
}

#[test]
fn test_percent_alone() {
    must_fail("%");
}

#[test]
fn test_double_at_sign() {
    must_fail("@@2024");
}

#[test]
fn test_comma_separated_expressions() {
    must_fail("1, 2");
}

#[test]
fn test_curly_brace_with_content() {
    must_fail("{1}");
}

#[test]
fn test_trailing_comma_in_function() {
    must_fail("foo(1,)");
}

#[test]
fn test_nested_unclosed_function() {
    must_fail("foo(bar(");
}

#[test]
fn test_is_with_number_type() {
    must_fail("x is 5");
}

#[test]
fn test_multiple_dots_in_path() {
    must_fail("a...b");
}
