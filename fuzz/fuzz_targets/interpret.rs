#![no_main]

use interpreter::{CollectingTraceHandler, Decimal, InterpreterContext, Value, interpret};
use libfuzzer_sys::Corpus;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static PATIENT: Value = Value::object(HashMap::from([
        ("resourceType".to_string(), Value::String("Patient".to_string())),
        ("id".to_string(), Value::String("example".to_string())),
        ("active".to_string(), Value::Boolean(true)),
        ("identifier".to_string(), Value::collection(vec![
            Value::object(HashMap::from([
                ("system".to_string(), Value::String("urn:oid:1.2.36.146.595.217.0.1".to_string())),
                ("value".to_string(), Value::String("12345".to_string())),
                ("period".to_string(), Value::object(HashMap::from([
                    ("start".to_string(), Value::String("2001-05-06".to_string())),
                    ("end".to_string(), Value::String("2021-05-06T23:59:59.999+14:00".to_string())),
                ]))),
            ])),
        ])),
        ("name".to_string(), Value::collection(vec![
            Value::object(HashMap::from([
                ("use".to_string(), Value::String("official".to_string())),
                ("family".to_string(), Value::String("Smith".to_string())),
                ("given".to_string(), Value::collection(vec![
                    Value::String("John".to_string()),
                    Value::String("张三".to_string()),
                    Value::Null,
                ])),
            ])),
            Value::object(HashMap::from([
                ("use".to_string(), Value::String("nickname".to_string())),
                ("given".to_string(), Value::collection(vec![Value::String("🙂 \u{0301}".to_string())])),
            ])),
        ])),
        ("gender".to_string(), Value::String("female".to_string())),
        ("birthDate".to_string(), Value::String("1974-12-25".to_string())),
        ("_birthDate".to_string(), Value::object(HashMap::from([
            ("extension".to_string(), Value::collection(vec![
                Value::object(HashMap::from([
                    ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string())),
                    ("valueDateTime".to_string(), Value::String("1974-12-25T14:35:45-05:00".to_string())),
                ])),
            ])),
        ]))),
        ("deceasedBoolean".to_string(), Value::Boolean(false)),
        ("multipleBirthInteger".to_string(), Value::Number(Decimal::from(i64::MAX), 0)),
        ("contained".to_string(), Value::collection(vec![
            Value::object(HashMap::from([
                ("resourceType".to_string(), Value::String("Observation".to_string())),
                ("id".to_string(), Value::String("weight".to_string())),
                ("status".to_string(), Value::String("final".to_string())),
                ("code".to_string(), Value::object(HashMap::from([
                    ("coding".to_string(), Value::collection(vec![
                        Value::object(HashMap::from([
                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                            ("code".to_string(), Value::String("29463-7".to_string())),
                            ("display".to_string(), Value::String("Body weight".to_string())),
                        ])),
                    ])),
                ]))),
                ("effectiveDateTime".to_string(), Value::String("2016-03-28T09:30:10.5+01:00".to_string())),
                ("issued".to_string(), Value::String("2016-03-28T08:30:10Z".to_string())),
                ("valueQuantity".to_string(), Value::object(HashMap::from([
                    ("value".to_string(), Value::number(185.25, 2)),
                    ("unit".to_string(), Value::String("lbs".to_string())),
                    ("system".to_string(), Value::String("http://unitsofmeasure.org".to_string())),
                    ("code".to_string(), Value::String("[lb_av]".to_string())),
                ]))),
                ("component".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("valueQuantity".to_string(), Value::object(HashMap::from([
                            ("value".to_string(), Value::number(-0.00000001, 8)),
                            ("unit".to_string(), Value::String("mg/dL".to_string())),
                            ("system".to_string(), Value::String("http://unitsofmeasure.org".to_string())),
                            ("code".to_string(), Value::String("mg/dL".to_string())),
                        ]))),
                    ])),
                    Value::object(HashMap::from([
                        ("valueTime".to_string(), Value::String("08:30:00.123".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("valueInteger".to_string(), Value::Number(Decimal::from(i64::MIN), 0)),
                    ])),
                ])),
            ])),
        ])),
    ]));
}

libfuzzer_sys::fuzz_target!(|expression: &str| -> Corpus {
    let parsed = match parser::parse(expression) {
        Ok(parsed) => parsed,
        Err(error) => {
            std::hint::black_box(error.to_string());
            return Corpus::Reject;
        }
    };
    let traces = Rc::new(CollectingTraceHandler::new());
    let context = InterpreterContext {
        evaluation_timestamp: Default::default(),
        repeat_item_limit: 10_000,
        repeat_comparison_limit: 1_000_000,
        ..InterpreterContext::new(PATIENT.with(Value::clone))
    }
    .with_trace_handler(traces.clone());
    let result = match interpret(&parsed, context) {
        Ok((result, _)) => result,
        Err(error) => {
            std::hint::black_box(error.to_string());
            return Corpus::Keep;
        }
    };
    drop(result.clone());
    let events = traces.events();
    let mut pending: Vec<&Value> = events
        .iter()
        .map(|event| &event.value)
        .chain([&result])
        .collect();
    while let Some(value) = pending.pop() {
        match value {
            Value::Collection(items) => pending.extend(items.iter()),
            Value::Object(fields) => pending.extend(fields.values()),
            Value::String(text) | Value::Quantity(_, _, text, _) => text.bytes().for_each(|byte| {
                std::hint::black_box(byte);
            }),
            Value::Null
            | Value::Boolean(_)
            | Value::Number(..)
            | Value::Date(..)
            | Value::DateTime(..)
            | Value::Time(..) => {}
        }
    }
    Corpus::Keep
});
