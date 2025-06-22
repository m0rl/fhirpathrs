use nom::character::complete::none_of;
use nom::multi::fold_many0;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Term(Term),
    Invocation(Box<Expression>, Invocation),
    Indexer(Box<Expression>, Box<Expression>),
    Polarity(PolarityOp, Box<Expression>),
    Multiplicative(Box<Expression>, MultiplicativeOp, Box<Expression>),
    Additive(Box<Expression>, AdditiveOp, Box<Expression>),
    Type(Box<Expression>, TypeOp, TypeSpecifier),
    Union(Box<Expression>, Box<Expression>),
    Inequality(Box<Expression>, InequalityOp, Box<Expression>),
    Equality(Box<Expression>, EqualityOp, Box<Expression>),
    Membership(Box<Expression>, MembershipOp, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, OrOp, Box<Expression>),
    Implies(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Invocation(Invocation),
    Literal(Literal),
    ExternalConstant(ExternalConstant),
    Parenthesized(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Date(String),
    DateTime(String),
    Time(String),
    Quantity(Quantity),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalConstant {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Invocation {
    Member(String),
    Function(String, Vec<Expression>),
    This,
    Index,
    Total,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    QualifiedIdentifier(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolarityOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MultiplicativeOp {
    Multiply,
    Divide,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AdditiveOp {
    Plus,
    Minus,
    Ampersand,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeOp {
    Is,
    As,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InequalityOp {
    LessEqual,
    Less,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EqualityOp {
    Equal,
    Equivalent,
    NotEqual,
    NotEquivalent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MembershipOp {
    In,
    Contains,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrOp {
    Or,
    Xor,
}

fn ws<'a, F, O>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, f, multispace0)
}

fn identifier(input: &str) -> IResult<&str, String> {
    ws(alt((
        map(
            recognize(pair(
                alt((alpha1, tag("_"))),
                take_while(|c: char| c.is_alphanumeric() || c == '_'),
            )),
            String::from,
        ),
        map(
            alt((tag("as"), tag("contains"), tag("in"), tag("is"))),
            String::from,
        ),
    )))(input)
}

fn string_literal(input: &str) -> IResult<&str, String> {
    ws(map(
        delimited(
            char('\''),
            fold_many0(
                alt((map(tag("\\'"), |_| '\''), map(none_of("'"), |c| c))),
                String::new,
                |mut acc, c| {
                    acc.push(c);
                    acc
                },
            ),
            char('\''),
        ),
        |s| s,
    ))(input)
}

fn number(input: &str) -> IResult<&str, f64> {
    ws(map_res(
        recognize(tuple((digit1, opt(tuple((char('.'), digit1)))))),
        |s: &str| s.parse::<f64>(),
    ))(input)
}

fn date(input: &str) -> IResult<&str, String> {
    ws(map(
        preceded(
            char('@'),
            recognize(tuple((
                take_while1(|c: char| c.is_numeric()),
                opt(tuple((char('-'), take_while1(|c: char| c.is_numeric())))),
                opt(tuple((char('-'), take_while1(|c: char| c.is_numeric())))),
            ))),
        ),
        String::from,
    ))(input)
}

fn datetime(input: &str) -> IResult<&str, String> {
    ws(map(
        preceded(
            char('@'),
            recognize(tuple((
                take_while1(|c: char| c.is_numeric() || c == '-'),
                char('T'),
                opt(take_while1(|c: char| {
                    c.is_numeric() || c == ':' || c == '.' || c == '+' || c == '-' || c == 'Z'
                })),
            ))),
        ),
        String::from,
    ))(input)
}

fn time(input: &str) -> IResult<&str, String> {
    ws(map(
        preceded(
            tag("@T"),
            take_while1(|c: char| c.is_numeric() || c == ':' || c == '.'),
        ),
        String::from,
    ))(input)
}

fn date_time_precision(input: &str) -> IResult<&str, &str> {
    ws(alt((
        tag("years"),
        tag("months"),
        tag("weeks"),
        tag("days"),
        tag("hours"),
        tag("minutes"),
        tag("seconds"),
        tag("milliseconds"),
        tag("year"),
        tag("month"),
        tag("week"),
        tag("day"),
        tag("hour"),
        tag("minute"),
        tag("second"),
        tag("millisecond"),
    )))(input)
}

fn unit(input: &str) -> IResult<&str, String> {
    alt((map(date_time_precision, String::from), string_literal))(input)
}

fn quantity(input: &str) -> IResult<&str, Quantity> {
    map(tuple((number, unit)), |(value, unit)| Quantity {
        value,
        unit,
    })(input)
}

fn param_list(input: &str) -> IResult<&str, Vec<Expression>> {
    separated_list0(ws(char(',')), expression)(input)
}

fn function(input: &str) -> IResult<&str, (String, Vec<Expression>)> {
    tuple((
        identifier,
        delimited(ws(char('(')), opt(param_list), ws(char(')'))),
    ))(input)
    .map(|(rest, (name, params))| (rest, (name, params.unwrap_or_default())))
}

fn literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map(ws(tag("true")), |_| Literal::Boolean(true)),
        map(ws(tag("false")), |_| Literal::Boolean(false)),
        map(ws(tuple((char('{'), char('}')))), |_| Literal::Null),
        map(quantity, Literal::Quantity),
        map(datetime, Literal::DateTime),
        map(date, Literal::Date),
        map(time, Literal::Time),
        map(number, Literal::Number),
        map(string_literal, Literal::String),
    ))(input)
}

fn invocation(input: &str) -> IResult<&str, Invocation> {
    alt((
        map(function, |(name, params)| {
            Invocation::Function(name, params)
        }),
        map(identifier, Invocation::Member),
        map(ws(tag("$this")), |_| Invocation::This),
        map(ws(tag("$index")), |_| Invocation::Index),
        map(ws(tag("$total")), |_| Invocation::Total),
    ))(input)
}

fn external_constant(input: &str) -> IResult<&str, ExternalConstant> {
    map(
        preceded(ws(char('%')), alt((identifier, string_literal))),
        |value| ExternalConstant { value },
    )(input)
}

fn term(input: &str) -> IResult<&str, Term> {
    alt((
        map(literal, Term::Literal),
        map(invocation, Term::Invocation),
        map(external_constant, Term::ExternalConstant),
        map(delimited(ws(char('(')), expression, ws(char(')'))), |e| {
            Term::Parenthesized(Box::new(e))
        }),
    ))(input)
}

fn postfix_expression(input: &str) -> IResult<&str, Expression> {
    let (input, base) = map(term, Expression::Term)(input)?;

    #[derive(Debug, Clone, PartialEq)]
    enum PostfixOp {
        Invocation(Invocation),
        Indexer(Expression),
    }

    let (input, postfixes) = many0(alt((
        map(preceded(ws(char('.')), invocation), PostfixOp::Invocation),
        map(
            delimited(ws(char('[')), expression, ws(char(']'))),
            PostfixOp::Indexer,
        ),
    )))(input)?;

    Ok((
        input,
        postfixes.into_iter().fold(base, |acc, op| match op {
            PostfixOp::Invocation(inv) => Expression::Invocation(Box::new(acc), inv),
            PostfixOp::Indexer(expr) => Expression::Indexer(Box::new(acc), Box::new(expr)),
        }),
    ))
}

fn polarity_expression(input: &str) -> IResult<&str, Expression> {
    let (input, ops) = many0(ws(alt((
        map(char('+'), |_| PolarityOp::Plus),
        map(char('-'), |_| PolarityOp::Minus),
    ))))(input)?;

    let (input, expr) = postfix_expression(input)?;

    Ok((
        input,
        ops.into_iter()
            .rev() // Reverse to maintain right-associativity
            .fold(expr, |acc, op| Expression::Polarity(op, Box::new(acc))),
    ))
}

fn multiplicative_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = polarity_expression(input)?;

    let (input, right) = many0(tuple((
        ws(alt((
            map(char('*'), |_| MultiplicativeOp::Multiply),
            map(char('/'), |_| MultiplicativeOp::Divide),
            map(tag("div"), |_| MultiplicativeOp::Div),
            map(tag("mod"), |_| MultiplicativeOp::Mod),
        ))),
        polarity_expression,
    )))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, (op, right)| {
            Expression::Multiplicative(Box::new(acc), op, Box::new(right))
        }),
    ))
}

fn additive_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = multiplicative_expression(input)?;

    let (input, right) = many0(tuple((
        ws(alt((
            map(char('+'), |_| AdditiveOp::Plus),
            map(char('-'), |_| AdditiveOp::Minus),
            map(char('&'), |_| AdditiveOp::Ampersand),
        ))),
        multiplicative_expression,
    )))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, (op, right)| {
            Expression::Additive(Box::new(acc), op, Box::new(right))
        }),
    ))
}

fn type_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = additive_expression(input)?;

    let (input, type_spec) = many0(tuple((
        ws(alt((
            map(tag("is"), |_| TypeOp::Is),
            map(tag("as"), |_| TypeOp::As),
        ))),
        map(
            separated_list0(ws(char('.')), identifier),
            TypeSpecifier::QualifiedIdentifier,
        ),
    )))(input)?;

    Ok((
        input,
        type_spec.into_iter().fold(left, |acc, (op, ts)| {
            Expression::Type(Box::new(acc), op, ts)
        }),
    ))
}

fn union_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = type_expression(input)?;

    let (input, right) = many0(preceded(ws(char('|')), type_expression))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, right| {
            Expression::Union(Box::new(acc), Box::new(right))
        }),
    ))
}

fn inequality_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = union_expression(input)?;

    let (input, right) = many0(tuple((
        ws(alt((
            map(tag("<="), |_| InequalityOp::LessEqual),
            map(tag(">="), |_| InequalityOp::GreaterEqual),
            map(tag("<"), |_| InequalityOp::Less),
            map(tag(">"), |_| InequalityOp::Greater),
        ))),
        union_expression,
    )))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, (op, right)| {
            Expression::Inequality(Box::new(acc), op, Box::new(right))
        }),
    ))
}

fn equality_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = inequality_expression(input)?;

    let (input, right) = many0(tuple((
        ws(alt((
            map(tag("!="), |_| EqualityOp::NotEqual),
            map(tag("!~"), |_| EqualityOp::NotEquivalent),
            map(tag("="), |_| EqualityOp::Equal),
            map(tag("~"), |_| EqualityOp::Equivalent),
        ))),
        inequality_expression,
    )))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, (op, right)| {
            Expression::Equality(Box::new(acc), op, Box::new(right))
        }),
    ))
}

fn membership_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = equality_expression(input)?;

    let (input, right) = many0(tuple((
        ws(alt((
            map(tag("in"), |_| MembershipOp::In),
            map(tag("contains"), |_| MembershipOp::Contains),
        ))),
        equality_expression,
    )))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, (op, right)| {
            Expression::Membership(Box::new(acc), op, Box::new(right))
        }),
    ))
}

fn and_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = membership_expression(input)?;

    let (input, right) = many0(preceded(ws(tag("and")), membership_expression))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, right| {
            Expression::And(Box::new(acc), Box::new(right))
        }),
    ))
}

fn or_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = and_expression(input)?;

    let (input, right) = many0(tuple((
        ws(alt((
            map(tag("or"), |_| OrOp::Or),
            map(tag("xor"), |_| OrOp::Xor),
        ))),
        and_expression,
    )))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, (op, right)| {
            Expression::Or(Box::new(acc), op, Box::new(right))
        }),
    ))
}

fn expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = or_expression(input)?;

    let (input, right) = many0(preceded(ws(tag("implies")), or_expression))(input)?;

    Ok((
        input,
        right.into_iter().fold(left, |acc, right| {
            Expression::Implies(Box::new(acc), Box::new(right))
        }),
    ))
}

pub fn parse(input: &str) -> IResult<&str, Expression> {
    expression(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_expressions() {
        assert_eq!(
            parse("1 + 2 * 3"),
            Ok((
                "",
                Expression::Additive(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    AdditiveOp::Plus,
                    Box::new(Expression::Multiplicative(
                        Box::new(Expression::Term(Term::Literal(Literal::Number(2.0)))),
                        MultiplicativeOp::Multiply,
                        Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
                    ))
                )
            ))
        );
        assert_eq!(
            parse("2 * 3 + 1"),
            Ok((
                "",
                Expression::Additive(
                    Box::new(Expression::Multiplicative(
                        Box::new(Expression::Term(Term::Literal(Literal::Number(2.0)))),
                        MultiplicativeOp::Multiply,
                        Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
                    )),
                    AdditiveOp::Plus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0))))
                )
            ))
        );
        assert_eq!(
            parse("1 + 2"),
            Ok((
                "",
                Expression::Additive(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    AdditiveOp::Plus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
                )
            ))
        );
        assert_eq!(
            parse("10 - 5"),
            Ok((
                "",
                Expression::Additive(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(10.0)))),
                    AdditiveOp::Minus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(5.0))))
                )
            ))
        );
        assert_eq!(
            parse("1 & 3"),
            Ok((
                "",
                Expression::Additive(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    AdditiveOp::Ampersand,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
                )
            ))
        );
        assert_eq!(
            parse("3 * 4"),
            Ok((
                "",
                Expression::Multiplicative(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0)))),
                    MultiplicativeOp::Multiply,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
                )
            ))
        );
        assert_eq!(
            parse("3 div 4"),
            Ok((
                "",
                Expression::Multiplicative(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0)))),
                    MultiplicativeOp::Div,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
                )
            ))
        );
        assert_eq!(
            parse("3 mod 4"),
            Ok((
                "",
                Expression::Multiplicative(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0)))),
                    MultiplicativeOp::Mod,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
                )
            ))
        );
        assert_eq!(
            parse("20 / 5"),
            Ok((
                "",
                Expression::Multiplicative(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(20.0)))),
                    MultiplicativeOp::Divide,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(5.0))))
                )
            ))
        );
    }

    #[test]
    fn test_logical_expressions() {
        assert_eq!(
            parse("true and false"),
            Ok((
                "",
                Expression::And(
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
                )
            ))
        );
        assert_eq!(
            parse("1 > 2"),
            Ok((
                "",
                Expression::Inequality(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    InequalityOp::Greater,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
                )
            ))
        );
        assert_eq!(
            parse("3 = 3"),
            Ok((
                "",
                Expression::Equality(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0)))),
                    EqualityOp::Equal,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
                )
            ))
        );
        assert_eq!(
            parse("4 >= 4"),
            Ok((
                "",
                Expression::Inequality(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(4.0)))),
                    InequalityOp::GreaterEqual,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
                )
            ))
        );
        assert_eq!(
            parse("true or false"),
            Ok((
                "",
                Expression::Or(
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
                    OrOp::Or,
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
                )
            ))
        );
        assert_eq!(
            parse("true xor false"),
            Ok((
                "",
                Expression::Or(
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
                    OrOp::Xor,
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
                )
            ))
        );
        assert_eq!(
            parse("true implies false"),
            Ok((
                "",
                Expression::Implies(
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(true)))),
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(false))))
                )
            ))
        );
    }

    #[test]
    fn test_membership_expressions() {
        assert_eq!(
            parse("1 in 2"),
            Ok((
                "",
                Expression::Membership(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    MembershipOp::In,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
                )
            ))
        );
        assert_eq!(
            parse("1 contains 2"),
            Ok((
                "",
                Expression::Membership(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    MembershipOp::Contains,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
                )
            ))
        );
    }

    #[test]
    fn test_invocation_expressions() {
        assert_eq!(
            parse("foo()"),
            Ok((
                "",
                Expression::Term(Term::Invocation(Invocation::Function(
                    "foo".to_string(),
                    vec![]
                )))
            ))
        );
        assert_eq!(
            parse("foo(1, 2)"),
            Ok((
                "",
                Expression::Term(Term::Invocation(Invocation::Function(
                    "foo".to_string(),
                    vec![
                        Expression::Term(Term::Literal(Literal::Number(1.0))),
                        Expression::Term(Term::Literal(Literal::Number(2.0)))
                    ]
                )))
            ))
        );
        assert_eq!(
            parse("$this"),
            Ok(("", Expression::Term(Term::Invocation(Invocation::This))))
        );
        assert_eq!(
            parse("$index"),
            Ok(("", Expression::Term(Term::Invocation(Invocation::Index))))
        );
        assert_eq!(
            parse("$total"),
            Ok(("", Expression::Term(Term::Invocation(Invocation::Total))))
        );
    }

    #[test]
    fn test_literal_expressions() {
        assert_eq!(
            parse("{}"),
            Ok(("", Expression::Term(Term::Literal(Literal::Null))))
        );
        assert_eq!(
            parse("'test'"),
            Ok((
                "",
                Expression::Term(Term::Literal(Literal::String("test".to_string())))
            ))
        );
        assert_eq!(
            parse("@2024-06-14"),
            Ok((
                "",
                Expression::Term(Term::Literal(Literal::Date("2024-06-14".to_string())))
            ))
        );
        assert_eq!(
            parse("@2024-06-14T15:30:00"),
            Ok((
                "",
                Expression::Term(Term::Literal(Literal::DateTime(
                    "2024-06-14T15:30:00".to_string()
                )))
            ))
        );
        assert_eq!(
            parse("@T15:30:00"),
            Ok((
                "",
                Expression::Term(Term::Literal(Literal::Time("15:30:00".to_string())))
            ))
        );
    }

    #[test]
    fn test_constant_expressions() {
        assert_eq!(
            parse("%external"),
            Ok((
                "",
                Expression::Term(Term::ExternalConstant(ExternalConstant {
                    value: "external".to_string()
                }))
            ))
        );
    }

    #[test]
    fn test_type_expressions() {
        assert_eq!(
            parse("x is System.String"),
            Ok((
                "",
                Expression::Type(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "x".to_string()
                    )))),
                    TypeOp::Is,
                    TypeSpecifier::QualifiedIdentifier(vec![
                        "System".to_string(),
                        "String".to_string()
                    ])
                )
            ))
        );
        assert_eq!(
            parse("1 is Integer"),
            Ok((
                "",
                Expression::Type(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    TypeOp::Is,
                    TypeSpecifier::QualifiedIdentifier(vec!["Integer".to_string()])
                )
            ))
        );
        assert_eq!(
            parse("1 as String"),
            Ok((
                "",
                Expression::Type(
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                    TypeOp::As,
                    TypeSpecifier::QualifiedIdentifier(vec!["String".to_string()])
                )
            ))
        );
    }

    #[test]
    fn test_complex_expressions() {
        assert_eq!(
            parse("(1 + 2) * 3"),
            Ok((
                "",
                Expression::Multiplicative(
                    Box::new(Expression::Term(Term::Parenthesized(Box::new(
                        Expression::Additive(
                            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                            AdditiveOp::Plus,
                            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
                        )
                    )))),
                    MultiplicativeOp::Multiply,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
                )
            ))
        );
        assert_eq!(
            parse("foo.bar[1]"),
            Ok((
                "",
                Expression::Indexer(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "foo".to_string()
                        )))),
                        Invocation::Member("bar".to_string())
                    )),
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0))))
                )
            ))
        );
        assert_eq!(
            parse("foo.bar.baz[0][1]"),
            Ok((
                "",
                Expression::Indexer(
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
                        Box::new(Expression::Term(Term::Literal(Literal::Number(0.0))))
                    )),
                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0))))
                )
            ))
        );
    }

    #[test]
    fn test_spec_3_2_paths() {
        assert_eq!(
            parse("Observation.value"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                        "Observation".to_string()
                    )))),
                    Invocation::Member("value".to_string())
                )
            ))
        );
        assert_eq!(
            parse("Observation.contained[0].value"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Indexer(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "Observation".to_string()
                            )))),
                            Invocation::Member("contained".to_string())
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(0.0))))
                    )),
                    Invocation::Member("value".to_string())
                )
            ))
        );
        assert_eq!(
            parse("Observation.contained[0] is Observation"),
            Ok((
                "",
                Expression::Type(
                    Box::new(Expression::Indexer(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                                "Observation".to_string()
                            )))),
                            Invocation::Member("contained".to_string())
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(0.0))))
                    )),
                    TypeOp::Is,
                    TypeSpecifier::QualifiedIdentifier(vec!["Observation".to_string()])
                )
            ))
        );
        assert_eq!(
            parse("Observation.children().value"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "Observation".to_string()
                        )))),
                        Invocation::Function("children".to_string(), vec![])
                    )),
                    Invocation::Member("value".to_string())
                )
            ))
        );
        assert_eq!(
            parse("Observation.children().children().value"),
            Ok((
                "",
                Expression::Invocation(
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
                )
            ))
        );
        assert_eq!(
            parse("Observation.descendants().value"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Invocation(Invocation::Member(
                            "Observation".to_string()
                        )))),
                        Invocation::Function("descendants".to_string(), vec![])
                    )),
                    Invocation::Member("value".to_string())
                )
            ))
        );
        assert_eq!(
            parse("Observation.descendants().where(resourceType = 'Observation').value"),
            Ok((
                "",
                Expression::Invocation(
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
                )
            ))
        );
        assert_eq!(
            parse(
                "contained.where(resourceType = 'QuestionnaireResponse').item.where(linkId = '1').answer.value"
            ),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Invocation(
                                Box::new(Expression::Invocation(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("contained".to_string())
                                    ))),
                                    Invocation::Function(
                                        "where".to_string(),
                                        vec![Expression::Equality(
                                            Box::new(Expression::Term(Term::Invocation(
                                                Invocation::Member("resourceType".to_string())
                                            ))),
                                            EqualityOp::Equal,
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::String(
                                                    "QuestionnaireResponse".to_string()
                                                )
                                            )))
                                        )]
                                    )
                                )),
                                Invocation::Member("item".to_string())
                            )),
                            Invocation::Function(
                                "where".to_string(),
                                vec![Expression::Equality(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("linkId".to_string())
                                    ))),
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
                )
            ))
        );
        assert_eq!(
            parse(
                "contained.where(resourceType = 'QuestionnaireResponse').descendants().where(linkId = '1.1').answer.value"
            ),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Invocation(
                                Box::new(Expression::Invocation(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("contained".to_string())
                                    ))),
                                    Invocation::Function(
                                        "where".to_string(),
                                        vec![Expression::Equality(
                                            Box::new(Expression::Term(Term::Invocation(
                                                Invocation::Member("resourceType".to_string())
                                            ))),
                                            EqualityOp::Equal,
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::String(
                                                    "QuestionnaireResponse".to_string()
                                                )
                                            )))
                                        )]
                                    )
                                )),
                                Invocation::Function("descendants".to_string(), vec![])
                            )),
                            Invocation::Function(
                                "where".to_string(),
                                vec![Expression::Equality(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("linkId".to_string())
                                    ))),
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
                )
            ))
        );
        assert_eq!(
            parse("contained.where(resourceType = 'QuestionnaireResponse').item.answer.value"),
            Ok((
                "",
                Expression::Invocation(
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
                        Invocation::Member("answer".to_string())
                    )),
                    Invocation::Member("value".to_string())
                )
            ))
        );
        assert_eq!(
            parse("contained.where(resourceType = 'QuestionnaireResponse').item.item.answer.value"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Invocation(
                                Box::new(Expression::Invocation(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("contained".to_string())
                                    ))),
                                    Invocation::Function(
                                        "where".to_string(),
                                        vec![Expression::Equality(
                                            Box::new(Expression::Term(Term::Invocation(
                                                Invocation::Member("resourceType".to_string())
                                            ))),
                                            EqualityOp::Equal,
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::String(
                                                    "QuestionnaireResponse".to_string()
                                                )
                                            )))
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
                )
            ))
        );
    }

    #[test]
    fn test_spec_4_1_literals() {
        assert_eq!(
            parse("2 'mo'"),
            Ok((
                "",
                Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                    value: 2.0,
                    unit: "mo".to_string()
                })))
            ))
        );
        assert_eq!(
            parse("2 years"),
            Ok((
                "",
                Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                    value: 2.0,
                    unit: "years".to_string()
                })))
            ))
        );
        assert_eq!(
            parse("@2025-09-13T01:00Z = @2015-09-13T21:00-04:00"),
            Ok((
                "",
                Expression::Equality(
                    Box::new(Expression::Term(Term::Literal(Literal::DateTime(
                        "2025-09-13T01:00Z".to_string()
                    )))),
                    EqualityOp::Equal,
                    Box::new(Expression::Term(Term::Literal(Literal::DateTime(
                        "2015-09-13T21:00-04:00".to_string()
                    ))))
                )
            ))
        );
        assert_eq!(
            parse("-7"),
            Ok((
                "",
                Expression::Polarity(
                    PolarityOp::Minus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(7.0))))
                )
            ))
        );
        assert_eq!(
            parse("+7"),
            Ok((
                "",
                Expression::Polarity(
                    PolarityOp::Plus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(7.0))))
                )
            ))
        );
        assert_eq!(
            parse("-7.3"),
            Ok((
                "",
                Expression::Polarity(
                    PolarityOp::Minus,
                    Box::new(Expression::Term(Term::Literal(Literal::Number(7.3))))
                )
            ))
        );
        assert_eq!(
            parse("(-7).combine(3)"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Term(Term::Parenthesized(Box::new(
                        Expression::Polarity(
                            PolarityOp::Minus,
                            Box::new(Expression::Term(Term::Literal(Literal::Number(7.0))))
                        )
                    )))),
                    Invocation::Function(
                        "combine".to_string(),
                        vec![Expression::Term(Term::Literal(Literal::Number(3.0)))]
                    )
                )
            ))
        );
        assert_eq!(
            parse("-true"),
            Ok((
                "",
                Expression::Polarity(
                    PolarityOp::Minus,
                    Box::new(Expression::Term(Term::Literal(Literal::Boolean(true))))
                )
            ))
        );
        assert_eq!(
            parse("-'zzz'"),
            Ok((
                "",
                Expression::Polarity(
                    PolarityOp::Minus,
                    Box::new(Expression::Term(Term::Literal(Literal::String(
                        "zzz".to_string()
                    ))))
                )
            ))
        );
    }

    #[test]
    fn test_spec_5_2_filtering_and_projection() {
        assert_eq!(
            parse("Patient.name.given.where(id = 'Test').count()"),
            Ok((
                "",
                Expression::Invocation(
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
                )
            ))
        );
        assert_eq!(
            parse("Patient.name.given.select($this & $this.id)"),
            Ok((
                "",
                Expression::Invocation(
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
                )
            ))
        );
        assert_eq!(
            parse("(1 | 2).repeat('item')"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Term(Term::Parenthesized(Box::new(
                        Expression::Union(
                            Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                            Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
                        )
                    )))),
                    Invocation::Function(
                        "repeat".to_string(),
                        vec![Expression::Term(Term::Literal(Literal::String(
                            "item".to_string()
                        )))]
                    )
                )
            ))
        );
        assert_eq!(
            parse("(1 year).combine(12 months).repeat($this)"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Parenthesized(Box::new(
                            Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                                value: 1.0,
                                unit: "year".to_string()
                            })))
                        )))),
                        Invocation::Function(
                            "combine".to_string(),
                            vec![Expression::Term(Term::Literal(Literal::Quantity(
                                Quantity {
                                    value: 12.0,
                                    unit: "months".to_string()
                                }
                            )))]
                        )
                    )),
                    Invocation::Function(
                        "repeat".to_string(),
                        vec![Expression::Term(Term::Invocation(Invocation::This))]
                    )
                )
            ))
        );
        assert_eq!(
            parse("(3 'min').combine(180 seconds).repeat($this)"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Parenthesized(Box::new(
                            Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                                value: 3.0,
                                unit: "min".to_string()
                            })))
                        )))),
                        Invocation::Function(
                            "combine".to_string(),
                            vec![Expression::Term(Term::Literal(Literal::Quantity(
                                Quantity {
                                    value: 180.0,
                                    unit: "seconds".to_string()
                                }
                            )))]
                        )
                    )),
                    Invocation::Function(
                        "repeat".to_string(),
                        vec![Expression::Term(Term::Invocation(Invocation::This))]
                    )
                )
            ))
        );
        assert_eq!(
            parse("Patient.name.given.ofType(System.String)"),
            Ok((
                "",
                Expression::Invocation(
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
                )
            ))
        );
    }

    #[test]
    fn test_spec_5_2_3_repeat() {
        assert_eq!(
            parse("Questionnaire.combine(Questionnaire).repeat(item).linkId"),
            Ok((
                "",
                Expression::Invocation(
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
                )
            ))
        );
    }

    #[test]
    fn test_spec_5_3_subsetting() {
        assert_eq!(
            parse("Bundle.entry.intersect(Bundle.entry) = (Bundle.entry[0] | Bundle.entry[2])"),
            Ok((
                "",
                Expression::Equality(
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
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("Bundle".to_string())
                                    ))),
                                    Invocation::Member("entry".to_string())
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(0.0))))
                            )),
                            Box::new(Expression::Indexer(
                                Box::new(Expression::Invocation(
                                    Box::new(Expression::Term(Term::Invocation(
                                        Invocation::Member("Bundle".to_string())
                                    ))),
                                    Invocation::Member("entry".to_string())
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(2.0))))
                            ))
                        )
                    ))))
                )
            ))
        );
        assert_eq!(
            parse("(1 year).combine(12 months).intersect(12 months)"),
            Ok((
                "",
                Expression::Invocation(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Parenthesized(Box::new(
                            Expression::Term(Term::Literal(Literal::Quantity(Quantity {
                                value: 1.0,
                                unit: "year".to_string()
                            })))
                        )))),
                        Invocation::Function(
                            "combine".to_string(),
                            vec![Expression::Term(Term::Literal(Literal::Quantity(
                                Quantity {
                                    value: 12.0,
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
                                unit: "months".to_string()
                            }
                        )))]
                    )
                )
            ))
        );
        assert_eq!(
            parse(
                "(1 year | 2 year | 3 year | 4 year | 5 year | 6 year).combine(12 months).intersect(12 months)"
            ),
            Ok((
                "",
                Expression::Invocation(
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
                                                        unit: "year".to_string()
                                                    })
                                                ))),
                                                Box::new(Expression::Term(Term::Literal(
                                                    Literal::Quantity(Quantity {
                                                        value: 2.0,
                                                        unit: "year".to_string()
                                                    })
                                                )))
                                            )),
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::Quantity(Quantity {
                                                    value: 3.0,
                                                    unit: "year".to_string()
                                                })
                                            )))
                                        )),
                                        Box::new(Expression::Term(Term::Literal(
                                            Literal::Quantity(Quantity {
                                                value: 4.0,
                                                unit: "year".to_string()
                                            })
                                        )))
                                    )),
                                    Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                                        Quantity {
                                            value: 5.0,
                                            unit: "year".to_string()
                                        }
                                    ))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Quantity(
                                    Quantity {
                                        value: 6.0,
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
                                unit: "months".to_string()
                            }
                        )))]
                    )
                )
            ))
        );
        assert_eq!(
            parse("(1 | 2 | 3 | 5 | 6 | 7).exclude(2 | 4) = 1 | 3 | 5 | 6 | 7"),
            Ok((
                "",
                Expression::Equality(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Parenthesized(Box::new(
                            Expression::Union(
                                Box::new(Expression::Union(
                                    Box::new(Expression::Union(
                                        Box::new(Expression::Union(
                                            Box::new(Expression::Union(
                                                Box::new(Expression::Term(Term::Literal(
                                                    Literal::Number(1.0)
                                                ))),
                                                Box::new(Expression::Term(Term::Literal(
                                                    Literal::Number(2.0)
                                                )))
                                            )),
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::Number(3.0)
                                            )))
                                        )),
                                        Box::new(Expression::Term(Term::Literal(Literal::Number(
                                            5.0
                                        ))))
                                    )),
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(6.0))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(7.0))))
                            )
                        )))),
                        Invocation::Function(
                            "exclude".to_string(),
                            vec![Expression::Union(
                                Box::new(Expression::Term(Term::Literal(Literal::Number(2.0)))),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
                            )]
                        )
                    )),
                    EqualityOp::Equal,
                    Box::new(Expression::Union(
                        Box::new(Expression::Union(
                            Box::new(Expression::Union(
                                Box::new(Expression::Union(
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(1.0)))),
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(3.0))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(5.0))))
                            )),
                            Box::new(Expression::Term(Term::Literal(Literal::Number(6.0))))
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(7.0))))
                    ))
                )
            ))
        );
        assert_eq!(
            parse("(1 | 2 | 3 | 4 | 5 | 6 | 7).exclude({}) = 1 | 2 | 3 | 4 | 5 | 6 | 7"),
            Ok((
                "",
                Expression::Equality(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Term(Term::Parenthesized(Box::new(
                            Expression::Union(
                                Box::new(Expression::Union(
                                    Box::new(Expression::Union(
                                        Box::new(Expression::Union(
                                            Box::new(Expression::Union(
                                                Box::new(Expression::Union(
                                                    Box::new(Expression::Term(Term::Literal(
                                                        Literal::Number(1.0)
                                                    ))),
                                                    Box::new(Expression::Term(Term::Literal(
                                                        Literal::Number(2.0)
                                                    )))
                                                )),
                                                Box::new(Expression::Term(Term::Literal(
                                                    Literal::Number(3.0)
                                                )))
                                            )),
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::Number(4.0)
                                            )))
                                        )),
                                        Box::new(Expression::Term(Term::Literal(Literal::Number(
                                            5.0
                                        ))))
                                    )),
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(6.0))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(7.0))))
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
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::Number(1.0)
                                            ))),
                                            Box::new(Expression::Term(Term::Literal(
                                                Literal::Number(2.0)
                                            )))
                                        )),
                                        Box::new(Expression::Term(Term::Literal(Literal::Number(
                                            3.0
                                        ))))
                                    )),
                                    Box::new(Expression::Term(Term::Literal(Literal::Number(4.0))))
                                )),
                                Box::new(Expression::Term(Term::Literal(Literal::Number(5.0))))
                            )),
                            Box::new(Expression::Term(Term::Literal(Literal::Number(6.0))))
                        )),
                        Box::new(Expression::Term(Term::Literal(Literal::Number(7.0))))
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_spec_5_5_conversion() {
        assert_eq!(
            parse("'1 \\'wk\\''.toQuantity() = 7 days"),
            Ok((
                "",
                Expression::Equality(
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
                            unit: "days".to_string()
                        }
                    ))))
                )
            ))
        );
        assert_eq!(
            parse("'1 \\'wk\\''.toQuantity('d').toString() = '7 \\'d\\''"),
            Ok((
                "",
                Expression::Equality(
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
                )
            ))
        );
        assert_eq!(
            parse("'1 year'.toQuantity() ~ 1 'a'"),
            Ok((
                "",
                Expression::Equality(
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
                            unit: "a".to_string()
                        }
                    ))))
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
            Ok((
                "",
                Expression::Or(
                    Box::new(Expression::Invocation(
                        Box::new(Expression::Invocation(
                            Box::new(Expression::Invocation(
                                Box::new(Expression::Invocation(
                                    Box::new(Expression::Invocation(
                                        Box::new(Expression::Term(Term::Invocation(
                                            Invocation::Member("Patient".to_string())
                                        ))),
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
                                                        Box::new(Expression::Term(
                                                            Term::Invocation(Invocation::Member(
                                                                "system".to_string()
                                                            ))
                                                        )),
                                                        EqualityOp::Equal,
                                                        Box::new(Expression::Term(Term::Literal(
                                                            Literal::String(
                                                                "fully-qualified-uri".to_string()
                                                            )
                                                        )))
                                                    )),
                                                    Box::new(Expression::Equality(
                                                        Box::new(Expression::Term(
                                                            Term::Invocation(Invocation::Member(
                                                                "code".to_string()
                                                            ))
                                                        )),
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
                                                Box::new(Expression::Term(Term::Literal(
                                                    Literal::String("official".to_string())
                                                )))
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
                                        unit: "years".to_string()
                                    }
                                ))))
                            ))
                        ))
                    ))
                )
            ))
        );
    }
}
