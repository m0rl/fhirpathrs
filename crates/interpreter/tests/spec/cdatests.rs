use super::*;

#[ignore] // not implemented: cdaTests
#[test]
fn testhastemplateid1() {
    let data = Value::object(HashMap::new());
    let expr = parse("hasTemplateIdOf('http://hl7.org/cda/us/ccda/StructureDefinition/ContinuityofCareDocumentCCD')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: cdaTests
#[test]
fn testhastemplateid2() {
    let data = Value::object(HashMap::new());
    let expr = parse("ClinicalDocument.hasTemplateIdOf('http://hl7.org/cda/us/ccda/StructureDefinition/ContinuityofCareDocumentCCD')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // not implemented: cdaTests
#[test]
fn testhastemplateid3() {
    let data = Value::object(HashMap::new());
    let expr = parse("recordTarget.patientRole.hasTemplateIdOf('http://hl7.org/cda/us/ccda/StructureDefinition/ContinuityofCareDocumentCCD')").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(false)]);
}
