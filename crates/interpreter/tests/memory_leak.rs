#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use interpreter::Value;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::collections::HashMap;

struct TrackingAllocator;

thread_local! {
    static LIVE_BYTES: Cell<isize> = const { Cell::new(0) };
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        LIVE_BYTES.with(|c| c.set(c.get() + layout.size() as isize));
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE_BYTES.with(|c| c.set(c.get() - layout.size() as isize));
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOC: TrackingAllocator = TrackingAllocator;

fn assert_no_leak<F: FnOnce()>(f: F) {
    let before = LIVE_BYTES.with(Cell::get);
    f();
    let after = LIVE_BYTES.with(Cell::get);
    let leaked = after - before;
    assert_eq!(leaked, 0, "Leaked {leaked} bytes");
}

#[test]
fn no_leak_deep_collections() {
    assert_no_leak(|| {
        let mut val = Value::String("leaf".to_string());
        for _ in 0..10_000 {
            val = Value::collection(vec![val]);
        }
    });
}

#[test]
fn no_leak_deep_objects() {
    assert_no_leak(|| {
        let mut val = Value::Number(42.0);
        for _ in 0..10_000 {
            val = Value::object(HashMap::from([("item".to_string(), val)]));
        }
    });
}

#[test]
fn no_leak_mixed_collection_object_nesting() {
    assert_no_leak(|| {
        let mut val = Value::Boolean(true);
        for i in 0..5_000 {
            val = if i % 2 == 0 {
                Value::collection(vec![val])
            } else {
                Value::object(HashMap::from([("x".to_string(), val)]))
            };
        }
    });
}

#[test]
fn no_leak_wide_collection_of_strings() {
    assert_no_leak(|| {
        let items: Vec<Value> = (0..10_000)
            .map(|i| Value::String(format!("item_{i}")))
            .collect();
        Value::collection(items);
    });
}

#[test]
fn no_leak_wide_object() {
    assert_no_leak(|| {
        let map: HashMap<String, Value> = (0..1_000)
            .map(|i| (format!("key_{i}"), Value::Number(i as f64)))
            .collect();
        Value::object(map);
    });
}

#[test]
fn no_leak_shared_rc_clones() {
    assert_no_leak(|| {
        let shared = Value::collection(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);
        let c1 = shared.clone();
        let c2 = shared.clone();
        Value::collection(vec![shared, c1, c2]);
    });
}

#[test]
fn no_leak_clone_drop_order() {
    assert_no_leak(|| {
        let original = Value::collection(vec![
            Value::object(HashMap::from([
                ("name".to_string(), Value::String("test".to_string())),
                ("value".to_string(), Value::Number(42.0)),
            ])),
            Value::Quantity(1.0, "mg".to_string(), None),
        ]);
        let _clone = original.clone();
    });
}

#[test]
fn no_leak_binary_tree() {
    assert_no_leak(|| {
        let depth = 15;
        let leaf_count = 1usize << depth;
        let mut level: Vec<Value> = (0..leaf_count).map(|_| Value::Number(1.0)).collect();
        for _ in 0..depth {
            let mut next = Vec::with_capacity(level.len() / 2);
            let mut iter = level.into_iter();
            while let (Some(l), Some(r)) = (iter.next(), iter.next()) {
                next.push(Value::collection(vec![l, r]));
            }
            level = next;
        }
    });
}

#[test]
fn no_leak_quantities_in_collection() {
    assert_no_leak(|| {
        let items: Vec<Value> = (0..1_000)
            .map(|i| Value::Quantity(i as f64, format!("unit_{i}"), None))
            .collect();
        Value::collection(items);
    });
}

#[test]
fn no_leak_nested_objects_with_multiple_string_keys() {
    assert_no_leak(|| {
        let mut val = Value::String("deep".to_string());
        for i in 0..5_000 {
            val = Value::object(HashMap::from([
                (format!("key_{i}"), val),
                (format!("extra_{i}"), Value::String(format!("val_{i}"))),
            ]));
        }
    });
}

#[test]
fn no_leak_empty_containers() {
    assert_no_leak(|| {
        Value::collection(vec![]);
        Value::object(HashMap::new());
    });
}

#[test]
fn no_leak_deep_shared_spine() {
    assert_no_leak(|| {
        let leaf = Value::collection(vec![
            Value::String("shared_leaf".to_string()),
            Value::Number(99.0),
        ]);
        let mut val = leaf.clone();
        for _ in 0..5_000 {
            val = Value::collection(vec![val, leaf.clone()]);
        }
    });
}
