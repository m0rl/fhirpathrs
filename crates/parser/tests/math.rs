#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{AdditiveOp, Expression, Literal, MultiplicativeOp, Term, parse};

#[test]
fn test_arithmetic_expressions() {
    assert_eq!(
        parse("1 + 2 * 3"),
        Ok(Expression::Additive(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
            AdditiveOp::Plus,
            Box::new(Expression::Multiplicative(
                Box::new(Expression::Term(Term::Literal(Literal::Number(2.0)))),
                MultiplicativeOp::Multiply,
                Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
            ))
        ))
    );
    assert_eq!(
        parse("2 * 3 + 1"),
        Ok(Expression::Additive(
            Box::new(Expression::Multiplicative(
                Box::new(Expression::Term(Term::Literal(Literal::Number(2.0)))),
                MultiplicativeOp::Multiply,
                Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
            )),
            AdditiveOp::Plus,
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0))))
        ))
    );
    assert_eq!(
        parse("1 + 2"),
        Ok(Expression::Additive(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
            AdditiveOp::Plus,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
        ))
    );
    assert_eq!(
        parse("10 - 5"),
        Ok(Expression::Additive(
            Box::new(Expression::Term(Term::Literal(Literal::Number(10.0)))),
            AdditiveOp::Minus,
            Box::new(Expression::Term(Term::Literal(Literal::Number(5.0))))
        ))
    );
    assert_eq!(
        parse("1 & 3"),
        Ok(Expression::Additive(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
            AdditiveOp::Ampersand,
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
        ))
    );
    assert_eq!(
        parse("3 * 4"),
        Ok(Expression::Multiplicative(
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0)))),
            MultiplicativeOp::Multiply,
            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
        ))
    );
    assert_eq!(
        parse("3 div 4"),
        Ok(Expression::Multiplicative(
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0)))),
            MultiplicativeOp::Div,
            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
        ))
    );
    assert_eq!(
        parse("3 mod 4"),
        Ok(Expression::Multiplicative(
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0)))),
            MultiplicativeOp::Mod,
            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
        ))
    );
    assert_eq!(
        parse("20 / 5"),
        Ok(Expression::Multiplicative(
            Box::new(Expression::Term(Term::Literal(Literal::Number(20.0)))),
            MultiplicativeOp::Divide,
            Box::new(Expression::Term(Term::Literal(Literal::Number(5.0))))
        ))
    );
}
