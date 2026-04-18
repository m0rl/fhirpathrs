#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{
    AdditiveOp, EqualityOp, Expression, ExternalConstant, InequalityOp, Invocation, Literal,
    MembershipOp, MultiplicativeOp, OrOp, PolarityOp, Quantity, Term, TypeOp, TypeSpecifier, parse,
};

#[test]
fn test_logical_expressions() {
    assert_eq!(
        parse("true and false"),
        Ok(Expression::And(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
    assert_eq!(
        parse("1 > 2"),
        Ok(Expression::Inequality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            InequalityOp::Greater,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
    assert_eq!(
        parse("3 = 3"),
        Ok(Expression::Equality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0)))),
            EqualityOp::Equal,
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0))))
        ))
    );
    assert_eq!(
        parse("4 >= 4"),
        Ok(Expression::Inequality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0, 0)))),
            InequalityOp::GreaterEqual,
            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0, 0))))
        ))
    );
    assert_eq!(
        parse("true or false"),
        Ok(Expression::Or(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            OrOp::Or,
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
    assert_eq!(
        parse("true xor false"),
        Ok(Expression::Or(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            OrOp::Xor,
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
    assert_eq!(
        parse("true implies false"),
        Ok(Expression::Implies(
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
        ))
    );
}

#[test]
fn test_membership_expressions() {
    assert_eq!(
        parse("1 in 2"),
        Ok(Expression::Membership(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            MembershipOp::In,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
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
fn test_complex_expressions() {
    assert_eq!(
        parse("(1 + 2) * 3"),
        Ok(Expression::Multiplicative(
            Box::new(Expression::Term(Term::Parenthesized(Box::new(
                Expression::Additive(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                    AdditiveOp::Plus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
                )
            )))),
            MultiplicativeOp::Multiply,
            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0))))
        ))
    );
    assert_eq!(
        parse("foo.bar[1]"),
        Ok(Expression::Indexer(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "foo".to_string()
                )))),
                Invocation::Member("bar".to_string())
            )),
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0))))
        ))
    );
    assert_eq!(
        parse("foo.bar.baz[0][1]"),
        Ok(Expression::Indexer(
            Box::new(Expression::Indexer(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "foo".to_string()
                        )))),
                        Invocation::Member("bar".to_string())
                    )),
                    Invocation::Member("baz".to_string())
                )),
                Box::new(Expression::Term(Term::Literal(Literal::Number(0.0, 0))))
            )),
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0))))
        ))
    );
}

#[test]
fn test_less_than() {
    assert_eq!(
        parse("1 < 2"),
        Ok(Expression::Inequality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            InequalityOp::Less,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
}

#[test]
fn test_less_equal() {
    assert_eq!(
        parse("1 <= 2"),
        Ok(Expression::Inequality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            InequalityOp::LessEqual,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
}

#[test]
fn test_not_equal() {
    assert_eq!(
        parse("1 != 2"),
        Ok(Expression::Equality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            EqualityOp::NotEqual,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
}

#[test]
fn test_not_equal_alias_angle_brackets() {
    assert_eq!(parse("1 <> 2"), parse("1 != 2"));
}

#[test]
fn test_not_equivalent() {
    assert_eq!(
        parse("1 !~ 2"),
        Ok(Expression::Equality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            EqualityOp::NotEquivalent,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
}

#[test]
fn test_equivalent() {
    assert_eq!(
        parse("1 ~ 2"),
        Ok(Expression::Equality(
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
            EqualityOp::Equivalent,
            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
        ))
    );
}

#[test]
fn test_nested_function_calls() {
    assert_eq!(
        parse("foo(bar())"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "foo".to_string(),
            vec![Expression::Term(Term::Invocation(Invocation::Function(
                "bar".to_string(),
                vec![]
            )))]
        ))))
    );
}

#[test]
fn test_function_with_three_arguments() {
    assert_eq!(
        parse("iif(true, 1, 2)"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "iif".to_string(),
            vec![
                Expression::Term(Term::Literal(Literal::Boolean(true))),
                Expression::Term(Term::Literal(Literal::Number(1.0, 0))),
                Expression::Term(Term::Literal(Literal::Number(2.0, 0)))
            ]
        ))))
    );
}

#[test]
fn test_deeply_nested_function_args() {
    assert_eq!(
        parse("iif(a.exists(), a.where(b = 1).first(), {})"),
        Ok(Expression::Term(Term::Invocation(Invocation::Function(
            "iif".to_string(),
            vec![
                Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "a".to_string()
                    )))),
                    Invocation::Function("exists".to_string(), vec![])
                ),
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "a".to_string()
                        )))),
                        Invocation::Function(
                            "where".to_string(),
                            vec![Expression::Equality(
                                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                    "b".to_string()
                                )))),
                                EqualityOp::Equal,
                                Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0))))
                            )]
                        )
                    )),
                    Invocation::Function("first".to_string(), vec![])
                ),
                Expression::Term(Term::Literal(Literal::Null))
            ]
        ))))
    );
}

#[test]
fn test_expression_in_indexer() {
    assert_eq!(
        parse("a[1 + 2]"),
        Ok(Expression::Indexer(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "a".to_string()
            )))),
            Box::new(Expression::Additive(
                Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                AdditiveOp::Plus,
                Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
            ))
        ))
    );
}

#[test]
fn test_null_in_arithmetic() {
    assert_eq!(
        parse("{} + 1"),
        Ok(Expression::Additive(
            Box::new(Expression::Term(Term::Literal(Literal::Null))),
            AdditiveOp::Plus,
            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0))))
        ))
    );
}

#[test]
fn test_null_in_equality() {
    assert_eq!(
        parse("x = {}"),
        Ok(Expression::Equality(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "x".to_string()
            )))),
            EqualityOp::Equal,
            Box::new(Expression::Term(Term::Literal(Literal::Null)))
        ))
    );
}

#[test]
fn test_chained_comparisons_left_associative() {
    assert_eq!(
        parse("a < b and b < c"),
        Ok(Expression::And(
            Box::new(Expression::Inequality(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "a".to_string()
                )))),
                InequalityOp::Less,
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "b".to_string()
                ))))
            )),
            Box::new(Expression::Inequality(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "b".to_string()
                )))),
                InequalityOp::Less,
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "c".to_string()
                ))))
            ))
        ))
    );
}

#[test]
fn test_all_precedence_levels() {
    assert_eq!(
        parse("a * b + c < d = e in f and g or h implies i"),
        Ok(Expression::Implies(
            Box::new(Expression::Or(
                Box::new(Expression::And(
                    Box::new(Expression::Membership(
                        Box::new(Expression::Equality(
                            Box::new(Expression::Inequality(
                                Box::new(Expression::Additive(
                                    Box::new(Expression::Multiplicative(
                                        Box::new(Expression::Term(Term::Invocation(
                                            Invocation::Member("a".to_string())
                                        ))),
                                        MultiplicativeOp::Multiply,
                                        Box::new(Expression::Term(Term::Invocation(
                                            Invocation::Member("b".to_string())
                                        )))
                                    )),
                                    AdditiveOp::Plus,
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("c".to_string())
                                    )))
                                )),
                                InequalityOp::Less,
                                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                    "d".to_string()
                                ))))
                            )),
                            EqualityOp::Equal,
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "e".to_string()
                            ))))
                        )),
                        MembershipOp::In,
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "f".to_string()
                        ))))
                    )),
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "g".to_string()
                    ))))
                )),
                OrOp::Or,
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "h".to_string()
                ))))
            )),
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "i".to_string()
            ))))
        ))
    );
}

#[test]
fn test_union_in_indexer() {
    assert_eq!(
        parse("a[1 | 2]"),
        Ok(Expression::Indexer(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "a".to_string()
            )))),
            Box::new(Expression::Union(
                Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
            ))
        ))
    );
}

#[test]
fn test_polarity_on_parenthesized_union() {
    assert_eq!(
        parse("-(1 | 2)"),
        Ok(Expression::Polarity(
            PolarityOp::Minus,
            Box::new(Expression::Term(Term::Parenthesized(Box::new(
                Expression::Union(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
                )
            ))))
        ))
    );
}

#[test]
fn test_quoted_external_constant() {
    assert_eq!(
        parse("%'vs-administrative-gender'"),
        Ok(Expression::Term(Term::ExternalConstant(ExternalConstant {
            value: "vs-administrative-gender".to_string()
        })))
    );
}

#[test]
fn test_external_constant_in_expression() {
    assert_eq!(
        parse("code in %'vs-administrative-gender'"),
        Ok(Expression::Membership(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "code".to_string()
            )))),
            MembershipOp::In,
            Box::new(Expression::Term(Term::ExternalConstant(ExternalConstant {
                value: "vs-administrative-gender".to_string()
            })))
        ))
    );
}

#[test]
fn test_type_check_in_where() {
    assert_eq!(
        parse("value.where($this is Quantity)"),
        Ok(Expression::Invocation(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "value".to_string()
            )))),
            Invocation::Function(
                "where".to_string(),
                vec![Expression::Type(
                    Box::new(Expression::Term(Term::Invocation(Invocation::This))),
                    TypeOp::Is,
                    TypeSpecifier::QualifiedIdentifier(vec!["Quantity".to_string()])
                )]
            )
        ))
    );
}

#[test]
fn test_multipart_logic_expression() {
    assert_eq!(
        parse(
            "Patient.contact
                    .where(relationship.coding
                        .exists(system = 'fully-qualified-uri' and code = 'C1'))
                    .name.given.first()
                or Patient.name
                    .where(use = 'official')
                    .given.first()
                in Patient.link.other.resolve().name.given
                and Patient.birthDate <= today() - 38 'years'"
        ),
        Ok(Expression::Or(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Invocation(
                                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                    "Patient".to_string()
                                )))),
                                Invocation::Member("contact".to_string())
                            )),
                            Invocation::Function(
                                "where".to_string(),
                                vec![Expression::Invocation(
                                    Box::new(Expression::Invocation(
                                        Box::new(Expression::Term(Term::Invocation(
                                            Invocation::Member("relationship".to_string())
                                        ))),
                                        Invocation::Member("coding".to_string())
                                    )),
                                    Invocation::Function(
                                        "exists".to_string(),
                                        vec![Expression::And(
                                            Box::new(Expression::Equality(
                                                Box::new(Expression::Term(Term::Invocation(
                                                    Invocation::Member("system".to_string())
                                                ))),
                                                EqualityOp::Equal,
                                                Box::new(Expression::Term(Term::Literal(
                                                    Literal::String(
                                                        "fully-qualified-uri".to_string()
                                                    )
                                                )))
                                            )),
                                            Box::new(Expression::Equality(
                                                Box::new(Expression::Term(Term::Invocation(
                                                    Invocation::Member("code".to_string())
                                                ))),
                                                EqualityOp::Equal,
                                                Box::new(Expression::Term(Term::Literal(
                                                    Literal::String("C1".to_string())
                                                )))
                                            ))
                                        )]
                                    )
                                )]
                            )
                        )),
                        Invocation::Member("name".to_string())
                    )),
                    Invocation::Member("given".to_string())
                )),
                Invocation::Function("first".to_string(), vec![])
            )),
            OrOp::Or,
            Box::new(Expression::And(
                Box::new(Expression::Membership(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Invocation(
                                Box::new(Expression::Invocation(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("Patient".to_string())
                                    ))),
                                    Invocation::Member("name".to_string())
                                )),
                                Invocation::Function(
                                    "where".to_string(),
                                    vec![Expression::Equality(
                                        Box::new(Expression::Term(Term::Invocation(
                                            Invocation::Member("use".to_string())
                                        ))),
                                        EqualityOp::Equal,
                                        Box::new(Expression::Term(Term::Literal(Literal::String(
                                            "official".to_string()
                                        ))))
                                    )]
                                )
                            )),
                            Invocation::Member("given".to_string())
                        )),
                        Invocation::Function("first".to_string(), vec![])
                    )),
                    MembershipOp::In,
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Invocation(
                                Box::new(Expression::Invocation(
                                    Box::new(Expression::Invocation(
                                        Box::new(Expression::Term(Term::Invocation(
                                            Invocation::Member("Patient".to_string())
                                        ))),
                                        Invocation::Member("link".to_string())
                                    )),
                                    Invocation::Member("other".to_string())
                                )),
                                Invocation::Function("resolve".to_string(), vec![])
                            )),
                            Invocation::Member("name".to_string())
                        )),
                        Invocation::Member("given".to_string())
                    ))
                )),
                Box::new(Expression::Inequality(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "Patient".to_string()
                        )))),
                        Invocation::Member("birthDate".to_string())
                    )),
                    InequalityOp::LessEqual,
                    Box::new(Expression::Additive(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Function(
                            "today".to_string(),
                            vec![]
                        )))),
                        AdditiveOp::Minus,
                        Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                            Quantity {
                                value: 38.0,
                                precision: 0,
                                unit: "years".to_string()
                            }
                        ))))
                    ))
                ))
            ))
        ))
    );
}
