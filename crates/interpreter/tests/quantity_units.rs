#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::approx_constant
)]

use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;

#[test]
fn test_quantity_add_same_unit() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' + 2 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.0, 0, "kg".to_string(), None));
}

#[test]
fn test_quantity_add_compatible_kg_g() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' + 500 'g'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.5, 1, "kg".to_string(), None));
}

#[test]
fn test_quantity_add_compatible_g_kg() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("500 'g' + 1 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1500.0, 0, "g".to_string(), None));
}

#[test]
fn test_quantity_add_compatible_volume() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'L' + 500 'mL'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.5, 1, "L".to_string(), None));
}

#[test]
fn test_quantity_add_incompatible_returns_empty() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' + 1 'mL'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_quantity_sub_same_unit() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("3 'kg' - 1 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(2.0, 0, "kg".to_string(), None));
}

#[test]
fn test_quantity_sub_compatible() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("2 'kg' - 500 'g'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.5, 1, "kg".to_string(), None));
}

#[test]
fn test_quantity_sub_incompatible_returns_empty() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' - 1 'mL'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_quantity_mul_scalar_right() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("2 'kg' * 3").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(6.0, 0, "kg".to_string(), None));
}

#[test]
fn test_quantity_mul_scalar_left() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("3 * 2 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(6.0, 0, "kg".to_string(), None));
}

#[test]
fn test_quantity_div_scalar() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("6 'kg' / 2").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.0, 0, "kg".to_string(), None));
}

#[test]
fn test_quantity_div_same_unit_returns_number() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("6 'kg' / 2 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(3.0, 0));
}

#[test]
fn test_quantity_div_compatible_units_returns_number() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' / 500 'g'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Number(2.0, 0));
}

#[test]
fn test_quantity_div_incompatible_returns_empty() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' / 1 'mL'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_quantity_cmp_same_unit_greater() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("2 'kg' > 1 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_quantity_cmp_same_unit_less() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' < 2 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_quantity_cmp_compatible_greater() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' > 500 'g'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_quantity_cmp_compatible_less() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("500 'g' < 1 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_quantity_cmp_compatible_less_equal() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1000 'g' <= 1 'kg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_quantity_cmp_compatible_greater_equal() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' >= 1000 'g'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_quantity_cmp_incompatible_returns_empty() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' > 1 'mL'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_sum_quantities_same_unit() {
    let data = Value::collection(vec![
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(2.0, 0, "kg".to_string(), None),
        Value::Quantity(3.0, 0, "kg".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(6.0, 0, "kg".to_string(), None));
}

#[test]
fn test_sum_quantities_compatible_units() {
    let data = Value::collection(vec![
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(500.0, 0, "g".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.5, 1, "kg".to_string(), None));
}

#[test]
fn test_sum_quantities_incompatible_returns_empty() {
    let data = Value::collection(vec![
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(1.0, 0, "mL".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("sum()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_avg_quantities_same_unit() {
    let data = Value::collection(vec![
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(3.0, 0, "kg".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(2.0, 0, "kg".to_string(), None));
}

#[test]
fn test_avg_quantities_compatible_units() {
    let data = Value::collection(vec![
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(1000.0, 0, "g".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("avg()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.0, 0, "kg".to_string(), None));
}

#[test]
fn test_min_quantities_same_unit() {
    let data = Value::collection(vec![
        Value::Quantity(3.0, 0, "kg".to_string(), None),
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(2.0, 0, "kg".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.0, 0, "kg".to_string(), None));
}

#[test]
fn test_min_quantities_compatible_units() {
    let data = Value::collection(vec![
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(500.0, 0, "g".to_string(), None),
        Value::Quantity(2.0, 0, "kg".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("min()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(500.0, 0, "g".to_string(), None));
}

#[test]
fn test_max_quantities_same_unit() {
    let data = Value::collection(vec![
        Value::Quantity(1.0, 0, "kg".to_string(), None),
        Value::Quantity(3.0, 0, "kg".to_string(), None),
        Value::Quantity(2.0, 0, "kg".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(3.0, 0, "kg".to_string(), None));
}

#[test]
fn test_max_quantities_compatible_units() {
    let data = Value::collection(vec![
        Value::Quantity(500.0, 0, "g".to_string(), None),
        Value::Quantity(2.0, 0, "kg".to_string(), None),
        Value::Quantity(1.0, 0, "kg".to_string(), None),
    ]);
    let ctx = InterpreterContext::new(data);

    let expr = parse("max()").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(2.0, 0, "kg".to_string(), None));
}

#[test]
fn test_unknown_unit_same_unit_works() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'xyz' + 1 'xyz'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(2.0, 0, "xyz".to_string(), None));
}

#[test]
fn test_unknown_unit_mixed_with_known_returns_empty() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'kg' + 1 'xyz'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::collection(vec![]));
}

#[test]
fn test_medical_units_mg_g() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'g' + 1000 'mg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(2.0, 0, "g".to_string(), None));
}

#[test]
fn test_medical_units_mcg_mg() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'mg' + 1000 'mcg'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(2.0, 0, "mg".to_string(), None));
}

#[test]
fn test_medical_units_ml_l() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("500 'mL' < 1 'L'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_medical_units_mmol_mol() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'mol' + 500 'mmol'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.5, 1, "mol".to_string(), None));
}

#[test]
fn test_length_units_cm_m() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'm' + 50 'cm'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.5, 1, "m".to_string(), None));
}

#[test]
fn test_time_units_h_min() {
    let ctx = InterpreterContext::new(Value::Null);
    let expr = parse("1 'h' + 30 'min'").expect("parse failed");
    let (result, _) = interpret(&expr, ctx.clone()).expect("interpret failed");
    assert_eq!(result, Value::Quantity(1.5, 1, "h".to_string(), None));
}
