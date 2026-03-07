use interpreter::Value;
use interpreter::datetime::{format_date, format_datetime, format_time};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};
use std::collections::HashMap;

enum PyToValueWork<'py> {
    Convert(Bound<'py, PyAny>),
    BuildList(usize),
    BuildDict(Vec<String>),
}

pub fn py_to_value(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    let mut stack: Vec<PyToValueWork<'_>> = Vec::with_capacity(32);
    stack.push(PyToValueWork::Convert(obj.clone()));
    let mut results: Vec<Value> = Vec::with_capacity(32);

    while let Some(work) = stack.pop() {
        match work {
            PyToValueWork::Convert(obj) => {
                if obj.is_none() {
                    results.push(Value::Null);
                } else if let Ok(b) = obj.cast::<PyBool>() {
                    results.push(Value::Boolean(b.is_true()));
                } else if obj.is_instance_of::<PyInt>() || obj.is_instance_of::<PyFloat>() {
                    let n: f64 = obj.extract()?;
                    results.push(Value::Number(n));
                } else if let Ok(s) = obj.cast::<PyString>() {
                    let rust_str = s.to_str().map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
                    })?;
                    results.push(Value::String(rust_str.to_owned()));
                } else if let Ok(list) = obj.cast::<PyList>() {
                    let len = list.len();
                    stack.push(PyToValueWork::BuildList(len));
                    for i in (0..len).rev() {
                        let item = list.get_item(i).map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyIndexError, _>(e.to_string())
                        })?;
                        stack.push(PyToValueWork::Convert(item));
                    }
                } else if let Ok(dict) = obj.cast::<PyDict>() {
                    let len = dict.len();
                    let mut keys = Vec::with_capacity(len);
                    stack.reserve(len + 1);
                    // Collect keys first, then push values in reverse order.
                    // We need two passes because dict.iter() yields items in
                    // forward order but the stack needs reverse order.
                    let items: Vec<(Bound<'_, PyAny>, Bound<'_, PyAny>)> = dict.iter().collect();
                    for (key, _) in &items {
                        let key_str = key
                            .cast::<PyString>()
                            .map_err(|_| {
                                PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                                    "Dict keys must be strings",
                                )
                            })?
                            .to_str()
                            .map_err(|e| {
                                PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
                            })?
                            .to_owned();
                        keys.push(key_str);
                    }
                    stack.push(PyToValueWork::BuildDict(keys));
                    for (_, value) in items.into_iter().rev() {
                        stack.push(PyToValueWork::Convert(value));
                    }
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                        "Unsupported type: {}",
                        obj.get_type().name()?
                    )));
                }
            }
            PyToValueWork::BuildList(len) => {
                let start = results.len() - len;
                let items: Vec<Value> = results.drain(start..).collect();
                results.push(Value::collection(items));
            }
            PyToValueWork::BuildDict(keys) => {
                let start = results.len() - keys.len();
                let values: Vec<Value> = results.drain(start..).collect();
                let mut map = HashMap::with_capacity(keys.len());
                for (k, v) in keys.into_iter().zip(values) {
                    map.insert(k, v);
                }
                results.push(Value::object(map));
            }
        }
    }

    results
        .pop()
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Conversion failed"))
}

enum ValueToPyWork<'a> {
    Convert(&'a Value),
    BuildList(usize),
    BuildDict(Vec<&'a str>),
}

pub fn value_to_py(py: Python<'_>, value: &Value) -> PyResult<Py<PyAny>> {
    let mut stack: Vec<ValueToPyWork<'_>> = Vec::with_capacity(32);
    stack.push(ValueToPyWork::Convert(value));
    let mut results: Vec<Py<PyAny>> = Vec::with_capacity(32);

    while let Some(work) = stack.pop() {
        match work {
            ValueToPyWork::Convert(val) => match val {
                Value::Null => results.push(py.None()),
                Value::Boolean(b) => {
                    results.push((*b).into_pyobject(py)?.to_owned().into_any().unbind());
                }
                Value::String(s) => {
                    results.push(s.as_str().into_pyobject(py)?.into_any().unbind());
                }
                Value::Number(n) => {
                    if n.fract() == 0.0 {
                        #[allow(clippy::cast_possible_truncation)]
                        let i = *n as i64;
                        results.push(i.into_pyobject(py)?.into_any().unbind());
                    } else {
                        results.push((*n).into_pyobject(py)?.into_any().unbind());
                    }
                }
                Value::Date(d, p) => {
                    let s = format_date(*d, *p);
                    results.push(s.as_str().into_pyobject(py)?.into_any().unbind());
                }
                Value::DateTime(dt, p, tz) => {
                    let s = format_datetime(*dt, *p, tz);
                    results.push(s.as_str().into_pyobject(py)?.into_any().unbind());
                }
                Value::Time(t, p) => {
                    let s = format_time(*t, *p);
                    results.push(s.as_str().into_pyobject(py)?.into_any().unbind());
                }
                Value::Quantity(v, u, _) => {
                    let dict = PyDict::new(py);
                    dict.set_item("value", v)?;
                    dict.set_item("unit", u)?;
                    results.push(dict.unbind().into_any());
                }
                Value::Collection(items) => {
                    let len = items.len();
                    stack.push(ValueToPyWork::BuildList(len));
                    for item in items.iter().rev() {
                        stack.push(ValueToPyWork::Convert(item));
                    }
                }
                Value::Object(map) => {
                    let entries: Vec<(&str, &Value)> =
                        map.iter().map(|(k, v)| (k.as_str(), v)).collect();
                    let keys: Vec<&str> = entries.iter().map(|(k, _)| *k).collect();
                    stack.push(ValueToPyWork::BuildDict(keys));
                    for (_, value) in entries.into_iter().rev() {
                        stack.push(ValueToPyWork::Convert(value));
                    }
                }
            },
            ValueToPyWork::BuildList(len) => {
                let start = results.len() - len;
                let items: Vec<Py<PyAny>> = results.drain(start..).collect();
                let list = PyList::new(py, &items)?;
                results.push(list.unbind().into_any());
            }
            ValueToPyWork::BuildDict(keys) => {
                let start = results.len() - keys.len();
                let py_values: Vec<Py<PyAny>> = results.drain(start..).collect();
                let dict = PyDict::new(py);
                for (key, py_val) in keys.iter().zip(py_values) {
                    dict.set_item(key, py_val)?;
                }
                results.push(dict.unbind().into_any());
            }
        }
    }

    results
        .pop()
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Conversion failed"))
}
