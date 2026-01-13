#![allow(clippy::unwrap_used)]
use parser::parse;

fn main() {
    divan::main();
}

mod literals {
    use super::*;

    #[divan::bench]
    fn boolean() -> parser::Expression {
        parse(divan::black_box("true")).unwrap()
    }

    #[divan::bench]
    fn number_int() -> parser::Expression {
        parse(divan::black_box("42")).unwrap()
    }

    #[divan::bench]
    fn number_decimal() -> parser::Expression {
        parse(divan::black_box("3.14159")).unwrap()
    }

    #[divan::bench]
    fn string() -> parser::Expression {
        parse(divan::black_box("'hello world'")).unwrap()
    }

    #[divan::bench]
    fn date() -> parser::Expression {
        parse(divan::black_box("@2024-01-15")).unwrap()
    }

    #[divan::bench]
    fn datetime() -> parser::Expression {
        parse(divan::black_box("@2024-01-15T10:30:00.000+05:00")).unwrap()
    }

    #[divan::bench]
    fn time() -> parser::Expression {
        parse(divan::black_box("@T14:30:00.123")).unwrap()
    }

    #[divan::bench]
    fn quantity() -> parser::Expression {
        parse(divan::black_box("10 'mg'")).unwrap()
    }

    #[divan::bench]
    fn null() -> parser::Expression {
        parse(divan::black_box("{}")).unwrap()
    }
}

mod paths {
    use super::*;

    #[divan::bench]
    fn single() -> parser::Expression {
        parse(divan::black_box("name")).unwrap()
    }

    #[divan::bench]
    fn three_deep() -> parser::Expression {
        parse(divan::black_box("Patient.name.given")).unwrap()
    }

    #[divan::bench]
    fn eight_deep() -> parser::Expression {
        parse(divan::black_box("a.b.c.d.e.f.g.h")).unwrap()
    }
}

mod operators {
    use super::*;

    #[divan::bench]
    fn arithmetic() -> parser::Expression {
        parse(divan::black_box("1 + 2 * 3 - 4 / 5")).unwrap()
    }

    #[divan::bench]
    fn comparison() -> parser::Expression {
        parse(divan::black_box("a > 5 and b < 10 or c = 0")).unwrap()
    }

    #[divan::bench]
    fn precedence_chain() -> parser::Expression {
        parse(divan::black_box(
            "a implies b or c xor d and e is Integer in f != g < h | i + j * k",
        ))
        .unwrap()
    }

    #[divan::bench]
    fn parenthesized() -> parser::Expression {
        parse(divan::black_box("(1 + 2) * (3 - 4) / (5 + 6)")).unwrap()
    }
}

mod functions {
    use super::*;

    #[divan::bench]
    fn filter() -> parser::Expression {
        parse(divan::black_box("name.where(use = 'official')")).unwrap()
    }

    #[divan::bench]
    fn chained() -> parser::Expression {
        parse(divan::black_box("children().ofType(Observation).count()")).unwrap()
    }

    #[divan::bench]
    fn iif() -> parser::Expression {
        parse(divan::black_box("iif(x > 0, x, -x)")).unwrap()
    }

    #[divan::bench]
    fn multi_arg() -> parser::Expression {
        parse(divan::black_box("substring(0, 5).replace('a', 'b')")).unwrap()
    }
}

mod complex {
    use super::*;

    #[divan::bench]
    fn fhir_query() -> parser::Expression {
        parse(divan::black_box(
            "entry.resource.where(resourceType = 'Observation').value.where(code.coding.exists(system = 'http://loinc.org')).first()",
        ))
        .unwrap()
    }

    #[divan::bench]
    fn aggregate_pipeline() -> parser::Expression {
        parse(divan::black_box(
            "item.where(answer.exists()).select(answer.value).distinct().count()",
        ))
        .unwrap()
    }
}

mod deep_nesting {
    use super::*;

    #[divan::bench]
    fn where_100(bencher: divan::Bencher) {
        let mut e = "x".to_string();
        for _ in 0..100 {
            e = format!("{e}.where(true)");
        }
        bencher.bench_local(move || parse(divan::black_box(&e)).unwrap());
    }

    #[divan::bench]
    fn iif_100(bencher: divan::Bencher) {
        let mut e = "42".to_string();
        for _ in 0..100 {
            e = format!("iif(true, {e}, 0)");
        }
        bencher.bench_local(move || parse(divan::black_box(&e)).unwrap());
    }

    #[divan::bench]
    fn path_100(bencher: divan::Bencher) {
        let e = (0..100)
            .map(|i| format!("f{i}"))
            .collect::<Vec<_>>()
            .join(".");
        bencher.bench_local(move || parse(divan::black_box(&e)).unwrap());
    }
}
