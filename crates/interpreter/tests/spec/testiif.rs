use super::*;

#[test]
fn testiif1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(Patient.name.exists(), 'named', 'unnamed') = 'named'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testiif2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(Patient.name.empty(), 'unnamed', 'named') = 'named'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testiif3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(true, true, (1 | 2).toString())").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testiif4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(false, (1 | 2).toString(), true)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testiif5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif(false, 'true-result').empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testiif6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("iif('non boolean criteria', 'true-result', 'false-result')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("true-result".to_string())]);
}

#[test]
fn testiif7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("{}.iif(true, 'true-result', 'false-result')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("true-result".to_string())]);
}

#[test]
fn testiif8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("('item').iif(true, 'true-result', 'false-result')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("true-result".to_string())]);
}

#[test]
fn testiif9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("('context').iif(true, select($this), 'false-result')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("context".to_string())]);
}

#[ignore] // iif() multi-item collection error not enforced
#[test]
fn testiif10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("('item1' | 'item2').iif(true, 'true-result', 'false-result')").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[test]
fn testiif11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("('context').iif($this = 'context','true-result', 'false-result')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("true-result".to_string())]);
}

#[test]
fn testiif12() {
    let data = fixtures::PATIENT_EXAMPLE_NAME.with(Value::clone);
    let expr = parse("Patient.name.first().iif(text.exists(), text, family+given.first())").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("Pater J Chalmers".to_string())]);
}
