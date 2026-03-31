#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{Expression, Invocation, Literal, Term, parse};

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
                Expression::Term(Term::Literal(Literal::Number(1.0, 0))),
                Expression::Term(Term::Literal(Literal::Number(2.0, 0)))
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
