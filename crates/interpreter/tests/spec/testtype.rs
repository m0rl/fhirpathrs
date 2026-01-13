use super::*;

#[test]
fn testtype1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.type().namespace = 'System'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype1a() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("1.type().name = 'Integer'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.type().namespace = 'System'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype2a() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("'1'.type().name = 'String'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.type().namespace = 'System'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.type().name = 'Boolean'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype5() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.is(Boolean)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype6() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true.is(System.Boolean)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype7() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true is Boolean").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype8() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("true is System.Boolean").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR primitive type provenance not tracked
#[test]
fn testtype9() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.active.type().namespace = 'FHIR'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR primitive type provenance not tracked
#[test]
fn testtype10() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.active.type().name = 'boolean'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR primitive type provenance not tracked
#[test]
fn testtype11() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.active.is(boolean)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR primitive type provenance not tracked
#[test]
fn testtype12() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.active.is(Boolean).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR primitive type provenance not tracked
#[test]
fn testtype13() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.active.is(FHIR.boolean)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR primitive type provenance not tracked
#[test]
fn testtype14() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.active.is(System.Boolean).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype15() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.type().namespace = 'FHIR'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype16() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.type().name = 'Patient'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype17() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.is(Patient)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype18() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.is(FHIR.Patient)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype19() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.is(FHIR.`Patient`)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype20() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.ofType(Patient).type().name").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("Patient".to_string())]);
}

#[test]
fn testtype21() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.ofType(FHIR.Patient).type().name").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("Patient".to_string())]);
}

#[test]
fn testtype22() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.is(System.Patient).not()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtype23() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.ofType(FHIR.`Patient`).type().name").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("Patient".to_string())]);
}

#[test]
fn testtypea1() {
    let data = fixtures::PARAMETERS_EXAMPLE_TYPES.with(Value::clone);
    let expr = parse("Parameters.parameter[0].value.is(FHIR.string)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtypea2() {
    let data = fixtures::PARAMETERS_EXAMPLE_TYPES.with(Value::clone);
    let expr = parse("Parameters.parameter[1].value.is(FHIR.integer)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR type hierarchy uuid/uri not implemented
#[test]
fn testtypea3() {
    let data = fixtures::PARAMETERS_EXAMPLE_TYPES.with(Value::clone);
    let expr = parse("Parameters.parameter[2].value.is(FHIR.uuid)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // FHIR type hierarchy uuid/uri not implemented
#[test]
fn testtypea4() {
    let data = fixtures::PARAMETERS_EXAMPLE_TYPES.with(Value::clone);
    let expr = parse("Parameters.parameter[2].value.is(FHIR.uri)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testtypea() {
    let data = fixtures::PARAMETERS_EXAMPLE_TYPES.with(Value::clone);
    let expr = parse("Parameters.parameter[3].value.is(FHIR.decimal)").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
