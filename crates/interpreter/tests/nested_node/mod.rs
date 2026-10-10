use interpreter::{ObjectSource, Value};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct NestedNodeSource {
    depth: u32,
    field_loads: Rc<Cell<usize>>,
}

impl NestedNodeSource {
    pub fn nested(depth: u32) -> (Value, Rc<Cell<usize>>) {
        let field_loads = Rc::new(Cell::new(0));
        let source = NestedNodeSource {
            depth,
            field_loads: field_loads.clone(),
        };
        (Value::lazy_object(Box::new(source)), field_loads)
    }

    fn load(&self, key: &str) -> Option<Value> {
        let value = match key {
            "resourceType" => Value::String("Node".to_string()),
            "value" => Value::number(f64::from(self.depth), 0),
            "item" if self.depth > 0 => Value::lazy_object(Box::new(NestedNodeSource {
                depth: self.depth - 1,
                field_loads: self.field_loads.clone(),
            })),
            _ => return None,
        };
        self.field_loads.set(self.field_loads.get() + 1);
        Some(value)
    }
}

impl ObjectSource for NestedNodeSource {
    fn field(&self, key: &str) -> Option<Value> {
        self.load(key)
    }

    fn fields(&self, mut loaded: HashMap<String, Value>) -> HashMap<String, Value> {
        for key in ["resourceType", "value", "item"] {
            if !loaded.contains_key(key)
                && let Some(value) = self.load(key)
            {
                loaded.insert(key.to_string(), value);
            }
        }
        loaded
    }
}
