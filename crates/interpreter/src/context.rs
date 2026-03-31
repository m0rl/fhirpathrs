use crate::trace::SharedTraceHandler;
use crate::value::Value;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct InterpreterContext {
    pub data: Value,
    pub root_resource: Value,
    pub external_constants: Rc<HashMap<String, Value>>,
    pub this_context: Option<Value>,
    pub index_context: Option<usize>,
    pub total_context: Option<Value>,
    pub evaluation_timestamp: DateTime<Utc>,
    pub trace_handler: Option<SharedTraceHandler>,
}

impl InterpreterContext {
    pub fn new(data: Value) -> Self {
        Self {
            root_resource: data.clone(),
            data,
            external_constants: Rc::new(HashMap::new()),
            this_context: None,
            index_context: None,
            total_context: None,
            evaluation_timestamp: Utc::now(),
            trace_handler: None,
        }
    }

    pub fn with_trace_handler(mut self, handler: SharedTraceHandler) -> Self {
        self.trace_handler = Some(handler);
        self
    }

    pub fn with_this(mut self, this: Value) -> Self {
        self.this_context = Some(this.clone());
        self.data = this;
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index_context = Some(index);
        self
    }

    pub fn with_total(mut self, total: Value) -> Self {
        self.total_context = Some(total);
        self
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn with_total_count(self, count: usize) -> Self {
        self.with_total(Value::Number(count as f64, 0))
    }

    pub fn with_constant(mut self, name: String, value: Value) -> Self {
        Rc::make_mut(&mut self.external_constants).insert(name, value);
        self
    }
}
