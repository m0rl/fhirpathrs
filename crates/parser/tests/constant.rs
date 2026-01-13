#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{Expression, ExternalConstant, Term, parse};

#[test]
fn test_constant_expressions() {
    assert_eq!(
        parse("%external"),
        Ok(Expression::Term(Term::ExternalConstant(ExternalConstant {
            value: "external".to_string()
        })))
    );
}
