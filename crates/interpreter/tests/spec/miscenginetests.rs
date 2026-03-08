use super::*;

#[test]
fn testcontainedid() {
    let data = fixtures::PATIENT_CONTAINER_EXAMPLE.with(Value::clone);
    let expr = parse("contained.id").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::String("1".to_string())]);
}

#[test]
fn testmultipleresolve() {
    let data = Value::object(HashMap::new());
    let expr = parse("\ncomposition.exists()\nimplies\n(\ncomposition.resolve().section.entry.reference.where(resolve() is Observation)\n.where($this in (%resource.result.reference | %resource.result.reference.resolve().hasMember.reference)).exists()\n)\n").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    assert_eq!(actual, vec![Value::Boolean(true)]);
}

#[ignore] // fixture patient-name-extensions.json not available
#[test]
fn testprimitiveextensions() {
    let data = Value::object(HashMap::new());
    let expr = parse("Patient.name.given.select($this.hasValue())").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::Boolean(false),
        Value::Boolean(true),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}

#[ignore] // mode=element not supported
#[test]
fn testprimitiveextensionselement() {
    let data = Value::object(HashMap::new());
    let expr = parse("Patient.name.given.select($this.hasValue())").expect("parse");
    let ctx = InterpreterContext::new(data);
    let (result, _) = interpret(&expr, ctx).expect("interpret");
    let actual = result.to_vec();
    let expected = vec![
        Value::Boolean(false),
        Value::Boolean(true),
    ];
    assert_eq!(actual, expected, "results: {:?}", actual);
}
