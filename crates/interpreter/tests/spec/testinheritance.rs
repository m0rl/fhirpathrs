use super::*;

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.is(code)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.is(string)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.is(id)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction4() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.url.is(uri)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction5() {
    let data = fixtures::QUESTIONNAIRE_EXAMPLE.with(Value::clone);
    let expr = parse("Questionnaire.url.is(url)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction6() {
    let data = fixtures::VALUESET_EXAMPLE_EXPANSION.with(Value::clone);
    let expr = parse("ValueSet.version.is(string)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction7() {
    let data = fixtures::VALUESET_EXAMPLE_EXPANSION.with(Value::clone);
    let expr = parse("ValueSet.version.is(code)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction8() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.extension('http://example.com/fhir/StructureDefinition/patient-age').value is Age").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction9() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.extension('http://example.com/fhir/StructureDefinition/patient-age').value is Quantity").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathisfunction10() {
    let data = fixtures::OBSERVATION_EXAMPLE.with(Value::clone);
    let expr = parse("Observation.extension('http://example.com/fhir/StructureDefinition/patient-age').value is Duration").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.as(string)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.as(code)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("male".to_string())]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction13() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.as(id)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction14() {
    let data = fixtures::VALUESET_EXAMPLE_EXPANSION.with(Value::clone);
    let expr = parse("ValueSet.version.as(string)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("20150622".to_string())]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction15() {
    let data = fixtures::VALUESET_EXAMPLE_EXPANSION.with(Value::clone);
    let expr = parse("ValueSet.version.as(code)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction16() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.ofType(string)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction17() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.ofType(code)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("male".to_string())]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction18() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.ofType(id)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction19() {
    let data = fixtures::VALUESET_EXAMPLE_EXPANSION.with(Value::clone);
    let expr = parse("ValueSet.version.ofType(string)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("20150622".to_string())]);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction20() {
    let data = fixtures::VALUESET_EXAMPLE_EXPANSION.with(Value::clone);
    let expr = parse("ValueSet.version.ofType(code)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert!(actual.is_empty(), "expected empty, got {:?}", actual);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction21() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.as(HumanName).use").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction22() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.name.ofType(HumanName).use").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::String("official".to_string()),
        Value::String("usual".to_string()),
        Value::String("maiden".to_string()),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction23() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.as(string1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}

#[ignore] // not implemented: testInheritance
#[test]
fn testfhirpathasfunction24() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.gender.ofType(string1)").expect("parse");
    let ctx = InterpreterContext::new(data);
    assert!(interpret(&expr, ctx).is_err());
}
