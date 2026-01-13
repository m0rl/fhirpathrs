use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TraceEvent {
    pub name: String,
    pub value: Value,
}

pub trait TraceHandler {
    fn on_trace(&self, event: &TraceEvent);
}

pub type SharedTraceHandler = Rc<dyn TraceHandler>;

pub struct CollectingTraceHandler {
    events: RefCell<Vec<TraceEvent>>,
}

impl CollectingTraceHandler {
    pub fn new() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn events(&self) -> Vec<TraceEvent> {
        self.events.borrow().clone()
    }

    pub fn clear(&self) {
        self.events.borrow_mut().clear();
    }
}

impl Default for CollectingTraceHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceHandler for CollectingTraceHandler {
    fn on_trace(&self, event: &TraceEvent) {
        self.events.borrow_mut().push(event.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collecting_handler_collects_events() {
        let handler = CollectingTraceHandler::new();

        handler.on_trace(&TraceEvent {
            name: "test".to_string(),
            value: Value::Number(42.0),
        });

        let events = handler.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "test");
    }

    #[test]
    fn test_collecting_handler_clear() {
        let handler = CollectingTraceHandler::new();

        handler.on_trace(&TraceEvent {
            name: "test".to_string(),
            value: Value::Boolean(true),
        });

        assert_eq!(handler.events().len(), 1);
        handler.clear();
        assert_eq!(handler.events().len(), 0);
    }
}
