use super::*;

#[ignore] // not implemented: TerminologyTests
#[test]
fn txtest01() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("%terminologies.expand('http://hl7.org/fhir/ValueSet/administrative-gender').expansion.contains.count()").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Number(4.0)]);
}

#[ignore] // not implemented: TerminologyTests
#[test]
fn txtest02() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("%terminologies.validateVS('http://hl7.org/fhir/ValueSet/administrative-gender', $this.gender).parameter.where(name = 'result').value").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: TerminologyTests
#[test]
fn txtest03() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("%terminologies.translate('http://hl7.org/fhir/ConceptMap/cm-address-use-v2', $this.address.use).parameter.where(name = 'match').part.where(name = 'concept').value.code").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("H".to_string())]);
}
