use nom::character::complete::{none_of, satisfy};
use nom::multi::fold_many0;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, map_res, not, opt, peek, recognize},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
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
    Number(f64, u8),
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
    pub precision: u8,
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

fn keyword<'a>(kw: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    terminated(
        tag(kw),
        peek(not(satisfy(|c: char| c.is_alphanumeric() || c == '_'))),
    )
}

fn identifier(input: &str) -> IResult<&str, String> {
    ws(alt((
        map(
            delimited(char('`'), take_while1(|c: char| c != '`'), char('`')),
            String::from,
        ),
        map(
            recognize(pair(
                alt((alpha1, tag("_"))),
                take_while(|c: char| c.is_alphanumeric() || c == '_'),
            )),
            String::from,
        ),
    )))(input)
}

fn unicode_escape(input: &str) -> IResult<&str, char> {
    map_res(
        preceded(
            tag("\\u"),
            recognize(tuple((
                satisfy(|c| c.is_ascii_hexdigit()),
                satisfy(|c| c.is_ascii_hexdigit()),
                satisfy(|c| c.is_ascii_hexdigit()),
                satisfy(|c| c.is_ascii_hexdigit()),
            ))),
        ),
        |hex: &str| {
            u32::from_str_radix(hex, 16)
                .map_err(|e| e.to_string())
                .and_then(|cp| char::from_u32(cp).ok_or_else(|| "invalid code point".to_string()))
        },
    )(input)
}

fn string_literal(input: &str) -> IResult<&str, String> {
    ws(delimited(
        char('\''),
        fold_many0(
            alt((
                map(tag("\\'"), |_| '\''),
                map(tag("\\\\"), |_| '\\'),
                map(tag("\\\""), |_| '"'),
                map(tag("\\`"), |_| '`'),
                map(tag("\\/"), |_| '/'),
                map(tag("\\n"), |_| '\n'),
                map(tag("\\r"), |_| '\r'),
                map(tag("\\t"), |_| '\t'),
                map(tag("\\f"), |_| '\x0C'),
                unicode_escape,
                map(char('\\'), |c| c),
                map(none_of("'\\"), |c| c),
            )),
            String::new,
            |mut acc, c| {
                acc.push(c);
                acc
            },
        ),
        char('\''),
    ))(input)
}

fn number(input: &str) -> IResult<&str, (f64, u8)> {
    ws(map_res(
        recognize(tuple((digit1, opt(tuple((char('.'), digit1)))))),
        |s: &str| {
            s.parse::<f64>().map(|n| {
                #[allow(clippy::cast_possible_truncation)]
                let precision = s.find('.').map_or(0, |dot| (s.len() - dot - 1) as u8);
                (n, precision)
            })
        },
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
                opt(tuple((
                    take_while1(|c: char| c.is_numeric() || c == ':'),
                    opt(tuple((char('.'), digit1))),
                    opt(alt((
                        recognize(tuple((
                            alt((char('+'), char('-'))),
                            take_while1(|c: char| c.is_numeric() || c == ':'),
                        ))),
                        tag("Z"),
                    ))),
                ))),
            ))),
        ),
        String::from,
    ))(input)
}

fn time(input: &str) -> IResult<&str, String> {
    ws(map(
        preceded(
            tag("@T"),
            recognize(tuple((
                take_while1(|c: char| c.is_numeric() || c == ':'),
                opt(tuple((char('.'), digit1))),
                opt(alt((
                    recognize(tuple((
                        alt((char('+'), char('-'))),
                        take_while1(|c: char| c.is_numeric() || c == ':'),
                    ))),
                    tag("Z"),
                ))),
            ))),
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
    map(tuple((number, unit)), |((value, precision), unit)| {
        Quantity {
            value,
            precision,
            unit,
        }
    })(input)
}

fn literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map(ws(keyword("true")), |_| Literal::Boolean(true)),
        map(ws(keyword("false")), |_| Literal::Boolean(false)),
        map(ws(tuple((char('{'), multispace0, char('}')))), |_| {
            Literal::Null
        }),
        map(quantity, Literal::Quantity),
        map(datetime, Literal::DateTime),
        map(date, Literal::Date),
        map(time, Literal::Time),
        map(number, |(n, p)| Literal::Number(n, p)),
        map(string_literal, Literal::String),
    ))(input)
}

fn external_constant(input: &str) -> IResult<&str, ExternalConstant> {
    map(
        preceded(ws(char('%')), alt((identifier, string_literal))),
        |value| ExternalConstant { value },
    )(input)
}

fn type_specifier(input: &str) -> IResult<&str, TypeSpecifier> {
    map(
        separated_list1(ws(char('.')), identifier),
        TypeSpecifier::QualifiedIdentifier,
    )(input)
}

enum InfixOp {
    Implies,
    Or(OrOp),
    And,
    Type(TypeOp),
    Membership(MembershipOp),
    Equality(EqualityOp),
    Inequality(InequalityOp),
    Union,
    Additive(AdditiveOp),
    Multiplicative(MultiplicativeOp),
}

impl InfixOp {
    fn binding_powers(&self) -> (u8, u8) {
        match self {
            Self::Implies => (1, 2),
            Self::Or(_) => (3, 4),
            Self::And => (5, 6),
            Self::Membership(_) => (7, 8),
            Self::Equality(_) => (9, 10),
            Self::Inequality(_) => (11, 12),
            Self::Union => (13, 14),
            Self::Type(_) => (15, 16),
            Self::Additive(_) => (17, 18),
            Self::Multiplicative(_) => (19, 20),
        }
    }

    #[allow(clippy::panic)] // Type is handled inline in the parser, never reaches here
    fn build_expression(self, left: Expression, right: Expression) -> Expression {
        let (left, right) = (Box::new(left), Box::new(right));
        match self {
            Self::Implies => Expression::Implies(left, right),
            Self::Or(op) => Expression::Or(left, op, right),
            Self::And => Expression::And(left, right),
            Self::Type(_) => unreachable!(),
            Self::Membership(op) => Expression::Membership(left, op, right),
            Self::Equality(op) => Expression::Equality(left, op, right),
            Self::Inequality(op) => Expression::Inequality(left, op, right),
            Self::Union => Expression::Union(left, right),
            Self::Additive(op) => Expression::Additive(left, op, right),
            Self::Multiplicative(op) => Expression::Multiplicative(left, op, right),
        }
    }
}

enum Frame {
    Infix {
        left: Expression,
        op: InfixOp,
        min_bp: u8,
    },
    Paren {
        polarity_ops: Vec<PolarityOp>,
        min_bp: u8,
    },
    Bracket {
        base: Expression,
        min_bp: u8,
    },
    FuncArg {
        base_expr: Option<Expression>,
        name: String,
        args: Vec<Expression>,
        polarity_ops: Vec<PolarityOp>,
        min_bp: u8,
    },
}

fn peek_infix_op(input: &str) -> Option<(InfixOp, &str)> {
    let input = input.trim_start();
    if let Ok((r, _)) = keyword("implies")(input) {
        return Some((InfixOp::Implies, r));
    }
    if let Ok((r, _)) = keyword("contains")(input) {
        return Some((InfixOp::Membership(MembershipOp::Contains), r));
    }
    if let Ok((r, _)) = keyword("xor")(input) {
        return Some((InfixOp::Or(OrOp::Xor), r));
    }
    if let Ok((r, _)) = keyword("or")(input) {
        return Some((InfixOp::Or(OrOp::Or), r));
    }
    if let Ok((r, _)) = keyword("and")(input) {
        return Some((InfixOp::And, r));
    }
    if let Ok((r, _)) = keyword("is")(input) {
        return Some((InfixOp::Type(TypeOp::Is), r));
    }
    if let Ok((r, _)) = keyword("as")(input) {
        return Some((InfixOp::Type(TypeOp::As), r));
    }
    if let Ok((r, _)) = keyword("in")(input) {
        return Some((InfixOp::Membership(MembershipOp::In), r));
    }
    if let Ok((r, _)) = keyword("div")(input) {
        return Some((InfixOp::Multiplicative(MultiplicativeOp::Div), r));
    }
    if let Ok((r, _)) = keyword("mod")(input) {
        return Some((InfixOp::Multiplicative(MultiplicativeOp::Mod), r));
    }
    if let Some(rest) = input.strip_prefix("!=") {
        return Some((InfixOp::Equality(EqualityOp::NotEqual), rest));
    }
    if let Some(rest) = input.strip_prefix("!~") {
        return Some((InfixOp::Equality(EqualityOp::NotEquivalent), rest));
    }
    if let Some(rest) = input.strip_prefix("<=") {
        return Some((InfixOp::Inequality(InequalityOp::LessEqual), rest));
    }
    if let Some(rest) = input.strip_prefix(">=") {
        return Some((InfixOp::Inequality(InequalityOp::GreaterEqual), rest));
    }
    match input.as_bytes().first() {
        Some(b'=') => Some((InfixOp::Equality(EqualityOp::Equal), &input[1..])),
        Some(b'~') => Some((InfixOp::Equality(EqualityOp::Equivalent), &input[1..])),
        Some(b'<') => Some((InfixOp::Inequality(InequalityOp::Less), &input[1..])),
        Some(b'>') => Some((InfixOp::Inequality(InequalityOp::Greater), &input[1..])),
        Some(b'|') => Some((InfixOp::Union, &input[1..])),
        Some(b'+') => Some((InfixOp::Additive(AdditiveOp::Plus), &input[1..])),
        Some(b'-') => Some((InfixOp::Additive(AdditiveOp::Minus), &input[1..])),
        Some(b'&') => Some((InfixOp::Additive(AdditiveOp::Ampersand), &input[1..])),
        Some(b'*') => Some((
            InfixOp::Multiplicative(MultiplicativeOp::Multiply),
            &input[1..],
        )),
        Some(b'/') => Some((
            InfixOp::Multiplicative(MultiplicativeOp::Divide),
            &input[1..],
        )),
        _ => None,
    }
}

fn parse_postfix_chain(
    mut input: &str,
    mut expr: Expression,
) -> IResult<&str, (Expression, Option<String>)> {
    loop {
        let after_dot = match input.trim_start().strip_prefix('.') {
            Some(rest) => rest,
            None => return Ok((input, (expr, None))),
        };

        if let Ok((inp, _)) = ws(tag("$this"))(after_dot) {
            expr = Expression::Invocation(Box::new(expr), Invocation::This);
            input = inp;
            continue;
        }
        if let Ok((inp, _)) = ws(tag("$index"))(after_dot) {
            expr = Expression::Invocation(Box::new(expr), Invocation::Index);
            input = inp;
            continue;
        }
        if let Ok((inp, _)) = ws(tag("$total"))(after_dot) {
            expr = Expression::Invocation(Box::new(expr), Invocation::Total);
            input = inp;
            continue;
        }

        match identifier(after_dot) {
            Ok((inp, name)) => {
                if let Some(rest) = inp.trim_start().strip_prefix('(') {
                    if let Some(rest2) = rest.trim_start().strip_prefix(')') {
                        expr = Expression::Invocation(
                            Box::new(expr),
                            Invocation::Function(name, vec![]),
                        );
                        input = rest2;
                        continue;
                    }
                    return Ok((rest, (expr, Some(name))));
                }
                expr = Expression::Invocation(Box::new(expr), Invocation::Member(name));
                input = inp;
            }
            Err(nom::Err::Error(_)) => return Ok((input, (expr, None))),
            Err(e) => return Err(e),
        }
    }
}

fn expression(input: &str) -> IResult<&str, Expression> {
    let mut stack: Vec<Frame> = Vec::new();
    let mut min_bp: u8 = 0;
    let mut input = input;
    let mut lhs;

    loop {
        let (inp, polarity_ops) = many0(ws(alt((
            map(char('+'), |_| PolarityOp::Plus),
            map(char('-'), |_| PolarityOp::Minus),
        ))))(input)?;
        input = inp;

        if let Some(rest) = input.trim_start().strip_prefix('(') {
            stack.push(Frame::Paren {
                polarity_ops,
                min_bp,
            });
            input = rest;
            min_bp = 0;
            continue;
        }

        let term_expr;
        if let Ok((inp, l)) = literal(input) {
            term_expr = Expression::Term(Term::Literal(l));
            input = inp;
        } else if let Ok((inp, _)) = ws(tag("$this"))(input) {
            term_expr = Expression::Term(Term::Invocation(Invocation::This));
            input = inp;
        } else if let Ok((inp, _)) = ws(tag("$index"))(input) {
            term_expr = Expression::Term(Term::Invocation(Invocation::Index));
            input = inp;
        } else if let Ok((inp, _)) = ws(tag("$total"))(input) {
            term_expr = Expression::Term(Term::Invocation(Invocation::Total));
            input = inp;
        } else if let Ok((inp, name)) = identifier(input) {
            if let Some(rest) = inp.trim_start().strip_prefix('(') {
                if let Some(rest2) = rest.trim_start().strip_prefix(')') {
                    term_expr =
                        Expression::Term(Term::Invocation(Invocation::Function(name, vec![])));
                    input = rest2;
                } else {
                    stack.push(Frame::FuncArg {
                        base_expr: None,
                        name,
                        args: vec![],
                        polarity_ops,
                        min_bp,
                    });
                    input = rest;
                    min_bp = 0;
                    continue;
                }
            } else {
                term_expr = Expression::Term(Term::Invocation(Invocation::Member(name)));
                input = inp;
            }
        } else if let Ok((inp, ec)) = external_constant(input) {
            term_expr = Expression::Term(Term::ExternalConstant(ec));
            input = inp;
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alt,
            )));
        }

        let (inp, (postfix_expr, func_call)) = parse_postfix_chain(input, term_expr)?;
        input = inp;

        if let Some(func_name) = func_call {
            stack.push(Frame::FuncArg {
                base_expr: Some(postfix_expr),
                name: func_name,
                args: vec![],
                polarity_ops,
                min_bp,
            });
            min_bp = 0;
            continue;
        }

        lhs = polarity_ops
            .into_iter()
            .rev()
            .fold(postfix_expr, |acc, op| {
                Expression::Polarity(op, Box::new(acc))
            });

        'infix: loop {
            if let Some(rest) = input.trim_start().strip_prefix('[') {
                stack.push(Frame::Bracket { base: lhs, min_bp });
                input = rest;
                min_bp = 0;
                break;
            }

            if let Some((op, inp_after)) = peek_infix_op(input) {
                let (l_bp, r_bp) = op.binding_powers();
                if l_bp >= min_bp {
                    input = inp_after;

                    if let InfixOp::Type(type_op) = op {
                        let (inp, ts) = type_specifier(input)?;
                        input = inp;
                        lhs = Expression::Type(Box::new(lhs), type_op, ts);
                        continue 'infix;
                    }

                    stack.push(Frame::Infix {
                        left: lhs,
                        op,
                        min_bp,
                    });
                    min_bp = r_bp;
                    break;
                }
            }

            match stack.pop() {
                None => return Ok((input, lhs)),
                Some(Frame::Infix {
                    left,
                    op,
                    min_bp: saved_bp,
                }) => {
                    lhs = op.build_expression(left, lhs);
                    min_bp = saved_bp;
                }
                Some(Frame::Paren {
                    polarity_ops,
                    min_bp: saved_bp,
                }) => {
                    let (inp, _) = ws(char(')'))(input).map_err(|e| match e {
                        nom::Err::Error(e) => nom::Err::Failure(e),
                        other => other,
                    })?;
                    input = inp;
                    lhs = Expression::Term(Term::Parenthesized(Box::new(lhs)));

                    let (inp, (postfix_expr, func_call)) = parse_postfix_chain(input, lhs)?;
                    input = inp;

                    if let Some(func_name) = func_call {
                        stack.push(Frame::FuncArg {
                            base_expr: Some(postfix_expr),
                            name: func_name,
                            args: vec![],
                            polarity_ops,
                            min_bp: saved_bp,
                        });
                        min_bp = 0;
                        break;
                    }

                    lhs = postfix_expr;
                    for op in polarity_ops.into_iter().rev() {
                        lhs = Expression::Polarity(op, Box::new(lhs));
                    }
                    min_bp = saved_bp;
                }
                Some(Frame::Bracket {
                    base,
                    min_bp: saved_bp,
                }) => {
                    let (inp, _) = ws(char(']'))(input).map_err(|e| match e {
                        nom::Err::Error(e) => nom::Err::Failure(e),
                        other => other,
                    })?;
                    input = inp;
                    lhs = Expression::Indexer(Box::new(base), Box::new(lhs));

                    let (inp, (postfix_expr, func_call)) = parse_postfix_chain(input, lhs)?;
                    input = inp;

                    if let Some(func_name) = func_call {
                        stack.push(Frame::FuncArg {
                            base_expr: Some(postfix_expr),
                            name: func_name,
                            args: vec![],
                            polarity_ops: vec![],
                            min_bp: saved_bp,
                        });
                        min_bp = 0;
                        break;
                    }

                    lhs = postfix_expr;
                    min_bp = saved_bp;
                }
                Some(Frame::FuncArg {
                    base_expr,
                    name,
                    mut args,
                    polarity_ops,
                    min_bp: saved_bp,
                }) => {
                    args.push(lhs);

                    if let Some(rest) = input.trim_start().strip_prefix(',') {
                        input = rest;
                        stack.push(Frame::FuncArg {
                            base_expr,
                            name,
                            args,
                            polarity_ops,
                            min_bp: saved_bp,
                        });
                        min_bp = 0;
                        break;
                    }

                    let (inp, _) = ws(char(')'))(input).map_err(|e| match e {
                        nom::Err::Error(e) => nom::Err::Failure(e),
                        other => other,
                    })?;
                    input = inp;

                    let func_inv = Invocation::Function(name, args);
                    lhs = match base_expr {
                        Some(base) => Expression::Invocation(Box::new(base), func_inv),
                        None => Expression::Term(Term::Invocation(func_inv)),
                    };

                    let (inp, (postfix_expr, func_call)) = parse_postfix_chain(input, lhs)?;
                    input = inp;

                    if let Some(func_name) = func_call {
                        stack.push(Frame::FuncArg {
                            base_expr: Some(postfix_expr),
                            name: func_name,
                            args: vec![],
                            polarity_ops,
                            min_bp: saved_bp,
                        });
                        min_bp = 0;
                        break;
                    }

                    lhs = postfix_expr;
                    for op in polarity_ops.into_iter().rev() {
                        lhs = Expression::Polarity(op, Box::new(lhs));
                    }
                    min_bp = saved_bp;
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    Invalid(String),
    TrailingInput(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Invalid(msg) => write!(f, "parse error: {msg}"),
            ParserError::TrailingInput(trailing) => {
                write!(f, "unexpected trailing input: {trailing:?}")
            }
        }
    }
}

impl std::error::Error for ParserError {}

fn line_comment(input: &str) -> IResult<&str, ()> {
    map(pair(tag("//"), take_while(|c: char| c != '\n')), |_| ())(input)
}

fn block_comment(input: &str) -> IResult<&str, ()> {
    map(delimited(tag("/*"), take_until("*/"), tag("*/")), |_| ())(input)
}

fn fhirpath_string(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        char('\''),
        many0(alt((tag("\\'"), tag("\\\\"), recognize(none_of("'\\"))))),
        char('\''),
    ))(input)
}

fn strip_comments(input: &str) -> Result<String, ParserError> {
    let mut out = String::with_capacity(input.len());
    let mut remaining = input;

    while !remaining.is_empty() {
        if let Ok((rest, s)) = fhirpath_string(remaining) {
            out.push_str(s);
            remaining = rest;
        } else if let Ok((rest, _)) = line_comment(remaining) {
            remaining = rest;
        } else if let Ok((rest, _)) = block_comment(remaining) {
            out.push(' ');
            remaining = rest;
        } else if remaining.starts_with("/*") {
            return Err(ParserError::Invalid(
                "Unterminated block comment".to_string(),
            ));
        } else if let Some(ch) = remaining.chars().next() {
            out.push(ch);
            remaining = &remaining[ch.len_utf8()..];
        }
    }
    Ok(out)
}

pub fn parse(input: &str) -> Result<Expression, ParserError> {
    let stripped = strip_comments(input)?;
    match expression(&stripped) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(ParserError::TrailingInput(remaining.trim().to_string()))
            }
        }
        Err(e) => Err(ParserError::Invalid(e.to_string())),
    }
}
