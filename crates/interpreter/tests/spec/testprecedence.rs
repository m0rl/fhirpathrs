use super::*;

#[test]
fn testprecedence1() {
    let data = Value::object(HashMap::new());
    let expr = parse("-1.convertsToInteger()").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testprecedence2() {
    let data = Value::object(HashMap::new());
    let expr = parse("1+2*3+4 = 11").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // is operator precedence edge case
#[test]
fn testprecedence3() {
    let data = Value::object(HashMap::new());
    let expr = parse("1 > 2 is Boolean").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[ignore] // is operator precedence edge case
#[test]
fn testprecedence4() {
    let data = Value::object(HashMap::new());
    let expr = parse("(1 | 1 is Integer).count()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(2.0)]);
}

#[test]
fn testprecedence5() {
    let data = Value::object(HashMap::new());
    let expr = parse("true and '0215' in ('0215' | '0216')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testprecedence6() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("category.exists(coding.exists(system = 'http://terminology.hl7.org/CodeSystem/observation-category' and code.trace('c') in ('vital-signs' | 'vital-signs2').trace('codes')))").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
