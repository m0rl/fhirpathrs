use super::*;

#[test]
fn testcomment1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2 + 2 // This is a single-line comment + 4").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(4.0)]);
}

#[test]
fn testcomment2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("// This is a multi line comment using // that\n// should not fail during parsing\n2+2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(4.0)]);
}

#[test]
fn testcomment3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2 + 2\n/*\nThis is a multi-line comment\nAny text enclosed within is ignored\n+2\n*/").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(4.0)]);
}

#[test]
fn testcomment4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2 + 2\n/*\nThis is a multi-line comment\nAny text enclosed within is ignored\n*/\n+2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(6.0)]);
}

#[test]
fn testcomment5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("/*\nThis is a multi-line comment\nAny text enclosed within is ignored\n*/\n2+2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(4.0)]);
}

#[test]
fn testcomment6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2 // comment\n/ 2").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(1.0_f64)]);
}

#[test]
fn testcomment7() {
    assert!(parse("2 + 2 /").is_err());
}

#[test]
fn testcomment8() {
    assert!(parse("2 + 2 /* not finished").is_err());
}

#[test]
fn testcomment9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("2 + /* inline $@%^+ * */ 2 = 4").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
