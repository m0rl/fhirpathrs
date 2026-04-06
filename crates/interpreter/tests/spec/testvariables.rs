use super::*;

#[test]
fn testvariables1() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("%sct = 'http://snomed.info/sct'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testvariables2() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("%loinc = 'http://loinc.org'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[test]
fn testvariables3() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr = parse("%ucum = 'http://unitsofmeasure.org'").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // %vs-administrative-gender not a built-in variable
#[test]
fn testvariables4() {
    let data = fixtures::PATIENT_EXAMPLE.with(Value::clone);
    let expr =
        parse("%`vs-administrative-gender` = 'http://hl7.org/fhir/ValueSet/administrative-gender'")
            .expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}
