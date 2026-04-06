#![allow(clippy::unwrap_used, clippy::cast_precision_loss)]
use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;
use std::collections::HashMap;

fn main() {
    divan::main();
}

fn make_patient_collection(n: usize) -> Value {
    Value::collection(
        (0..n)
            .map(|i| {
                Value::object(HashMap::from([
                    ("name".to_string(), Value::String(format!("patient_{i}"))),
                    ("value".to_string(), Value::Number(i as f64, 0)),
                    ("active".to_string(), Value::Boolean(i % 2 == 0)),
                ]))
            })
            .collect(),
    )
}

mod path_navigation {
    use super::*;

    #[divan::bench]
    fn five_deep(bencher: divan::Bencher) {
        let nested = Value::object(HashMap::from([(
            "a".to_string(),
            Value::object(HashMap::from([(
                "b".to_string(),
                Value::object(HashMap::from([(
                    "c".to_string(),
                    Value::object(HashMap::from([(
                        "d".to_string(),
                        Value::object(HashMap::from([("e".to_string(), Value::Number(42.0, 0))])),
                    )])),
                )])),
            )])),
        )]));
        let expr = parse("a.b.c.d.e").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(nested.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn single_field(bencher: divan::Bencher) {
        let flat = Value::object(HashMap::from([(
            "name".to_string(),
            Value::String("test".to_string()),
        )]));
        let expr = parse("name").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(flat.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod collection_where {
    use super::*;

    #[divan::bench(consts = [10, 100, 1000])]
    fn n<const N: usize>(bencher: divan::Bencher) {
        let data = make_patient_collection(N);
        let expr = parse("where(value > 50)").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(data.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod collection_select {
    use super::*;

    #[divan::bench(consts = [10, 100, 1000])]
    fn n<const N: usize>(bencher: divan::Bencher) {
        let data = make_patient_collection(N);
        let expr = parse("select(name)").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(data.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod collection_repeat {
    use super::*;

    #[divan::bench(consts = [10, 100, 1000])]
    fn n<const N: usize>(bencher: divan::Bencher) {
        let data = make_patient_collection(N);
        let expr = parse("repeat(value)").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(data.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod collection_repeat_all {
    use super::*;

    #[divan::bench(consts = [10, 100, 1000])]
    fn n<const N: usize>(bencher: divan::Bencher) {
        let data = make_patient_collection(N);
        let expr = parse("repeatAll(value)").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(data.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod aggregation {
    use super::*;

    fn numbers_1000() -> Value {
        Value::collection((0..1000).map(|i| Value::Number(i as f64, 0)).collect())
    }

    #[divan::bench]
    fn sum_1000(bencher: divan::Bencher) {
        let numbers = numbers_1000();
        let expr = parse("sum()").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(numbers.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn count_1000(bencher: divan::Bencher) {
        let numbers = numbers_1000();
        let expr = parse("count()").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(numbers.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn aggregate_sum_1000(bencher: divan::Bencher) {
        let numbers = numbers_1000();
        let expr = parse("aggregate($total + $this, 0)").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(numbers.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod string_ops {
    use super::*;

    fn text() -> Value {
        Value::String("The quick brown fox jumps over the lazy dog".to_string())
    }

    #[divan::bench]
    fn replace(bencher: divan::Bencher) {
        let t = text();
        let expr = parse("replace('fox', 'cat')").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(t.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn matches(bencher: divan::Bencher) {
        let t = text();
        let expr = parse("matches('.*fox.*')").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(t.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn substring(bencher: divan::Bencher) {
        let t = text();
        let expr = parse("substring(4, 5)").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(t.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn length(bencher: divan::Bencher) {
        let t = text();
        let expr = parse("length()").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(t.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod type_conv {
    use super::*;

    #[divan::bench]
    fn num_to_string(bencher: divan::Bencher) {
        let num = Value::Number(42.5, 1);
        let expr = parse("toString()").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(num.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn string_to_int(bencher: divan::Bencher) {
        let s = Value::String("123".to_string());
        let expr = parse("toInteger()").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(s.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod deep_nesting {
    use super::*;

    #[divan::bench]
    fn where_100(bencher: divan::Bencher) {
        let mut e = "true".to_string();
        for _ in 0..100 {
            e = format!("{e}.where(true)");
        }
        let expr = parse(&e).unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(Value::Boolean(true)))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn iif_100(bencher: divan::Bencher) {
        let mut e = "42".to_string();
        for _ in 0..100 {
            e = format!("iif(true, {e}, 0)");
        }
        let expr = parse(&e).unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(Value::Null))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}

mod pipeline {
    use super::*;

    fn make_bundle() -> Value {
        Value::object(HashMap::from([(
            "entry".to_string(),
            Value::collection(
                (0..100)
                    .map(|i| {
                        Value::object(HashMap::from([(
                            "resource".to_string(),
                            Value::object(HashMap::from([
                                (
                                    "resourceType".to_string(),
                                    Value::String(if i % 3 == 0 {
                                        "Observation".to_string()
                                    } else {
                                        "Patient".to_string()
                                    }),
                                ),
                                ("id".to_string(), Value::String(format!("r{i}"))),
                                ("value".to_string(), Value::Number(i as f64, 0)),
                            ])),
                        )]))
                    })
                    .collect(),
            ),
        )]))
    }

    #[divan::bench]
    fn filter_and_count(bencher: divan::Bencher) {
        let data = make_bundle();
        let expr =
            parse("entry.resource.where(resourceType = 'Observation').value.count()").unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(data.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }

    #[divan::bench]
    fn filter_project_distinct(bencher: divan::Bencher) {
        let data = make_bundle();
        let expr =
            parse("entry.resource.where(resourceType = 'Observation').select(value).distinct()")
                .unwrap();
        bencher
            .with_inputs(|| InterpreterContext::new(data.clone()))
            .bench_local_values(|ctx| interpret(&expr, ctx));
    }
}
