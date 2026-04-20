use crate::value::Value;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitCategory {
    Mass,
    Volume,
    Length,
    Time,
    Amount,
    Dimensionless,
}

pub fn is_calendar_unit(u: &str) -> bool {
    matches!(
        u,
        "year"
            | "years"
            | "month"
            | "months"
            | "week"
            | "weeks"
            | "day"
            | "days"
            | "hour"
            | "hours"
            | "minute"
            | "minutes"
            | "second"
            | "seconds"
            | "millisecond"
            | "milliseconds"
    )
}

#[derive(Debug, Clone, Copy)]
struct UnitDef {
    to_base: f64,
    category: UnitCategory,
}

static UNITS: LazyLock<HashMap<&'static str, UnitDef>> = LazyLock::new(|| {
    HashMap::from([
        (
            "kg",
            UnitDef {
                to_base: 1000.0,
                category: UnitCategory::Mass,
            },
        ),
        (
            "g",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Mass,
            },
        ),
        (
            "mg",
            UnitDef {
                to_base: 0.001,
                category: UnitCategory::Mass,
            },
        ),
        (
            "ug",
            UnitDef {
                to_base: 1e-6,
                category: UnitCategory::Mass,
            },
        ),
        (
            "mcg",
            UnitDef {
                to_base: 1e-6,
                category: UnitCategory::Mass,
            },
        ),
        (
            "ng",
            UnitDef {
                to_base: 1e-9,
                category: UnitCategory::Mass,
            },
        ),
        (
            "pg",
            UnitDef {
                to_base: 1e-12,
                category: UnitCategory::Mass,
            },
        ),
        (
            "lb",
            UnitDef {
                to_base: 453.592,
                category: UnitCategory::Mass,
            },
        ),
        (
            "[lb_av]",
            UnitDef {
                to_base: 453.592,
                category: UnitCategory::Mass,
            },
        ),
        (
            "oz",
            UnitDef {
                to_base: 28.3495,
                category: UnitCategory::Mass,
            },
        ),
        (
            "[oz_av]",
            UnitDef {
                to_base: 28.3495,
                category: UnitCategory::Mass,
            },
        ),
        (
            "L",
            UnitDef {
                to_base: 1000.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "l",
            UnitDef {
                to_base: 1000.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "dL",
            UnitDef {
                to_base: 100.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "dl",
            UnitDef {
                to_base: 100.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "cL",
            UnitDef {
                to_base: 10.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "cl",
            UnitDef {
                to_base: 10.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "mL",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "ml",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "uL",
            UnitDef {
                to_base: 0.001,
                category: UnitCategory::Volume,
            },
        ),
        (
            "ul",
            UnitDef {
                to_base: 0.001,
                category: UnitCategory::Volume,
            },
        ),
        (
            "nL",
            UnitDef {
                to_base: 1e-6,
                category: UnitCategory::Volume,
            },
        ),
        (
            "nl",
            UnitDef {
                to_base: 1e-6,
                category: UnitCategory::Volume,
            },
        ),
        (
            "cc",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Volume,
            },
        ),
        (
            "m",
            UnitDef {
                to_base: 100.0,
                category: UnitCategory::Length,
            },
        ),
        (
            "dm",
            UnitDef {
                to_base: 10.0,
                category: UnitCategory::Length,
            },
        ),
        (
            "cm",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Length,
            },
        ),
        (
            "mm",
            UnitDef {
                to_base: 0.1,
                category: UnitCategory::Length,
            },
        ),
        (
            "um",
            UnitDef {
                to_base: 1e-4,
                category: UnitCategory::Length,
            },
        ),
        (
            "nm",
            UnitDef {
                to_base: 1e-7,
                category: UnitCategory::Length,
            },
        ),
        (
            "in",
            UnitDef {
                to_base: 2.54,
                category: UnitCategory::Length,
            },
        ),
        (
            "[in_i]",
            UnitDef {
                to_base: 2.54,
                category: UnitCategory::Length,
            },
        ),
        (
            "ft",
            UnitDef {
                to_base: 30.48,
                category: UnitCategory::Length,
            },
        ),
        (
            "[ft_i]",
            UnitDef {
                to_base: 30.48,
                category: UnitCategory::Length,
            },
        ),
        (
            "a",
            UnitDef {
                to_base: 31_557_600.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "mo",
            UnitDef {
                to_base: 2_629_800.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "wk",
            UnitDef {
                to_base: 604_800.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "week",
            UnitDef {
                to_base: 604_800.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "weeks",
            UnitDef {
                to_base: 604_800.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "d",
            UnitDef {
                to_base: 86_400.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "day",
            UnitDef {
                to_base: 86_400.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "days",
            UnitDef {
                to_base: 86_400.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "h",
            UnitDef {
                to_base: 3_600.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "hour",
            UnitDef {
                to_base: 3_600.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "hours",
            UnitDef {
                to_base: 3_600.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "min",
            UnitDef {
                to_base: 60.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "minute",
            UnitDef {
                to_base: 60.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "minutes",
            UnitDef {
                to_base: 60.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "s",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "second",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "seconds",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Time,
            },
        ),
        (
            "ms",
            UnitDef {
                to_base: 0.001,
                category: UnitCategory::Time,
            },
        ),
        (
            "millisecond",
            UnitDef {
                to_base: 0.001,
                category: UnitCategory::Time,
            },
        ),
        (
            "milliseconds",
            UnitDef {
                to_base: 0.001,
                category: UnitCategory::Time,
            },
        ),
        (
            "us",
            UnitDef {
                to_base: 1e-6,
                category: UnitCategory::Time,
            },
        ),
        (
            "ns",
            UnitDef {
                to_base: 1e-9,
                category: UnitCategory::Time,
            },
        ),
        (
            "mol",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Amount,
            },
        ),
        (
            "mmol",
            UnitDef {
                to_base: 0.001,
                category: UnitCategory::Amount,
            },
        ),
        (
            "umol",
            UnitDef {
                to_base: 1e-6,
                category: UnitCategory::Amount,
            },
        ),
        (
            "nmol",
            UnitDef {
                to_base: 1e-9,
                category: UnitCategory::Amount,
            },
        ),
        (
            "pmol",
            UnitDef {
                to_base: 1e-12,
                category: UnitCategory::Amount,
            },
        ),
        (
            "%",
            UnitDef {
                to_base: 0.01,
                category: UnitCategory::Dimensionless,
            },
        ),
        (
            "1",
            UnitDef {
                to_base: 1.0,
                category: UnitCategory::Dimensionless,
            },
        ),
    ])
});

#[derive(Debug, Clone)]
pub enum QuantityResult {
    Ok(Value),
    Incompatible,
}

fn normalize(value: f64, unit: &str) -> Option<(f64, UnitCategory)> {
    UNITS
        .get(unit)
        .map(|def| (value * def.to_base, def.category))
}

fn from_base(base_value: f64, unit: &str) -> Option<f64> {
    UNITS.get(unit).map(|def| base_value / def.to_base)
}

pub fn quantity_add(left: &Value, right: &Value) -> QuantityResult {
    let (v1, u1, t1) = match left {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return QuantityResult::Incompatible,
    };
    let (v2, u2, t2) = match right {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return QuantityResult::Incompatible,
    };

    if t1 != t2 {
        return QuantityResult::Incompatible;
    }

    if u1 == u2 {
        let result = v1 + v2;
        return QuantityResult::Ok(Value::Quantity(
            result,
            Value::precision(result),
            u1.to_string(),
            *t1,
        ));
    }

    let (base1, cat1) = match normalize(v1, u1) {
        Some(n) => n,
        None => return QuantityResult::Incompatible,
    };
    let (base2, cat2) = match normalize(v2, u2) {
        Some(n) => n,
        None => return QuantityResult::Incompatible,
    };

    if cat1 != cat2 {
        return QuantityResult::Incompatible;
    }

    let result_base = base1 + base2;
    match from_base(result_base, u1) {
        Some(result) => QuantityResult::Ok(Value::Quantity(
            result,
            Value::precision(result),
            u1.to_string(),
            *t1,
        )),
        None => QuantityResult::Incompatible,
    }
}

pub fn quantity_sub(left: &Value, right: &Value) -> QuantityResult {
    let (v1, u1, t1) = match left {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return QuantityResult::Incompatible,
    };
    let (v2, u2, t2) = match right {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return QuantityResult::Incompatible,
    };

    if t1 != t2 {
        return QuantityResult::Incompatible;
    }

    if u1 == u2 {
        let result = v1 - v2;
        return QuantityResult::Ok(Value::Quantity(
            result,
            Value::precision(result),
            u1.to_string(),
            *t1,
        ));
    }

    let (base1, cat1) = match normalize(v1, u1) {
        Some(n) => n,
        None => return QuantityResult::Incompatible,
    };
    let (base2, cat2) = match normalize(v2, u2) {
        Some(n) => n,
        None => return QuantityResult::Incompatible,
    };

    if cat1 != cat2 {
        return QuantityResult::Incompatible;
    }

    let result_base = base1 - base2;
    match from_base(result_base, u1) {
        Some(result) => QuantityResult::Ok(Value::Quantity(
            result,
            Value::precision(result),
            u1.to_string(),
            *t1,
        )),
        None => QuantityResult::Incompatible,
    }
}

pub fn quantity_cmp(left: &Value, right: &Value) -> Option<Ordering> {
    let (v1, u1, t1) = match left {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return None,
    };
    let (v2, u2, t2) = match right {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return None,
    };

    if t1 != t2 {
        return None;
    }

    if u1 == u2 {
        return v1.partial_cmp(&v2);
    }

    let (base1, cat1) = normalize(v1, u1)?;
    let (base2, cat2) = normalize(v2, u2)?;

    if cat1 != cat2 {
        return None;
    }

    base1.partial_cmp(&base2)
}

pub fn quantity_equivalent(left: &Value, right: &Value) -> bool {
    let (v1, u1_raw, t1) = match left {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return false,
    };
    let (v2, u2_raw, t2) = match right {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return false,
    };

    if t1 != t2 {
        return false;
    }

    if u1_raw == u2_raw {
        return (v1 - v2).abs() < f64::EPSILON;
    }

    // Lowercase for case-insensitive unit match (`'cm' ~ 'CM'` is true per spec), then alias
    // calendar year/month to UCUM `'a'`/`'mo'` so equivalence (~) treats them as equal per 3.0
    // spec, even though equality (=) considers them uncomparable (year ≈ 365.25d vs UCUM `'a'`
    // ≈ 365.2422d; same for `month` vs `'mo'`). Other calendar units (week/day/hour/...) are
    // already in UNITS with exact UCUM factors.
    let u1 = u1_raw.to_lowercase();
    let u1 = match u1.as_str() {
        "year" | "years" => "a",
        "month" | "months" => "mo",
        _ => u1.as_str(),
    };
    let u2 = u2_raw.to_lowercase();
    let u2 = match u2.as_str() {
        "year" | "years" => "a",
        "month" | "months" => "mo",
        _ => u2.as_str(),
    };

    if u1 == u2 {
        return (v1 - v2).abs() < f64::EPSILON;
    }

    let (base1, cat1) = match normalize(v1, u1) {
        Some(n) => n,
        None => return false,
    };
    let (base2, cat2) = match normalize(v2, u2) {
        Some(n) => n,
        None => return false,
    };

    cat1 == cat2 && (base1 - base2).abs() < f64::EPSILON
}

pub fn quantity_cmp_units(unit_a: &str, unit_b: &str) -> bool {
    if unit_a == unit_b {
        return true;
    }
    match (UNITS.get(unit_a), UNITS.get(unit_b)) {
        (Some(a), Some(b)) => a.category == b.category,
        _ => false,
    }
}

pub fn quantity_div(left: &Value, right: &Value) -> Option<f64> {
    let (v1, u1, t1) = match left {
        Value::Quantity(v, _, u, t) => (*v, u.as_str(), t),
        _ => return None,
    };
    let (v2, u2, t2) = match right {
        Value::Quantity(v, _, u, t) if *v != 0.0 => (*v, u.as_str(), t),
        _ => return None,
    };

    if t1 != t2 {
        return None;
    }

    if u1 == u2 {
        return Some(v1 / v2);
    }

    let (base1, cat1) = normalize(v1, u1)?;
    let (base2, cat2) = normalize(v2, u2)?;

    if cat1 != cat2 {
        return None;
    }

    Some(base1 / base2)
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_same_unit_add() {
        let left = Value::Quantity(1.0, 0, "kg".to_string(), None);
        let right = Value::Quantity(2.0, 0, "kg".to_string(), None);
        match quantity_add(&left, &right) {
            QuantityResult::Ok(Value::Quantity(value, _, ref unit, _)) => {
                assert!((value - 3.0).abs() < f64::EPSILON);
                assert_eq!(unit, "kg");
            }
            _ => panic!("Expected Ok"),
        }
    }

    #[test]
    fn test_compatible_unit_add() {
        let left = Value::Quantity(1.0, 0, "kg".to_string(), None);
        let right = Value::Quantity(500.0, 0, "g".to_string(), None);
        match quantity_add(&left, &right) {
            QuantityResult::Ok(Value::Quantity(value, _, ref unit, _)) => {
                assert!((value - 1.5).abs() < f64::EPSILON);
                assert_eq!(unit, "kg");
            }
            _ => panic!("Expected Ok"),
        }
    }

    #[test]
    fn test_incompatible_units() {
        let left = Value::Quantity(1.0, 0, "kg".to_string(), None);
        let right = Value::Quantity(1.0, 0, "mL".to_string(), None);
        match quantity_add(&left, &right) {
            QuantityResult::Incompatible => {}
            QuantityResult::Ok(_) => panic!("Expected Incompatible"),
        }
    }

    #[test]
    fn test_quantity_cmp_compatible() {
        let kg1 = Value::Quantity(1.0, 0, "kg".to_string(), None);
        let g500 = Value::Quantity(500.0, 0, "g".to_string(), None);
        let g1000 = Value::Quantity(1000.0, 0, "g".to_string(), None);
        assert_eq!(quantity_cmp(&kg1, &g500), Some(Ordering::Greater));
        assert_eq!(quantity_cmp(&g500, &kg1), Some(Ordering::Less));
        assert_eq!(quantity_cmp(&g1000, &kg1), Some(Ordering::Equal));
    }

    #[test]
    fn test_quantity_cmp_incompatible() {
        let kg = Value::Quantity(1.0, 0, "kg".to_string(), None);
        let ml = Value::Quantity(1.0, 0, "mL".to_string(), None);
        assert_eq!(quantity_cmp(&kg, &ml), None);
    }

    #[test]
    fn test_quantity_div_same_unit() {
        let left = Value::Quantity(6.0, 0, "kg".to_string(), None);
        let right = Value::Quantity(2.0, 0, "kg".to_string(), None);
        assert!((quantity_div(&left, &right).expect("should work") - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_quantity_div_compatible() {
        let left = Value::Quantity(1.0, 0, "kg".to_string(), None);
        let right = Value::Quantity(500.0, 0, "g".to_string(), None);
        assert!((quantity_div(&left, &right).expect("should work") - 2.0).abs() < f64::EPSILON);
    }
}
