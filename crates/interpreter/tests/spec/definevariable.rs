use super::*;

#[test]
fn definevariable1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('v1', 'value1').select(%v1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("value1".to_string())]);
}

#[test]
fn definevariable2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('n1', name.first()).select(%n1.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn definevariable3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr =
        parse("defineVariable('n1', name.first()).select(%n1.given).first()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("Peter".to_string())]);
}

#[test]
fn definevariable4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('n1', name.first()).select(%n1.given) | defineVariable('n1', name.skip(1).first()).select(%n1.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("Peter".to_string()),
        Value::String("James".to_string()),
        Value::String("Jim".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn definevariable5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('n1', name.first()).where(active.not()) | defineVariable('n1', name.skip(1).first()).select(%n1.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("Jim".to_string())]);
}

#[test]
fn definevariable6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('n1', name.first()).select(id & '-' & %n1.given.join('|')) | defineVariable('n2', name.skip(1).first()).select(%n2.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("example-Peter|James".to_string()),
        Value::String("Jim".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn definevariable7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('n1', name.first()).active | defineVariable('n2', name.skip(1).first()).select(%n2.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![Value::Boolean(true), Value::String("Jim".to_string())];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn definevariable8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('v1', 'value1').select(%v1).trace('data').defineVariable('v2', 'value2').select($this & ':' & %v1 & '-' & %v2) | defineVariable('v3', 'value3').select(%v3)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("value1:value1-value2".to_string()),
        Value::String("value3".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[ignore] // semantic validation not implemented
#[test]
fn definevariable9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('n1', name.first()).active | defineVariable('n2', name.skip(1).first()).select(%n1.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // semantic validation not implemented
#[test]
fn definevariable10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("select(%fam.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // semantic validation not implemented
#[test]
fn dvredefiningvariablethrowserror() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('v1').defineVariable('v1').select(%v1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // semantic validation not implemented
#[test]
fn definevariable12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.defineVariable('n1', first()).active | Patient.name.defineVariable('n2', skip(1).first()).select(%n1.given)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[test]
fn definevariable13() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.defineVariable('n2', skip(1).first()).defineVariable('res', %n2.given+%n2.given).select(%res)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("JimJim".to_string()),
        Value::String("JimJim".to_string()),
        Value::String("JimJim".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn definevariable14() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.defineVariable('n1', first()).select(%n1).exists() | Patient.name.defineVariable('n2', skip(1).first()).defineVariable('res', %n2.given+%n2.given).select(%res)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![Value::Boolean(true), Value::String("JimJim".to_string())];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[test]
fn definevariable15() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('root', 'r1-').select(defineVariable('v1', 'v1').defineVariable('v2', 'v2').select(%v1 | %v2)).select(%root & $this)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("r1-v1".to_string()),
        Value::String("r1-v2".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[ignore] // semantic validation not implemented
#[test]
fn definevariable16() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('root', 'r1-').select(defineVariable('v1', 'v1').defineVariable('v2', 'v2').select(%v1 | %v2)).select(%root & $this & %v1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // semantic validation not implemented
#[test]
fn dvcantoverwritesystemvar() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('context', 'oops')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // R5 field naming mismatch in conceptmap fixture
#[test]
fn dvconceptmapexample() {
    let data = fixtures::CONCEPTMAP_EXAMPLE.with(Value::clone);
    let expr = parse("\ngroup.select(\ndefineVariable('grp')\n.element\n.select(\ndefineVariable('ele')\n.target\n.select(%grp.source & '|' & %ele.code & ' ' & relationship & ' ' & %grp.target & '|' & code)\n)\n)\n.trace('all')\n.isDistinct()\n").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[test]
fn definevariable19() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable(defineVariable('param','ppp').select(%param), defineVariable('param','value').select(%param)).select(%ppp)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("value".to_string())]);
}

#[test]
fn dvparametersdontcolide() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'aaa'.replace(defineVariable('param', 'aaa').select(%param), defineVariable('param','bbb').select(%param))").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("bbb".to_string())]);
}

#[ignore] // semantic validation not implemented
#[test]
fn dvusageoutsidescopethrows() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("defineVariable('n1', 'v1').active | defineVariable('n2', 'v2').select(%n1)")
        .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}
