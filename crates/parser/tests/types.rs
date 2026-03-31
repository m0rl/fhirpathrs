#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{EqualityOp, Expression, Invocation, Literal, Term, TypeOp, TypeSpecifier, parse};

#[test]
fn test_type_expressions() {
    assert_eq!(
        parse("x is System.String"),
        Ok(Expression::Type(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "x".to_string()
            )))),
            TypeOp::Is,
            TypeSpecifier::QualifiedIdentifier(vec!["System".to_string(), "String".to_string()])
        ))
    );
    assert_eq!(
        parse("1 is Integer"),
        Ok(Expression::Type(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            TypeOp::Is,
            TypeSpecifier::QualifiedIdentifier(vec!["Integer".to_string()])
        ))
    );
    assert_eq!(
        parse("1 as String"),
        Ok(Expression::Type(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            TypeOp::As,
            TypeSpecifier::QualifiedIdentifier(vec!["String".to_string()])
        ))
    );
}

#[test]
fn test_type_binds_tighter_than_union() {
    assert_eq!(
        parse("A | B is C"),
        Ok(Expression::Union(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "A".to_string()
            )))),
            Box::new(Expression::Type(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "B".to_string()
                )))),
                TypeOp::Is,
                TypeSpecifier::QualifiedIdentifier(vec!["C".to_string()])
            )),
        ))
    );
}

#[test]
fn test_type_binds_tighter_than_equality() {
    assert_eq!(
        parse("A = B is C"),
        Ok(Expression::Equality(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "A".to_string()
            )))),
            EqualityOp::Equal,
            Box::new(Expression::Type(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "B".to_string()
                )))),
                TypeOp::Is,
                TypeSpecifier::QualifiedIdentifier(vec!["C".to_string()])
            )),
        ))
    );
}
