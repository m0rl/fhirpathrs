#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{Expression, Invocation, Literal, Term, parse};

#[test]
fn test_literal_expressions() {
    assert_eq!(
        parse("{}"),
        Ok(Expression::Term(Term::Literal(Literal::Null)))
    );
    assert_eq!(
        parse("'test'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "test".to_string()
        ))))
    );
    assert_eq!(
        parse("@2024-06-14"),
        Ok(Expression::Term(Term::Literal(Literal::Date(
            "2024-06-14".to_string()
        ))))
    );
    assert_eq!(
        parse("@2024-06-14T15:30:00"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-06-14T15:30:00".to_string()
        ))))
    );
    assert_eq!(
        parse("@T15:30:00"),
        Ok(Expression::Term(Term::Literal(Literal::Time(
            "15:30:00".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_backslash() {
    assert_eq!(
        parse(r"'a\\b'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a\\b".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_newline() {
    assert_eq!(
        parse(r"'hello\nworld'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "hello\nworld".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_tab() {
    assert_eq!(
        parse(r"'col1\tcol2'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "col1\tcol2".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_carriage_return() {
    assert_eq!(
        parse(r"'a\rb'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a\rb".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_form_feed() {
    assert_eq!(
        parse(r"'a\fb'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a\x0Cb".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_double_quote() {
    assert_eq!(
        parse(r#"'a\"b'"#),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a\"b".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_backtick() {
    assert_eq!(
        parse(r"'a\`b'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a`b".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_forward_slash() {
    assert_eq!(
        parse(r"'a\/b'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a/b".to_string()
        ))))
    );
}

#[test]
fn test_string_escape_single_quote() {
    assert_eq!(
        parse(r"'it\'s'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "it's".to_string()
        ))))
    );
}

#[test]
fn test_string_unicode_escape() {
    assert_eq!(
        parse(r"'\u0041'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "A".to_string()
        ))))
    );
}

#[test]
fn test_string_unicode_escape_heart() {
    assert_eq!(
        parse(r"'\u2764'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "\u{2764}".to_string()
        ))))
    );
}

#[test]
fn test_string_unrecognized_escape_passthrough() {
    assert_eq!(
        parse(r"'a\xb'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            r"a\xb".to_string()
        ))))
    );
}

#[test]
fn test_string_multiple_escapes() {
    assert_eq!(
        parse(r"'a\nb\tc'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a\nb\tc".to_string()
        ))))
    );
}

#[test]
fn test_string_consecutive_backslashes() {
    assert_eq!(
        parse(r"'a\\\\b'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "a\\\\b".to_string()
        ))))
    );
}

#[test]
fn test_backtick_identifier() {
    assert_eq!(
        parse("`div`"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "div".to_string()
        ))))
    );
}

#[test]
fn test_backtick_identifier_contains() {
    assert_eq!(
        parse("`contains`"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "contains".to_string()
        ))))
    );
}

#[test]
fn test_backtick_identifier_in_path() {
    assert_eq!(
        parse("x.`div`"),
        Ok(Expression::Invocation(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "x".to_string()
            )))),
            Invocation::Member("div".to_string())
        ))
    );
}

#[test]
fn test_backtick_identifier_with_spaces() {
    assert_eq!(
        parse("`has spaces`"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "has spaces".to_string()
        ))))
    );
}

#[test]
fn test_datetime_dot_not_consumed_before_method() {
    let expr = parse("@2024-01-15T10:30:00.hour()").expect("parse failed");
    assert!(matches!(
        expr,
        Expression::Invocation(_, Invocation::Function(ref name, _)) if name == "hour"
    ));
}

#[test]
fn test_datetime_fractional_seconds_still_work() {
    assert_eq!(
        parse("@2024-01-15T10:30:00.500"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-01-15T10:30:00.500".to_string()
        ))))
    );
}

#[test]
fn test_datetime_with_timezone() {
    assert_eq!(
        parse("@2024-01-15T10:30:00+05:00"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-01-15T10:30:00+05:00".to_string()
        ))))
    );
}

#[test]
fn test_datetime_with_z_timezone() {
    assert_eq!(
        parse("@2024-01-15T10:30:00Z"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-01-15T10:30:00Z".to_string()
        ))))
    );
}

#[test]
fn test_datetime_fractional_with_timezone() {
    assert_eq!(
        parse("@2024-01-15T10:30:00.500-05:00"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-01-15T10:30:00.500-05:00".to_string()
        ))))
    );
}

#[test]
fn test_time_dot_not_consumed_before_method() {
    let expr = parse("@T10:00:00.minute()").expect("parse failed");
    assert!(matches!(
        expr,
        Expression::Invocation(_, Invocation::Function(ref name, _)) if name == "minute"
    ));
}

#[test]
fn test_null_with_inner_whitespace() {
    assert_eq!(
        parse("{ }"),
        Ok(Expression::Term(Term::Literal(Literal::Null)))
    );
}

#[test]
fn test_null_with_multiline_whitespace() {
    assert_eq!(
        parse("{  }"),
        Ok(Expression::Term(Term::Literal(Literal::Null)))
    );
}

#[test]
fn test_parse_rejects_trailing_input() {
    let result = parse("1 + 2 garbage");
    assert!(result.is_err());
}

#[test]
fn test_parse_accepts_trailing_whitespace() {
    assert!(parse("1 + 2  ").is_ok());
}

#[test]
fn test_partial_date_year_only() {
    assert_eq!(
        parse("@2024"),
        Ok(Expression::Term(Term::Literal(Literal::Date(
            "2024".to_string()
        ))))
    );
}

#[test]
fn test_partial_date_year_month() {
    assert_eq!(
        parse("@2024-06"),
        Ok(Expression::Term(Term::Literal(Literal::Date(
            "2024-06".to_string()
        ))))
    );
}

#[test]
fn test_partial_date_in_comparison() {
    assert_eq!(
        parse("birthDate >= @2000"),
        Ok(Expression::Inequality(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "birthDate".to_string()
            )))),
            parser::InequalityOp::GreaterEqual,
            Box::new(Expression::Term(Term::Literal(Literal::Date(
                "2000".to_string()
            ))))
        ))
    );
}

#[test]
fn test_partial_date_year_month_in_comparison() {
    assert_eq!(
        parse("birthDate < @2024-01"),
        Ok(Expression::Inequality(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "birthDate".to_string()
            )))),
            parser::InequalityOp::Less,
            Box::new(Expression::Term(Term::Literal(Literal::Date(
                "2024-01".to_string()
            ))))
        ))
    );
}

#[test]
fn test_datetime_with_negative_timezone() {
    assert_eq!(
        parse("@2024-01-15T10:30:00-05:00"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-01-15T10:30:00-05:00".to_string()
        ))))
    );
}

#[test]
fn test_datetime_without_seconds() {
    assert_eq!(
        parse("@2024-01-15T10:30Z"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-01-15T10:30Z".to_string()
        ))))
    );
}

#[test]
fn test_datetime_t_only() {
    assert_eq!(
        parse("@2024-01-15T"),
        Ok(Expression::Term(Term::Literal(Literal::DateTime(
            "2024-01-15T".to_string()
        ))))
    );
}

#[test]
fn test_integer_literal() {
    assert_eq!(
        parse("42"),
        Ok(Expression::Term(Term::Literal(Literal::Number(42.0))))
    );
}

#[test]
fn test_decimal_literal() {
    assert_eq!(
        parse("2.75"),
        Ok(Expression::Term(Term::Literal(Literal::Number(2.75))))
    );
}

#[test]
fn test_zero_literal() {
    assert_eq!(
        parse("0"),
        Ok(Expression::Term(Term::Literal(Literal::Number(0.0))))
    );
}

#[test]
fn test_empty_string_literal() {
    assert_eq!(
        parse("''"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            String::new()
        ))))
    );
}

#[test]
fn test_string_with_utf8() {
    assert_eq!(
        parse("'héllo wörld'"),
        Ok(Expression::Term(Term::Literal(Literal::String(
            "héllo wörld".to_string()
        ))))
    );
}

#[test]
fn test_quantity_all_date_time_units() {
    for unit in &[
        "year",
        "years",
        "month",
        "months",
        "week",
        "weeks",
        "day",
        "days",
        "hour",
        "hours",
        "minute",
        "minutes",
        "second",
        "seconds",
        "millisecond",
        "milliseconds",
    ] {
        let input = format!("1 {unit}");
        let result = parse(&input);
        assert_eq!(
            result,
            Ok(Expression::Term(Term::Literal(Literal::Quantity(
                parser::Quantity {
                    value: 1.0,
                    unit: unit.to_string()
                }
            )))),
            "failed to parse quantity with unit: {unit}"
        );
    }
}

#[test]
fn test_quantity_decimal_value() {
    assert_eq!(
        parse("2.5 'mg'"),
        Ok(Expression::Term(Term::Literal(Literal::Quantity(
            parser::Quantity {
                value: 2.5,
                unit: "mg".to_string()
            }
        ))))
    );
}

#[test]
fn test_date_method_chaining() {
    let expr = parse("@2024.toString()").expect("parse failed");
    assert!(matches!(
        expr,
        Expression::Invocation(_, Invocation::Function(ref name, _)) if name == "toString"
    ));
}

#[test]
fn test_time_fractional_seconds_still_work() {
    assert_eq!(
        parse("@T10:30:00.500"),
        Ok(Expression::Term(Term::Literal(Literal::Time(
            "10:30:00.500".to_string()
        ))))
    );
}
