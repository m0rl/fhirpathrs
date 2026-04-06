use super::*;

#[test]
fn testmatchescasesensitive1() {
    let data = Value::object(HashMap::new());
    let expr = parse("'FHIR'.matches('FHIR')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatchescasesensitive2() {
    let data = Value::object(HashMap::new());
    let expr = parse("'FHIR'.matches('fhir')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testmatchesempty() {
    let data = Value::object(HashMap::new());
    let expr = parse("'FHIR'.matches({}).empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatchesempty2() {
    let data = Value::object(HashMap::new());
    let expr = parse("{}.matches('FHIR').empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatchesempty3() {
    let data = Value::object(HashMap::new());
    let expr = parse("{}.matches({}).empty() = true").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatchessinglelinemode1() {
    let data = Value::object(HashMap::new());
    let expr = parse("'A\nB'.matches('A.*B')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatcheswithinurl1() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matches('library')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testmatcheswithinurl2() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matches('Library')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatcheswithinurl3() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matches('^Library$')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testmatcheswithinurl1a() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matches('.*Library.*')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatcheswithinurl4() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matches('Measure')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testmatchesfullwithinurl1() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matchesFull('library')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testmatchesfullwithinurl3() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matchesFull('Library')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testmatchesfullwithinurl4() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matchesFull('^Library$')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn testmatchesfullwithinurl1a() {
    let data = Value::object(HashMap::new());
    let expr = parse("'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matchesFull('.*Library.*')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testmatchesfullwithinurl2() {
    let data = Value::object(HashMap::new());
    let expr = parse(
        "'http://fhir.org/guides/cqf/common/Library/FHIR-ModelInfo|4.0.1'.matchesFull('Measure')",
    )
    .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}
