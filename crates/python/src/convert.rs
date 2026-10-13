use interpreter::datetime::{format_date, format_datetime, format_time};
use interpreter::{Decimal, InterpreterError, ObjectSource, Value};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct Converter {
    error: Rc<RefCell<Option<PyErr>>>,
}

struct PyDictSource {
    dict: Py<PyDict>,
    converter: Converter,
}

enum PyToValueWork<'py> {
    Convert(Bound<'py, PyAny>),
    BuildList(usize, usize),
}

pub enum Conversion {
    Converted(Value),
    OverBudget,
}

impl Converter {
    pub fn shallow(&self, obj: &Bound<'_, PyAny>, budget: &Cell<usize>) -> PyResult<Conversion> {
        let Some(remaining) = budget.get().checked_sub(1) else {
            return Ok(Conversion::OverBudget);
        };
        budget.set(remaining);
        let Ok(list) = obj.cast::<PyList>() else {
            return self.leaf(obj).map(Conversion::Converted);
        };
        let mut stack: Vec<PyToValueWork<'_>> = Vec::with_capacity(list.len() + 1);
        stack.push(PyToValueWork::Convert(obj.clone()));
        let mut results: Vec<Value> = Vec::with_capacity(list.len() + 1);
        let mut expanding: HashSet<usize> = HashSet::new();

        while let Some(work) = stack.pop() {
            match work {
                PyToValueWork::Convert(obj) => match obj.cast::<PyList>() {
                    Ok(list) => {
                        let address = list.as_ptr().addr();
                        if !expanding.insert(address) {
                            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                                "Circular reference detected",
                            ));
                        }
                        let len = list.len();
                        let Some(remaining) = budget.get().checked_sub(len) else {
                            return Ok(Conversion::OverBudget);
                        };
                        budget.set(remaining);
                        stack.push(PyToValueWork::BuildList(len, address));
                        for i in (0..len).rev() {
                            let item = list.get_item(i).map_err(|e| {
                                PyErr::new::<pyo3::exceptions::PyIndexError, _>(e.to_string())
                            })?;
                            stack.push(PyToValueWork::Convert(item));
                        }
                    }
                    Err(_) => results.push(self.leaf(&obj)?),
                },
                PyToValueWork::BuildList(len, address) => {
                    expanding.remove(&address);
                    let start = results.len() - len;
                    let items: Vec<Value> = results.drain(start..).collect();
                    results.push(Value::collection(items));
                }
            }
        }

        results
            .pop()
            .map(Conversion::Converted)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Conversion failed"))
    }

    fn leaf(&self, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
        if obj.is_none() {
            Ok(Value::Null)
        } else if let Ok(b) = obj.cast::<PyBool>() {
            Ok(Value::Boolean(b.is_true()))
        } else if obj.is_instance_of::<PyInt>() {
            obj.extract::<i128>()
                .ok()
                .and_then(Decimal::from_i128)
                .map(|decimal| Value::Number(decimal, 0))
                .ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "integer is outside the supported Decimal range",
                    )
                })
        } else if obj.is_instance_of::<PyFloat>() {
            Decimal::from_f64(obj.extract()?)
                .map(Value::decimal)
                .ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "float is not a finite number inside the supported Decimal range",
                    )
                })
        } else if let Ok(s) = obj.cast::<PyString>() {
            s.to_str()
                .map(|s| Value::String(s.to_owned()))
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        } else if let Ok(dict) = obj.cast::<PyDict>() {
            Ok(Value::lazy_object(Box::new(PyDictSource {
                dict: dict.clone().unbind(),
                converter: self.clone(),
            })))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Unsupported type: {}",
                obj.get_type().name()?
            )))
        }
    }

    pub fn finish<T>(&self, outcome: Result<T, InterpreterError>) -> PyResult<T> {
        if let Some(err) = self.error.take() {
            return Err(err);
        }
        outcome.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Evaluation error: {e}"))
        })
    }

    fn record(&self, err: PyErr) {
        self.error.borrow_mut().get_or_insert(err);
    }
}

impl PyDictSource {
    fn convert_fields(
        &self,
        fields: &RefCell<HashMap<String, Value>>,
        budget: &Cell<usize>,
    ) -> PyResult<bool> {
        Python::attach(|py| {
            let dict = self.dict.bind(py);
            fields.borrow_mut().reserve(dict.len());
            let fits = Cell::new(true);
            dict.locked_for_each(|key, value| {
                let key = key
                    .cast::<PyString>()
                    .map_err(|_| {
                        PyErr::new::<pyo3::exceptions::PyTypeError, _>("Dict keys must be strings")
                    })?
                    .to_str()
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
                if !fields.borrow().contains_key(key) {
                    match self.converter.shallow(&value, budget)? {
                        Conversion::Converted(value) => {
                            fields.borrow_mut().insert(key.to_owned(), value);
                        }
                        Conversion::OverBudget => fits.set(false),
                    }
                }
                Ok(())
            })?;
            Ok(fits.get())
        })
    }
}

impl ObjectSource for PyDictSource {
    fn field(&self, key: &str) -> Option<Value> {
        Python::attach(|py| {
            self.dict
                .bind(py)
                .get_item(key)?
                .map(
                    |value| match self.converter.shallow(&value, &Cell::new(usize::MAX))? {
                        Conversion::Converted(value) => Ok(value),
                        Conversion::OverBudget => {
                            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                                "Conversion failed",
                            ))
                        }
                    },
                )
                .transpose()
        })
        .unwrap_or_else(|err| {
            self.converter.record(err);
            None
        })
    }

    fn fields(&self, loaded: HashMap<String, Value>) -> HashMap<String, Value> {
        let fields = RefCell::new(loaded);
        if let Err(err) = self.convert_fields(&fields, &Cell::new(usize::MAX)) {
            self.converter.record(err);
        }
        fields.into_inner()
    }

    fn try_fields(
        &self,
        loaded: &HashMap<String, Value>,
        budget: &Cell<usize>,
    ) -> Option<HashMap<String, Value>> {
        let fields = RefCell::new(loaded.clone());
        self.convert_fields(&fields, budget)
            .ok()?
            .then(|| fields.into_inner())
    }
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
                Value::Number(n, _) => {
                    if n.is_integer() {
                        results.push(n.trunc_to_i128().into_pyobject(py)?.into_any().unbind());
                    } else {
                        results.push(n.to_f64().into_pyobject(py)?.into_any().unbind());
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
                Value::Quantity(v, _, u, _) => {
                    let dict = PyDict::new(py);
                    dict.set_item("value", v.to_f64())?;
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
                    if let Some(source) = map.source().and_then(|source| {
                        (source as &dyn std::any::Any).downcast_ref::<PyDictSource>()
                    }) {
                        results.push(source.dict.clone_ref(py).into_any());
                    } else {
                        let entries: Vec<(&str, &Value)> =
                            map.iter().map(|(k, v)| (k.as_str(), v)).collect();
                        let keys: Vec<&str> = entries.iter().map(|(k, _)| *k).collect();
                        stack.push(ValueToPyWork::BuildDict(keys));
                        for (_, value) in entries.into_iter().rev() {
                            stack.push(ValueToPyWork::Convert(value));
                        }
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
