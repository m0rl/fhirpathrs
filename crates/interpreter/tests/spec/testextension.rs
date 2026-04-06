use super::*;

#[ignore] // not implemented: testExtension
#[test]
fn testextension1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate.extension('http://hl7.org/fhir/StructureDefinition/patient-birthTime').exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testExtension
#[test]
fn testextension2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr =
        parse("Patient.birthDate.extension(%`ext-patient-birthTime`).exists()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: testExtension
#[test]
fn testextension3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("Patient.birthDate.extension('http://hl7.org/fhir/StructureDefinition/patient-birthTime1').empty()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
