use crate::context::InterpreterContext;
use crate::value::Value;

pub(crate) fn resolve_predefined_constant(
    name: &str,
    context: &InterpreterContext,
) -> Option<Value> {
    match name {
        "context" | "resource" | "rootResource" => Some(context.root_resource.clone()),
        "ucum" => Some(Value::String("http://unitsofmeasure.org".to_string())),
        "sct" => Some(Value::String("http://snomed.info/sct".to_string())),
        "loinc" => Some(Value::String("http://loinc.org".to_string())),
        _ => None,
    }
}

pub(crate) fn is_system_variable(name: &str) -> bool {
    matches!(
        name,
        "context" | "resource" | "rootResource" | "ucum" | "sct" | "loinc"
    )
}
