#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;
use std::collections::HashMap;

#[test]
fn number_equals_dimensionless_quantity() {
    for (expr_str, expected) in [
        ("2 = 2 '1'", Value::Boolean(true)),
        ("1 '1' = 1", Value::Boolean(true)),
        ("2 != 2 '1'", Value::Boolean(false)),
        ("2 = 3 '1'", Value::Boolean(false)),
        ("2 = 2 'mg'", Value::collection(vec![])),
        ("1 '1' ~ 1", Value::Boolean(true)),
        ("1 ~ 1.0 '1'", Value::Boolean(true)),
        ("1 !~ 1 'mg'", Value::Boolean(true)),
        ("2 in (2 '1')", Value::Boolean(true)),
        ("(2 '1') contains 2", Value::Boolean(true)),
        ("(1 | 1 '1').count() = 1", Value::Boolean(true)),
        (
            "(1 | 1 '1' | 2).distinct().count() = 2",
            Value::Boolean(true),
        ),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn number_compares_with_dimensionless_quantity() {
    for (expr_str, expected) in [
        ("2 > 1 '1'", Value::Boolean(true)),
        ("2.0 < 3 '1'", Value::Boolean(true)),
        ("1 '1' <= 1", Value::Boolean(true)),
        ("1 '1' >= 2", Value::Boolean(false)),
        ("2 > 1 'mg'", Value::collection(vec![])),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn decimal_comparison_uses_max_precision() {
    for (expr_str, expected) in [
        ("1.000000001 = 1", Value::Boolean(true)),
        ("1.00000001 = 1", Value::Boolean(false)),
        ("1.000000001 > 1", Value::Boolean(false)),
        ("1.00000001 > 1", Value::Boolean(true)),
        ("0.1 + 0.2 = 0.3", Value::Boolean(true)),
        ("1.000000001 'm' = 1 'm'", Value::Boolean(true)),
        ("1.00000001 'm' = 1 'm'", Value::Boolean(false)),
        ("1.000000001 'm' = 100 'cm'", Value::Boolean(true)),
        ("7 'cm' = 70 'mm'", Value::Boolean(true)),
        ("1.000000001.precision() = 8", Value::Boolean(true)),
        (
            "1.000000001.toString() = '1.00000000'",
            Value::Boolean(true),
        ),
        ("(1 / 3).toString() = '0.33333333'", Value::Boolean(true)),
        ("(1 / 3).precision() = 8", Value::Boolean(true)),
        ("(0.1 + 0.2).toString() = '0.3'", Value::Boolean(true)),
        ("'1.000000001'.toDecimal() = 1", Value::Boolean(true)),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn fhir_quantity_object_is_coerced_by_every_consumer() {
    let quantity = Value::object(HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Quantity".to_string()),
        ),
        ("value".to_string(), Value::number(1.0, 0)),
        ("code".to_string(), Value::String("1".to_string())),
        (
            "system".to_string(),
            Value::String("http://unitsofmeasure.org".to_string()),
        ),
    ]));
    for (expr_str, expected) in [
        ("1 = $this", Value::Boolean(true)),
        ("$this != 2", Value::Boolean(true)),
        ("1 <= $this", Value::Boolean(true)),
        ("$this < 2", Value::Boolean(true)),
        ("$this ~ 1.0", Value::Boolean(true)),
        ("1 in $this", Value::Boolean(true)),
        ("$this contains 1", Value::Boolean(true)),
        ("($this | 1).count() = 1", Value::Boolean(true)),
        (
            "($this | 1 | 2).distinct().count() = 2",
            Value::Boolean(true),
        ),
        (
            "($this | 2 | 0.5).sort().first() = 0.5",
            Value::Boolean(true),
        ),
        ("($this | 2 | 0.5).sort()[1] = 1", Value::Boolean(true)),
        ("(2 | $this).sort($this).first() = 1", Value::Boolean(true)),
        ("$this = 1 'mg'", Value::collection(vec![])),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(quantity.clone())).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn fhir_quantity_object_keeps_its_decimal_precision() {
    let quantity = Value::object(HashMap::from([
        ("value".to_string(), Value::number(1.5, 1)),
        ("code".to_string(), Value::String("mg".to_string())),
        (
            "system".to_string(),
            Value::String("http://unitsofmeasure.org".to_string()),
        ),
    ]));
    for (expr_str, expected) in [
        ("$this ~ 1.5 'mg'", Value::Boolean(true)),
        ("$this ~ 1.6 'mg'", Value::Boolean(false)),
        ("$this = 1500 'ug'", Value::Boolean(true)),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(quantity.clone())).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn values_beyond_max_precision_collapse_at_the_boundary() {
    let data = Value::collection(vec![
        Value::from_f64(3e-16),
        Value::number(1e20, 0),
        Value::number(2e20, 0),
    ]);
    for (expr_str, expected) in [
        ("$this[0] = 0", Value::Boolean(true)),
        ("$this[1] = $this[2]", Value::Boolean(false)),
        ("$this[1] < $this[2]", Value::Boolean(true)),
        ("($this[1] * $this[2]).empty()", Value::Boolean(true)),
        (
            "($this[1] + $this[2]).toString()",
            Value::String("300000000000000000000".to_string()),
        ),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(data.clone())).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn decimal_domain_covers_the_spec_range_exactly() {
    for (expr_str, expected) in [
        (
            "1000000000.00000001 = 1000000000.00000002",
            Value::Boolean(false),
        ),
        (
            "1000000000.00000001 < 1000000000.00000002",
            Value::Boolean(true),
        ),
        ("90071992.1 + 0.1 = 90071992.2", Value::Boolean(true)),
        (
            "100000000.1 '[in_i]' = 254000000.254 'cm'",
            Value::Boolean(true),
        ),
        (
            "99999999999999999999.99999999 + 0.00000001 = 100000000000000000000",
            Value::Boolean(true),
        ),
        ("(0.1 * 3).toString() = '0.3'", Value::Boolean(true)),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn overflow_and_underflow_are_empty() {
    for (expr_str, expected) in [
        ("(0.00000001 / 3).empty()", Value::Boolean(true)),
        ("(0.00000001 * 0.4).empty()", Value::Boolean(true)),
        ("0.00000001 * 0.5 = 0.00000001", Value::Boolean(true)),
        (
            "(99999999999999999999 * 99999999999999999999).empty()",
            Value::Boolean(true),
        ),
        ("(1 / 0).empty()", Value::Boolean(true)),
        ("(1 'mg' / 0).empty()", Value::Boolean(true)),
        ("(0.00000001 'mg' / 3).empty()", Value::Boolean(true)),
        (
            "(99999999999999999999 'mg' * 99999999999999999999).empty()",
            Value::Boolean(true),
        ),
        (
            "999999999999999999999999999999.combine(999999999999999999999999999999).sum().empty()",
            Value::Boolean(true),
        ),
        (
            "(999999999999999999999999999999 + 999999999999999999999999999999).empty()",
            Value::Boolean(true),
        ),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn fhir_quantity_object_with_unit_only_is_coerced() {
    let quantity = Value::object(HashMap::from([
        (
            "resourceType".to_string(),
            Value::String("Quantity".to_string()),
        ),
        ("value".to_string(), Value::number(1.0, 0)),
        ("unit".to_string(), Value::String("mg".to_string())),
    ]));
    for (expr_str, expected) in [
        ("$this = 1 'mg'", Value::Boolean(true)),
        ("$this <= 1 'mg'", Value::Boolean(true)),
        ("1 'mg' in $this", Value::Boolean(true)),
        ("($this | 1 'mg').count() = 1", Value::Boolean(true)),
        (
            "($this | 2 'mg' | 0.5 'mg').sort().first() = 0.5 'mg'",
            Value::Boolean(true),
        ),
        (
            "($this | 2 'mg' | 0.5 'mg').sort($this)[1] = 1 'mg'",
            Value::Boolean(true),
        ),
        ("$this = 1000 'ug'", Value::collection(vec![])),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(quantity.clone())).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn fhir_quantity_code_is_only_trusted_with_a_recognized_system() {
    let foreign_system = Value::object(HashMap::from([
        ("value".to_string(), Value::number(1.0, 0)),
        ("code".to_string(), Value::String("kg".to_string())),
        (
            "system".to_string(),
            Value::String("http://example.org/not-ucum".to_string()),
        ),
    ]));
    let no_system = Value::object(HashMap::from([
        ("value".to_string(), Value::number(1.0, 0)),
        ("code".to_string(), Value::String("kg".to_string())),
    ]));
    for quantity in [&foreign_system, &no_system] {
        for (expr_str, expected) in [
            ("$this = 1 'kg'", Value::collection(vec![])),
            ("$this = 1000 'g'", Value::collection(vec![])),
            ("$this < 2000 'g'", Value::collection(vec![])),
            ("1 'kg' in $this", Value::Boolean(false)),
            ("$this contains 1 'kg'", Value::Boolean(false)),
            ("($this | 1 'kg').count() = 2", Value::Boolean(true)),
            (
                "($this | 1 'kg').distinct().count() = 2",
                Value::Boolean(true),
            ),
        ] {
            let expr = parse(expr_str).expect("parse failed");
            let (result, _) = interpret(&expr, InterpreterContext::new(quantity.clone()))
                .expect("interpret failed");
            assert_eq!(result, expected, "{expr_str}");
        }
    }
}

#[test]
fn fhir_quantity_unit_matches_irrespective_of_system() {
    let quantity = Value::object(HashMap::from([
        ("value".to_string(), Value::number(1.0, 0)),
        ("unit".to_string(), Value::String("kg".to_string())),
        ("code".to_string(), Value::String("KGM".to_string())),
        (
            "system".to_string(),
            Value::String("http://example.org/not-ucum".to_string()),
        ),
    ]));
    for (expr_str, expected) in [
        ("$this = 1 'kg'", Value::Boolean(true)),
        ("$this <= 1 'kg'", Value::Boolean(true)),
        ("1 'kg' in $this", Value::Boolean(true)),
        ("($this | 1 'kg').count() = 1", Value::Boolean(true)),
        (
            "($this | 2 'kg').sort().first() = 1 'kg'",
            Value::Boolean(true),
        ),
        ("$this = 1000 'g'", Value::collection(vec![])),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(quantity.clone())).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn quantity_conversion_stays_in_range_for_large_values() {
    for (expr_str, expected) in [
        ("100000000 'kg' = 100000000000 'g'", Value::Boolean(true)),
        ("100000000 'kg' ~ 100000000000 'g'", Value::Boolean(true)),
        ("100000000 'kg' < 100000000001 'g'", Value::Boolean(true)),
        (
            "(100000000 'kg' + 0 'g').toString() = '100000000000 \\'g\\''",
            Value::Boolean(true),
        ),
        (
            "(100000000 'kg' - 1 'g') = 99999999999 'g'",
            Value::Boolean(true),
        ),
        (
            "100000000 'kg' / 100000000000 'g' = 1",
            Value::Boolean(true),
        ),
        (
            "100000000000000000000 * 1000 = 100000000000000000000000",
            Value::Boolean(true),
        ),
        (
            "100000000000000000000000 / 100000000000000000000000 = 1",
            Value::Boolean(true),
        ),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn division_near_the_decimal_range_limit() {
    for (expr_str, expected) in [
        (
            "(350000000000000000000000000000 / 400000000000000000000000000000).toString()",
            Value::String("0.875".to_string()),
        ),
        (
            "(300000000000000000000000000000 / 400000000000000000000000000000).toString()",
            Value::String("0.75".to_string()),
        ),
        (
            "(-350000000000000000000000000000 / 400000000000000000000000000000).toString()",
            Value::String("-0.875".to_string()),
        ),
        (
            "999999999999999999999999999999 / 1000000000000000000000000000000 = 1",
            Value::Boolean(true),
        ),
        (
            "(1 / 999999999999999999999999999999).empty()",
            Value::Boolean(true),
        ),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn ucum_avoirdupois_factors_are_exact() {
    for (expr_str, expected) in [
        ("1 '[lb_av]' = 453.59237 'g'", Value::Boolean(true)),
        ("1 '[oz_av]' = 28.349523125 'g'", Value::Boolean(true)),
        ("16 '[oz_av]' = 1 '[lb_av]'", Value::Boolean(true)),
        ("1 '[lb_av]' = 453.592 'g'", Value::Boolean(false)),
        ("1 '[oz_av]' = 28.3495 'g'", Value::Boolean(false)),
        ("1 '[lb_av]' > 453.592 'g'", Value::Boolean(true)),
        ("453.59237 'g' in (1 '[lb_av]')", Value::Boolean(true)),
        (
            "(1 '[lb_av]' | 453.59237 'g').count() = 1",
            Value::Boolean(true),
        ),
        (
            "2 '[lb_av]' - 1 '[lb_av]' = 453.59237 'g'",
            Value::Boolean(true),
        ),
        (
            "(1 '[lb_av]' + 0 'g').toString() = '453.59237 \\'g\\''",
            Value::Boolean(true),
        ),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}

#[test]
fn approximate_math_underflow_is_empty() {
    for (expr_str, expected) in [
        ("0.00000001.power(2).empty()", Value::Boolean(true)),
        ("10.power(-9).empty()", Value::Boolean(true)),
        ("(-20).exp().empty()", Value::Boolean(true)),
        ("1.000000001.ln()", Value::number(0.0, 0)),
        ("0.00000001.sqrt() = 0.0001", Value::Boolean(true)),
        ("0.sqrt() = 0", Value::Boolean(true)),
        ("0.exp() = 1", Value::Boolean(true)),
        ("1.ln() = 0", Value::Boolean(true)),
        ("1.log(10) = 0", Value::Boolean(true)),
        ("0.5.power(30).empty()", Value::Boolean(true)),
        ("(-1000).exp().empty()", Value::Boolean(true)),
        ("10.power(-1000).empty()", Value::Boolean(true)),
        ("0.5.power(2000).empty()", Value::Boolean(true)),
        ("(-0.5).power(2000).empty()", Value::Boolean(true)),
        ("0.power(3) = 0", Value::Boolean(true)),
        ("0.power(0) = 1", Value::Boolean(true)),
        ("0.power(-1).empty()", Value::Boolean(true)),
        ("1000.exp().empty()", Value::Boolean(true)),
    ] {
        let expr = parse(expr_str).expect("parse failed");
        let (result, _) =
            interpret(&expr, InterpreterContext::new(Value::Null)).expect("interpret failed");
        assert_eq!(result, expected, "{expr_str}");
    }
}
