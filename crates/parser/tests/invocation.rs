#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{AdditiveOp, Expression, Invocation, Literal, PolarityOp, SortDirection, Term, parse};

#[test]
fn test_invocation_expressions() {
    assert_eq!(
        parse("foo()"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "foo".to_string(),
            vec![]
        ))))
    );
    assert_eq!(
        parse("foo(1, 2)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "foo".to_string(),
            vec![
                Expression::Term(Term::Literal(Literal::Number("1".to_string()))),
                Expression::Term(Term::Literal(Literal::Number("2".to_string())))
            ]
        ))))
    );
    assert_eq!(
        parse("$this"),
        Ok(Expression::Term(Term::Invocation(Invocation::This)))
    );
    assert_eq!(
        parse("$index"),
        Ok(Expression::Term(Term::Invocation(Invocation::Index)))
    );
    assert_eq!(
        parse("$total"),
        Ok(Expression::Term(Term::Invocation(Invocation::Total)))
    );
}

#[test]
fn sort_accepts_asc_suffix() {
    assert_eq!(
        parse("sort(foo asc)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "sort".to_string(),
            vec![Expression::OrderedBy(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "foo".to_string()
                )))),
                SortDirection::Asc
            )]
        ))))
    );
}

#[test]
fn sort_accepts_desc_suffix() {
    assert_eq!(
        parse("sort(foo desc)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "sort".to_string(),
            vec![Expression::OrderedBy(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "foo".to_string()
                )))),
                SortDirection::Desc
            )]
        ))))
    );
}

#[test]
fn sort_suffix_binds_after_pratt_infix_loop() {
    assert_eq!(
        parse("sort(a + b desc)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "sort".to_string(),
            vec![Expression::OrderedBy(
                Box::new(Expression::Additive(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "a".to_string()
                    )))),
                    AdditiveOp::Plus,
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "b".to_string()
                    ))))
                )),
                SortDirection::Desc
            )]
        ))))
    );
}

#[test]
fn sort_suffix_on_member_chain() {
    assert_eq!(
        parse("sort(foo.bar desc)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "sort".to_string(),
            vec![Expression::OrderedBy(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "foo".to_string()
                    )))),
                    Invocation::Member("bar".to_string())
                )),
                SortDirection::Desc
            )]
        ))))
    );
}

#[test]
fn sort_multi_key_suffix() {
    assert_eq!(
        parse("sort(foo asc, bar desc)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "sort".to_string(),
            vec![
                Expression::OrderedBy(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "foo".to_string()
                    )))),
                    SortDirection::Asc
                ),
                Expression::OrderedBy(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "bar".to_string()
                    )))),
                    SortDirection::Desc
                ),
            ]
        ))))
    );
}

#[test]
fn sort_prefix_minus_binds_tighter_than_suffix() {
    assert_eq!(
        parse("sort(-foo desc)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "sort".to_string(),
            vec![Expression::OrderedBy(
                Box::new(Expression::Polarity(
                    PolarityOp::Minus,
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "foo".to_string()
                    ))))
                )),
                SortDirection::Desc
            )]
        ))))
    );
}

#[test]
fn non_sort_functions_reject_asc_desc_suffix() {
    assert!(parse("where(foo desc)").is_err());
    assert!(parse("select(x asc)").is_err());
    assert!(parse("f(x desc)").is_err());
}

#[test]
fn bare_asc_desc_parses_as_member() {
    assert_eq!(
        parse("desc"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "desc".to_string()
        ))))
    );
    assert_eq!(
        parse("asc"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "asc".to_string()
        ))))
    );
}

#[test]
fn keyword_boundary_does_not_match_prefixes() {
    assert_eq!(
        parse("descendants"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "descendants".to_string()
        ))))
    );
    assert_eq!(
        parse("ascending"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "ascending".to_string()
        ))))
    );
}

#[test]
fn sort_trailing_input_after_suffix_is_parse_error() {
    assert!(parse("sort(a desc + b)").is_err());
}
