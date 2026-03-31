#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{
    AdditiveOp, Expression, Invocation, Literal, MembershipOp, MultiplicativeOp, OrOp, Term,
    TypeOp, TypeSpecifier, parse,
};

#[test]
fn test_true_not_prefix_of_identifier() {
    let expr = parse("trueValue").expect("parse failed");
    assert_eq!(
        expr,
        Expression::Term(Term::Invocation(Invocation::Member(
            "trueValue".to_string()
        )))
    );
}

#[test]
fn test_false_not_prefix_of_identifier() {
    let expr = parse("falsify").expect("parse failed");
    assert_eq!(
        expr,
        Expression::Term(Term::Invocation(Invocation::Member("falsify".to_string())))
    );
}

#[test]
fn test_true_still_works_standalone() {
    assert_eq!(
        parse("true"),
        Ok(Expression::Term(Term::Literal(Literal::Boolean(true))))
    );
}

#[test]
fn test_false_still_works_standalone() {
    assert_eq!(
        parse("false"),
        Ok(Expression::Term(Term::Literal(Literal::Boolean(false))))
    );
}

#[test]
fn test_div_not_prefix_of_identifier() {
    assert_eq!(
        parse("x.divine"),
        Ok(Expression::Invocation(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "x".to_string()
            )))),
            Invocation::Member("divine".to_string())
        ))
    );
}

#[test]
fn test_mod_not_prefix_of_identifier() {
    assert_eq!(
        parse("x.modified"),
        Ok(Expression::Invocation(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "x".to_string()
            )))),
            Invocation::Member("modified".to_string())
        ))
    );
}

#[test]
fn test_in_not_prefix_of_identifier() {
    assert_eq!(
        parse("information"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "information".to_string()
        ))))
    );
}

#[test]
fn test_contains_not_prefix_of_identifier() {
    assert_eq!(
        parse("containsAll"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "containsAll".to_string()
        ))))
    );
}

#[test]
fn test_and_not_prefix_of_identifier() {
    assert_eq!(
        parse("android"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "android".to_string()
        ))))
    );
}

#[test]
fn test_or_not_prefix_of_identifier() {
    assert_eq!(
        parse("order"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "order".to_string()
        ))))
    );
}

#[test]
fn test_is_not_prefix_of_identifier() {
    assert_eq!(
        parse("island"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "island".to_string()
        ))))
    );
}

#[test]
fn test_as_not_prefix_of_identifier() {
    assert_eq!(
        parse("assembly"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "assembly".to_string()
        ))))
    );
}

#[test]
fn test_implies_not_prefix_of_identifier() {
    assert_eq!(
        parse("impliesNothing"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "impliesNothing".to_string()
        ))))
    );
}

#[test]
fn test_xor_not_prefix_of_identifier() {
    assert_eq!(
        parse("xorValue"),
        Ok(Expression::Term(Term::Invocation(Invocation::Member(
            "xorValue".to_string()
        ))))
    );
}

#[test]
fn test_keyword_and_with_spaces() {
    assert_eq!(
        parse("true and false"),
        Ok(Expression::And(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
}

#[test]
fn test_keyword_or_with_spaces() {
    assert_eq!(
        parse("true or false"),
        Ok(Expression::Or(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            OrOp::Or,
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
}

#[test]
fn test_keyword_xor_with_spaces() {
    assert_eq!(
        parse("true xor false"),
        Ok(Expression::Or(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            OrOp::Xor,
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
}

#[test]
fn test_keyword_implies_with_spaces() {
    assert_eq!(
        parse("true implies false"),
        Ok(Expression::Implies(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
}

#[test]
fn test_keyword_in_with_spaces() {
    assert_eq!(
        parse("1 in 2"),
        Ok(Expression::Membership(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            MembershipOp::In,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
}

#[test]
fn test_keyword_contains_with_spaces() {
    assert_eq!(
        parse("1 contains 2"),
        Ok(Expression::Membership(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            MembershipOp::Contains,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
}

#[test]
fn test_keyword_div_with_spaces() {
    assert_eq!(
        parse("3 div 4"),
        Ok(Expression::Multiplicative(
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0)))),
            MultiplicativeOp::Div,
            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0, 0))))
        ))
    );
}

#[test]
fn test_keyword_mod_with_spaces() {
    assert_eq!(
        parse("3 mod 4"),
        Ok(Expression::Multiplicative(
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0)))),
            MultiplicativeOp::Mod,
            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0, 0))))
        ))
    );
}

#[test]
fn test_keyword_is_with_spaces() {
    assert_eq!(
        parse("x is Integer"),
        Ok(Expression::Type(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "x".to_string()
            )))),
            TypeOp::Is,
            TypeSpecifier::QualifiedIdentifier(vec!["Integer".to_string()])
        ))
    );
}

#[test]
fn test_keyword_as_with_spaces() {
    assert_eq!(
        parse("x as String"),
        Ok(Expression::Type(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "x".to_string()
            )))),
            TypeOp::As,
            TypeSpecifier::QualifiedIdentifier(vec!["String".to_string()])
        ))
    );
}

#[test]
fn test_keyword_in_expression_with_add() {
    assert_eq!(
        parse("1 + 2 in 3"),
        Ok(Expression::Membership(
            Box::new(Expression::Additive(
                Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                AdditiveOp::Plus,
                Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
            )),
            MembershipOp::In,
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0))))
        ))
    );
}

#[test]
fn test_moderate_nesting_succeeds() {
    let depth = 100;
    let input = "(".repeat(depth) + "1" + &")".repeat(depth);
    assert!(parse(&input).is_ok());
}

#[test]
fn test_deep_nesting_succeeds() {
    let depth = 10000;
    let input = "(".repeat(depth) + "1" + &")".repeat(depth);
    assert!(parse(&input).is_ok());
}
