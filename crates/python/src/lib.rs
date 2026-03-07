mod convert;

use convert::{py_to_value, value_to_py};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
#[pyo3(signature = (expression, data=None, constants=None))]
fn evaluate(
    py: Python<'_>,
    expression: &str,
    data: Option<&Bound<'_, PyAny>>,
    constants: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let parsed = parser::parse(expression).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Parse error: {e}"))
    })?;

    let data_value = match data {
        Some(obj) => py_to_value(obj)?,
        None => interpreter::Value::Null,
    };
    let mut context = interpreter::InterpreterContext::new(data_value);

    if let Some(const_dict) = constants {
        for (key, value) in const_dict.iter() {
            let key_str: String = key.extract().map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyTypeError, _>("Constant keys must be strings")
            })?;
            let val = py_to_value(&value)?;
            context = context.with_constant(key_str, val);
        }
    }

    let (result, _) = interpreter::interpret(&parsed, context).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Evaluation error: {e}"))
    })?;

    value_to_py(py, &result)
}

#[pymodule]
fn fhirpathrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    Ok(())
}
