#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]
use parser::{
    AdditiveOp, EqualityOp, Expression, Invocation, Literal, PolarityOp, Quantity, Term, TypeOp,
    TypeSpecifier, parse,
};

#[test]
fn test_spec_3_2_paths() {
    assert_eq!(
        parse("Observation.value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                "Observation".to_string()
            )))),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse("Observation.contained[0].value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Indexer(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Observation".to_string()
                    )))),
                    Invocation::Member("contained".to_string())
                )),
                Box::new(Expression::Term(Term::Literal(Literal::Number(0.0, 0))))
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse("Observation.contained[0] is Observation"),
        Ok(Expression::Type(
            Box::new(Expression::Indexer(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Observation".to_string()
                    )))),
                    Invocation::Member("contained".to_string())
                )),
                Box::new(Expression::Term(Term::Literal(Literal::Number(0.0, 0))))
            )),
            TypeOp::Is,
            TypeSpecifier::QualifiedIdentifier(vec!["Observation".to_string()])
        ))
    );
    assert_eq!(
        parse("Observation.children().value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "Observation".to_string()
                )))),
                Invocation::Function("children".to_string(), vec![])
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse("Observation.children().children().value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Observation".to_string()
                    )))),
                    Invocation::Function("children".to_string(), vec![])
                )),
                Invocation::Function("children".to_string(), vec![])
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse("Observation.descendants().value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                    "Observation".to_string()
                )))),
                Invocation::Function("descendants".to_string(), vec![])
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse("Observation.descendants().where(resourceType = 'Observation').value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Observation".to_string()
                    )))),
                    Invocation::Function("descendants".to_string(), vec![])
                )),
                Invocation::Function(
                    "where".to_string(),
                    vec![Expression::Equality(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "resourceType".to_string()
                        )))),
                        EqualityOp::Equal,
                        Box::new(Expression::Term(Term::Literal(Literal::String(
                            "Observation".to_string()
                        ))))
                    )]
                )
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse(
            "contained.where(resourceType = 'QuestionnaireResponse').item.where(linkId = '1').answer.value"
        ),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "contained".to_string()
                            )))),
                            Invocation::Function(
                                "where".to_string(),
                                vec![Expression::Equality(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("resourceType".to_string())
                                    ))),
                                    EqualityOp::Equal,
                                    Box::new(Expression::Term(Term::Literal(Literal::String(
                                        "QuestionnaireResponse".to_string()
                                    ))))
                                )]
                            )
                        )),
                        Invocation::Member("item".to_string())
                    )),
                    Invocation::Function(
                        "where".to_string(),
                        vec![Expression::Equality(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "linkId".to_string()
                            )))),
                            EqualityOp::Equal,
                            Box::new(Expression::Term(Term::Literal(Literal::String(
                                "1".to_string()
                            ))))
                        )]
                    )
                )),
                Invocation::Member("answer".to_string())
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse(
            "contained.where(resourceType = 'QuestionnaireResponse').descendants().where(linkId = '1.1').answer.value"
        ),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "contained".to_string()
                            )))),
                            Invocation::Function(
                                "where".to_string(),
                                vec![Expression::Equality(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("resourceType".to_string())
                                    ))),
                                    EqualityOp::Equal,
                                    Box::new(Expression::Term(Term::Literal(Literal::String(
                                        "QuestionnaireResponse".to_string()
                                    ))))
                                )]
                            )
                        )),
                        Invocation::Function("descendants".to_string(), vec![])
                    )),
                    Invocation::Function(
                        "where".to_string(),
                        vec![Expression::Equality(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "linkId".to_string()
                            )))),
                            EqualityOp::Equal,
                            Box::new(Expression::Term(Term::Literal(Literal::String(
                                "1.1".to_string()
                            ))))
                        )]
                    )
                )),
                Invocation::Member("answer".to_string())
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse("contained.where(resourceType = 'QuestionnaireResponse').item.answer.value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "contained".to_string()
                        )))),
                        Invocation::Function(
                            "where".to_string(),
                            vec![Expression::Equality(
                                Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                    "resourceType".to_string()
                                )))),
                                EqualityOp::Equal,
                                Box::new(Expression::Term(Term::Literal(Literal::String(
                                    "QuestionnaireResponse".to_string()
                                ))))
                            )]
                        )
                    )),
                    Invocation::Member("item".to_string())
                )),
                Invocation::Member("answer".to_string())
            )),
            Invocation::Member("value".to_string())
        ))
    );
    assert_eq!(
        parse("contained.where(resourceType = 'QuestionnaireResponse').item.item.answer.value"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "contained".to_string()
                            )))),
                            Invocation::Function(
                                "where".to_string(),
                                vec![Expression::Equality(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("resourceType".to_string())
                                    ))),
                                    EqualityOp::Equal,
                                    Box::new(Expression::Term(Term::Literal(Literal::String(
                                        "QuestionnaireResponse".to_string()
                                    ))))
                                )]
                            )
                        )),
                        Invocation::Member("item".to_string())
                    )),
                    Invocation::Member("item".to_string())
                )),
                Invocation::Member("answer".to_string())
            )),
            Invocation::Member("value".to_string())
        ))
    );
}

#[test]
fn test_spec_4_1_literals() {
    assert_eq!(
        parse("2 'mo'"),
        Ok(Expression::Term(Term::Literal(Literal::Quantity(
            Quantity {
                value: 2.0,
                precision: 0,
                unit: "mo".to_string()
            }
        ))))
    );
    assert_eq!(
        parse("2 years"),
        Ok(Expression::Term(Term::Literal(Literal::Quantity(
            Quantity {
                value: 2.0,
                precision: 0,
                unit: "years".to_string()
            }
        ))))
    );
    assert_eq!(
        parse("@2025-09-13T01:00Z = @2015-09-13T21:00-04:00"),
        Ok(Expression::Equality(
            Box::new(Expression::Term(Term::Literal(Literal::DateTime(
                "2025-09-13T01:00Z".to_string()
            )))),
            EqualityOp::Equal,
            Box::new(Expression::Term(Term::Literal(Literal::DateTime(
                "2015-09-13T21:00-04:00".to_string()
            ))))
        ))
    );
    assert_eq!(
        parse("-7"),
        Ok(Expression::Polarity(
            PolarityOp::Minus,
            Box::new(Expression::Term(Term::Literal(Literal::Number(7.0, 0))))
        ))
    );
    assert_eq!(
        parse("+7"),
        Ok(Expression::Polarity(
            PolarityOp::Plus,
            Box::new(Expression::Term(Term::Literal(Literal::Number(7.0, 0))))
        ))
    );
    assert_eq!(
        parse("-7.3"),
        Ok(Expression::Polarity(
            PolarityOp::Minus,
            Box::new(Expression::Term(Term::Literal(Literal::Number(7.3, 1))))
        ))
    );
    assert_eq!(
        parse("(-7).combine(3)"),
        Ok(Expression::Invocation(
            Box::new(Expression::Term(Term::Parenthesized(Box::new(
                Expression::Polarity(
                    PolarityOp::Minus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(7.0, 0))))
                )
            )))),
            Invocation::Function(
                "combine".to_string(),
                vec![Expression::Term(Term::Literal(Literal::Number(3.0, 0)))]
            )
        ))
    );
    assert_eq!(
        parse("-true"),
        Ok(Expression::Polarity(
            PolarityOp::Minus,
            Box::new(Expression::Term(Term::Literal(Literal::Boolean(true))))
        ))
    );
    assert_eq!(
        parse("-'zzz'"),
        Ok(Expression::Polarity(
            PolarityOp::Minus,
            Box::new(Expression::Term(Term::Literal(Literal::String(
                "zzz".to_string()
            ))))
        ))
    );
}

#[test]
fn test_spec_5_2_filtering_and_projection() {
    assert_eq!(
        parse("Patient.name.given.where(id = 'Test').count()"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "Patient".to_string()
                        )))),
                        Invocation::Member("name".to_string())
                    )),
                    Invocation::Member("given".to_string())
                )),
                Invocation::Function(
                    "where".to_string(),
                    vec![Expression::Equality(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "id".to_string()
                        )))),
                        EqualityOp::Equal,
                        Box::new(Expression::Term(Term::Literal(Literal::String(
                            "Test".to_string()
                        ))))
                    )]
                )
            )),
            Invocation::Function("count".to_string(), vec![])
        ))
    );
    assert_eq!(
        parse("Patient.name.given.select($this & $this.id)"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Patient".to_string()
                    )))),
                    Invocation::Member("name".to_string())
                )),
                Invocation::Member("given".to_string())
            )),
            Invocation::Function(
                "select".to_string(),
                vec![Expression::Additive(
                    Box::new(Expression::Term(Term::Invocation(Invocation::This))),
                    AdditiveOp::Ampersand,
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::This))),
                        Invocation::Member("id".to_string())
                    ))
                )]
            )
        ))
    );
    assert_eq!(
        parse("(1 | 2).repeat('item')"),
        Ok(Expression::Invocation(
            Box::new(Expression::Term(Term::Parenthesized(Box::new(
                Expression::Union(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
                )
            )))),
            Invocation::Function(
                "repeat".to_string(),
                vec![Expression::Term(Term::Literal(Literal::String(
                    "item".to_string()
                )))]
            )
        ))
    );
    assert_eq!(
        parse("(1 year).combine(12 months).repeat($this)"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Parenthesized(Box::new(
                    Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                        value: 1.0,
                        precision: 0,
                        unit: "year".to_string()
                    })))
                )))),
                Invocation::Function(
                    "combine".to_string(),
                    vec![Expression::Term(Term::Literal(Literal::Quantity(
                        Quantity {
                            value: 12.0,
                            precision: 0,
                            unit: "months".to_string()
                        }
                    )))]
                )
            )),
            Invocation::Function(
                "repeat".to_string(),
                vec![Expression::Term(Term::Invocation(Invocation::This))]
            )
        ))
    );
    assert_eq!(
        parse("(3 'min').combine(180 seconds).repeat($this)"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Parenthesized(Box::new(
                    Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                        value: 3.0,
                        precision: 0,
                        unit: "min".to_string()
                    })))
                )))),
                Invocation::Function(
                    "combine".to_string(),
                    vec![Expression::Term(Term::Literal(Literal::Quantity(
                        Quantity {
                            value: 180.0,
                            precision: 0,
                            unit: "seconds".to_string()
                        }
                    )))]
                )
            )),
            Invocation::Function(
                "repeat".to_string(),
                vec![Expression::Term(Term::Invocation(Invocation::This))]
            )
        ))
    );
    assert_eq!(
        parse("Patient.name.given.ofType(System.String)"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Patient".to_string()
                    )))),
                    Invocation::Member("name".to_string())
                )),
                Invocation::Member("given".to_string())
            )),
            Invocation::Function(
                "ofType".to_string(),
                vec![Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "System".to_string()
                    )))),
                    Invocation::Member("String".to_string())
                )]
            )
        ))
    );
}

#[test]
fn test_spec_5_2_3_repeat() {
    assert_eq!(
        parse("Questionnaire.combine(Questionnaire).repeat(item).linkId"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Questionnaire".to_string()
                    )))),
                    Invocation::Function(
                        "combine".to_string(),
                        vec![Expression::Term(Term::Invocation(Invocation::Member(
                            "Questionnaire".to_string()
                        )))]
                    )
                )),
                Invocation::Function(
                    "repeat".to_string(),
                    vec![Expression::Term(Term::Invocation(Invocation::Member(
                        "item".to_string()
                    )))]
                )
            )),
            Invocation::Member("linkId".to_string())
        ))
    );
}

#[test]
fn test_spec_5_3_subsetting() {
    assert_eq!(
        parse("Bundle.entry.intersect(Bundle.entry) = (Bundle.entry[0] | Bundle.entry[2])"),
        Ok(Expression::Equality(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Bundle".to_string()
                    )))),
                    Invocation::Member("entry".to_string())
                )),
                Invocation::Function(
                    "intersect".to_string(),
                    vec![Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "Bundle".to_string()
                        )))),
                        Invocation::Member("entry".to_string())
                    )]
                )
            )),
            EqualityOp::Equal,
            Box::new(Expression::Term(Term::Parenthesized(Box::new(
                Expression::Union(
                    Box::new(Expression::Indexer(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "Bundle".to_string()
                            )))),
                            Invocation::Member("entry".to_string())
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(0.0, 0))))
                    )),
                    Box::new(Expression::Indexer(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "Bundle".to_string()
                            )))),
                            Invocation::Member("entry".to_string())
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
                    ))
                )
            ))))
        ))
    );
    assert_eq!(
        parse("(1 year).combine(12 months).intersect(12 months)"),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Parenthesized(Box::new(
                    Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                        value: 1.0,
                        precision: 0,
                        unit: "year".to_string()
                    })))
                )))),
                Invocation::Function(
                    "combine".to_string(),
                    vec![Expression::Term(Term::Literal(Literal::Quantity(
                        Quantity {
                            value: 12.0,
                            precision: 0,
                            unit: "months".to_string()
                        }
                    )))]
                )
            )),
            Invocation::Function(
                "intersect".to_string(),
                vec![Expression::Term(Term::Literal(Literal::Quantity(
                    Quantity {
                        value: 12.0,
                        precision: 0,
                        unit: "months".to_string()
                    }
                )))]
            )
        ))
    );
    assert_eq!(
        parse(
            "(1 year | 2 year | 3 year | 4 year | 5 year | 6 year).combine(12 months).intersect(12 months)"
        ),
        Ok(Expression::Invocation(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Parenthesized(Box::new(
                    Expression::Union(
                        Box::new(Expression::Union(
                            Box::new(Expression::Union(
                                Box::new(Expression::Union(
                                    Box::new(Expression::Union(
                                        Box::new(Expression::Term(Term::Literal(
                                            Literal::Quantity(Quantity {
                                                value: 1.0,
                                                precision: 0,
                                                unit: "year".to_string()
                                            })
                                        ))),
                                        Box::new(Expression::Term(Term::Literal(
                                            Literal::Quantity(Quantity {
                                                value: 2.0,
                                                precision: 0,
                                                unit: "year".to_string()
                                            })
                                        )))
                                    )),
                                    Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                                        Quantity {
                                            value: 3.0,
                                            precision: 0,
                                            unit: "year".to_string()
                                        }
                                    ))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                                    Quantity {
                                        value: 4.0,
                                        precision: 0,
                                        unit: "year".to_string()
                                    }
                                ))))
                            )),
                            Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                                Quantity {
                                    value: 5.0,
                                    precision: 0,
                                    unit: "year".to_string()
                                }
                            ))))
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                            Quantity {
                                value: 6.0,
                                precision: 0,
                                unit: "year".to_string()
                            }
                        ))))
                    )
                )))),
                Invocation::Function(
                    "combine".to_string(),
                    vec![Expression::Term(Term::Literal(Literal::Quantity(
                        Quantity {
                            value: 12.0,
                            precision: 0,
                            unit: "months".to_string()
                        }
                    )))]
                )
            )),
            Invocation::Function(
                "intersect".to_string(),
                vec![Expression::Term(Term::Literal(Literal::Quantity(
                    Quantity {
                        value: 12.0,
                        precision: 0,
                        unit: "months".to_string()
                    }
                )))]
            )
        ))
    );
    assert_eq!(
        parse("(1 | 2 | 3 | 5 | 6 | 7).exclude(2 | 4) = 1 | 3 | 5 | 6 | 7"),
        Ok(Expression::Equality(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Parenthesized(Box::new(
                    Expression::Union(
                        Box::new(Expression::Union(
                            Box::new(Expression::Union(
                                Box::new(Expression::Union(
                                    Box::new(Expression::Union(
                                        Box::new(Expression::Term(Term::Literal(Literal::Number(
                                            1.0, 0
                                        )))),
                                        Box::new(Expression::Term(Term::Literal(Literal::Number(
                                            2.0, 0
                                        ))))
                                    )),
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(5.0, 0))))
                            )),
                            Box::new(Expression::Term(Term::Literal(Literal::Number(6.0, 0))))
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(7.0, 0))))
                    )
                )))),
                Invocation::Function(
                    "exclude".to_string(),
                    vec![Expression::Union(
                        Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0)))),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(4.0, 0))))
                    )]
                )
            )),
            EqualityOp::Equal,
            Box::new(Expression::Union(
                Box::new(Expression::Union(
                    Box::new(Expression::Union(
                        Box::new(Expression::Union(
                            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                            Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0))))
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(5.0, 0))))
                    )),
                    Box::new(Expression::Term(Term::Literal(Literal::Number(6.0, 0))))
                )),
                Box::new(Expression::Term(Term::Literal(Literal::Number(7.0, 0))))
            ))
        ))
    );
    assert_eq!(
        parse("(1 | 2 | 3 | 4 | 5 | 6 | 7).exclude({}) = 1 | 2 | 3 | 4 | 5 | 6 | 7"),
        Ok(Expression::Equality(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Parenthesized(Box::new(
                    Expression::Union(
                        Box::new(Expression::Union(
                            Box::new(Expression::Union(
                                Box::new(Expression::Union(
                                    Box::new(Expression::Union(
                                        Box::new(Expression::Union(
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::Number(1.0, 0)
                                            ))),
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::Number(2.0, 0)
                                            )))
                                        )),
                                        Box::new(Expression::Term(Term::Literal(Literal::Number(
                                            3.0, 0
                                        ))))
                                    )),
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(4.0, 0))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(5.0, 0))))
                            )),
                            Box::new(Expression::Term(Term::Literal(Literal::Number(6.0, 0))))
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(7.0, 0))))
                    )
                )))),
                Invocation::Function(
                    "exclude".to_string(),
                    vec![Expression::Term(Term::Literal(Literal::Null))]
                )
            )),
            EqualityOp::Equal,
            Box::new(Expression::Union(
                Box::new(Expression::Union(
                    Box::new(Expression::Union(
                        Box::new(Expression::Union(
                            Box::new(Expression::Union(
                                Box::new(Expression::Union(
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0, 0)))),
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0, 0))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(3.0, 0))))
                            )),
                            Box::new(Expression::Term(Term::Literal(Literal::Number(4.0, 0))))
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(5.0, 0))))
                    )),
                    Box::new(Expression::Term(Term::Literal(Literal::Number(6.0, 0))))
                )),
                Box::new(Expression::Term(Term::Literal(Literal::Number(7.0, 0))))
            ))
        ))
    );
}

#[test]
fn test_spec_5_5_conversion() {
    assert_eq!(
        parse("'1 \\'wk\\''.toQuantity() = 7 days"),
        Ok(Expression::Equality(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Literal(Literal::String(
                    "1 'wk'".to_string()
                )))),
                Invocation::Function("toQuantity".to_string(), vec![])
            )),
            EqualityOp::Equal,
            Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                Quantity {
                    value: 7.0,
                    precision: 0,
                    unit: "days".to_string()
                }
            ))))
        ))
    );
    assert_eq!(
        parse("'1 \\'wk\\''.toQuantity('d').toString() = '7 \\'d\\''"),
        Ok(Expression::Equality(
            Box::new(Expression::Invocation(
                Box::new(Expression::Invocation(
                    Box::new(Expression::Term(Term::Literal(Literal::String(
                        "1 'wk'".to_string()
                    )))),
                    Invocation::Function(
                        "toQuantity".to_string(),
                        vec![Expression::Term(Term::Literal(Literal::String(
                            "d".to_string()
                        )))]
                    )
                )),
                Invocation::Function("toString".to_string(), vec![])
            )),
            EqualityOp::Equal,
            Box::new(Expression::Term(Term::Literal(Literal::String(
                "7 'd'".to_string()
            ))))
        ))
    );
    assert_eq!(
        parse("'1 year'.toQuantity() ~ 1 'a'"),
        Ok(Expression::Equality(
            Box::new(Expression::Invocation(
                Box::new(Expression::Term(Term::Literal(Literal::String(
                    "1 year".to_string()
                )))),
                Invocation::Function("toQuantity".to_string(), vec![])
            )),
            EqualityOp::Equivalent,
            Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                Quantity {
                    value: 1.0,
                    precision: 0,
                    unit: "a".to_string()
                }
            ))))
        ))
    );
}
