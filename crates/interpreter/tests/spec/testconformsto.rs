use super::*;

#[ignore] // not implemented: testConformsTo
#[test]
fn testconformsto1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr =
        parse("conformsTo('http://hl7.org/fhir/StructureDefinition/Patient')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testConformsTo
#[test]
fn testconformsto2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr =
        parse("conformsTo('http://hl7.org/fhir/StructureDefinition/Person')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[ignore] // not implemented: testConformsTo
#[test]
fn testconformsto3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("conformsTo('http://trash')").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}
