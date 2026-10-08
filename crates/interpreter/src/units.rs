use crate::decimal::Decimal;
use crate::value::{Comparison, QuantityView, Value};
use std::collections::HashMap;
use std::sync::LazyLock;

pub const UCUM_SYSTEM: &str = "http://unitsofmeasure.org";

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
    to_base: i64,
    category: UnitCategory,
}

static UNITS: LazyLock<HashMap<&'static str, UnitDef>> = LazyLock::new(|| {
    use UnitCategory::{Amount, Dimensionless, Length, Mass, Time, Volume};
    [
        ("kg", 1_000_000_000_000_000, Mass),
        ("g", 1_000_000_000_000, Mass),
        ("mg", 1_000_000_000, Mass),
        ("ug", 1_000_000, Mass),
        ("mcg", 1_000_000, Mass),
        ("ng", 1_000, Mass),
        ("pg", 1, Mass),
        ("lb", 453_592_370_000_000, Mass),
        ("[lb_av]", 453_592_370_000_000, Mass),
        ("oz", 28_349_523_125_000, Mass),
        ("[oz_av]", 28_349_523_125_000, Mass),
        ("L", 1_000_000_000, Volume),
        ("l", 1_000_000_000, Volume),
        ("dL", 100_000_000, Volume),
        ("dl", 100_000_000, Volume),
        ("cL", 10_000_000, Volume),
        ("cl", 10_000_000, Volume),
        ("mL", 1_000_000, Volume),
        ("ml", 1_000_000, Volume),
        ("cc", 1_000_000, Volume),
        ("uL", 1_000, Volume),
        ("ul", 1_000, Volume),
        ("nL", 1, Volume),
        ("nl", 1, Volume),
        ("m", 1_000_000_000, Length),
        ("dm", 100_000_000, Length),
        ("cm", 10_000_000, Length),
        ("mm", 1_000_000, Length),
        ("um", 1_000, Length),
        ("nm", 1, Length),
        ("in", 25_400_000, Length),
        ("[in_i]", 25_400_000, Length),
        ("ft", 304_800_000, Length),
        ("[ft_i]", 304_800_000, Length),
        ("a", 31_557_600_000_000_000, Time),
        ("mo", 2_629_800_000_000_000, Time),
        ("wk", 604_800_000_000_000, Time),
        ("week", 604_800_000_000_000, Time),
        ("weeks", 604_800_000_000_000, Time),
        ("d", 86_400_000_000_000, Time),
        ("day", 86_400_000_000_000, Time),
        ("days", 86_400_000_000_000, Time),
        ("h", 3_600_000_000_000, Time),
        ("hour", 3_600_000_000_000, Time),
        ("hours", 3_600_000_000_000, Time),
        ("min", 60_000_000_000, Time),
        ("minute", 60_000_000_000, Time),
        ("minutes", 60_000_000_000, Time),
        ("s", 1_000_000_000, Time),
        ("second", 1_000_000_000, Time),
        ("seconds", 1_000_000_000, Time),
        ("ms", 1_000_000, Time),
        ("millisecond", 1_000_000, Time),
        ("milliseconds", 1_000_000, Time),
        ("us", 1_000, Time),
        ("ns", 1, Time),
        ("mol", 1_000_000_000_000, Amount),
        ("mmol", 1_000_000_000, Amount),
        ("umol", 1_000_000, Amount),
        ("nmol", 1_000, Amount),
        ("pmol", 1, Amount),
        ("1", 100, Dimensionless),
        ("%", 1, Dimensionless),
    ]
    .into_iter()
    .map(|(name, to_base, category)| (name, UnitDef { to_base, category }))
    .collect()
});

#[derive(Debug, Clone)]
pub enum QuantityResult {
    Ok(Value),
    Incompatible,
}

struct Aligned<'a> {
    left: Decimal,
    right: Decimal,
    unit: &'a str,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn reduced_ratio(coarse: i64, fine: i64) -> (Decimal, Decimal) {
    let divisor = gcd(coarse, fine);
    (
        Decimal::from(coarse / divisor),
        Decimal::from(fine / divisor),
    )
}

fn into_finer(value: Decimal, coarse: i64, fine: i64) -> Option<Decimal> {
    let (numerator, denominator) = reduced_ratio(coarse, fine);
    value.checked_mul(numerator)?.checked_div(denominator)
}

fn align<'a>(a: &QuantityView<'a>, b: &QuantityView<'a>) -> Option<Aligned<'a>> {
    let (code_a, code_b) = (a.ucum_code()?, b.ucum_code()?);
    let (def_a, def_b) = (UNITS.get(code_a)?, UNITS.get(code_b)?);
    if def_a.category != def_b.category {
        return None;
    }
    if def_a.to_base <= def_b.to_base {
        Some(Aligned {
            left: a.value,
            right: into_finer(b.value, def_b.to_base, def_a.to_base)?,
            unit: code_a,
        })
    } else {
        Some(Aligned {
            left: into_finer(a.value, def_a.to_base, def_b.to_base)?,
            right: b.value,
            unit: code_b,
        })
    }
}

fn combine(
    left: &Value,
    right: &Value,
    op: fn(Decimal, Decimal) -> Option<Decimal>,
) -> QuantityResult {
    let (Some(a), Some(b)) = (left.quantity_view(), right.quantity_view()) else {
        return QuantityResult::Incompatible;
    };
    if a.quantity_type != b.quantity_type {
        return QuantityResult::Incompatible;
    }
    let (l, r, unit) = if a.same_unit_as(&b) {
        let Some(unit) = a.unit else {
            return QuantityResult::Incompatible;
        };
        (a.value, b.value, unit)
    } else {
        let Some(aligned) = align(&a, &b) else {
            return QuantityResult::Incompatible;
        };
        (aligned.left, aligned.right, aligned.unit)
    };
    match op(l, r) {
        Some(value) => QuantityResult::Ok(Value::Quantity(
            value,
            value.precision(),
            unit.to_string(),
            a.quantity_type,
        )),
        None => QuantityResult::Incompatible,
    }
}

pub fn quantity_add(left: &Value, right: &Value) -> QuantityResult {
    combine(left, right, Decimal::checked_add)
}

pub fn quantity_sub(left: &Value, right: &Value) -> QuantityResult {
    combine(left, right, Decimal::checked_sub)
}

pub fn quantity_cmp(left: &Value, right: &Value) -> Comparison {
    let (Some(a), Some(b)) = (left.quantity_view(), right.quantity_view()) else {
        return Comparison::Uncomparable;
    };
    if a.quantity_type != b.quantity_type {
        return Comparison::Unequal;
    }
    if a.same_unit_as(&b) {
        return a.value.cmp(&b.value).into();
    }
    match align(&a, &b) {
        Some(aligned) => aligned.left.cmp(&aligned.right).into(),
        None => Comparison::Uncomparable,
    }
}

fn calendar_alias(unit: &str) -> String {
    match unit.to_lowercase().as_str() {
        "year" | "years" => "a".to_string(),
        "month" | "months" => "mo".to_string(),
        lower => lower.to_string(),
    }
}

fn rounded_to_min_precision(a: Decimal, b: Decimal, pa: u8, pb: u8) -> bool {
    let places = i32::from(pa.min(pb));
    matches!((a.round_dp(places), b.round_dp(places)), (Some(x), Some(y)) if x == y)
}

fn in_finer_unit_with_grain(
    value: Decimal,
    precision: u8,
    factor: i64,
    finer: i64,
) -> Option<(Decimal, Decimal)> {
    let scaled = into_finer(value, factor, finer)?;
    let (numerator, denominator) = reduced_ratio(factor, finer);
    let grain = numerator
        .checked_div(denominator)?
        .checked_mul(Decimal::ten_pow(-i32::from(precision))?)?;
    Some((scaled, grain))
}

pub fn quantity_equivalent(left: &Value, right: &Value) -> bool {
    let (Some(a), Some(b)) = (left.quantity_view(), right.quantity_view()) else {
        return false;
    };
    if a.quantity_type != b.quantity_type {
        return false;
    }
    if a.same_unit_as(&b) {
        return rounded_to_min_precision(a.value, b.value, a.precision, b.precision);
    }
    let (Some(code_a), Some(code_b)) = (a.ucum_code(), b.ucum_code()) else {
        return false;
    };
    let (alias_a, alias_b) = (calendar_alias(code_a), calendar_alias(code_b));
    if alias_a == alias_b {
        return rounded_to_min_precision(a.value, b.value, a.precision, b.precision);
    }
    let (Some(def_a), Some(def_b)) = (UNITS.get(alias_a.as_str()), UNITS.get(alias_b.as_str()))
    else {
        return false;
    };
    if def_a.category != def_b.category {
        return false;
    }
    let finer = def_a.to_base.min(def_b.to_base);
    let (Some((va, grain_a)), Some((vb, grain_b))) = (
        in_finer_unit_with_grain(a.value, a.precision, def_a.to_base, finer),
        in_finer_unit_with_grain(b.value, b.precision, def_b.to_base, finer),
    ) else {
        return false;
    };
    let grain = grain_a.max(grain_b);
    matches!(
        (va.round_to_grain(grain), vb.round_to_grain(grain)),
        (Some(x), Some(y)) if x == y
    )
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

pub fn quantity_div(left: &Value, right: &Value) -> Option<Decimal> {
    let (a, b) = (left.quantity_view()?, right.quantity_view()?);
    if a.quantity_type != b.quantity_type {
        return None;
    }
    if a.same_unit_as(&b) {
        return a.value.checked_div(b.value);
    }
    let aligned = align(&a, &b)?;
    aligned.left.checked_div(aligned.right)
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use crate::value::QuantityType;

    fn q(value: f64, precision: u8, unit: &str) -> Value {
        Value::quantity(value, precision, unit.to_string(), None)
    }

    fn fhir_quantity(fields: &[(&str, Value)]) -> Value {
        Value::object(
            fields
                .iter()
                .map(|(k, v)| ((*k).to_string(), v.clone()))
                .collect(),
        )
    }

    #[test]
    fn test_same_unit_add() {
        match quantity_add(&q(1.0, 0, "kg"), &q(2.0, 0, "kg")) {
            QuantityResult::Ok(v) => assert_eq!(v, q(3.0, 0, "kg")),
            QuantityResult::Incompatible => panic!("Expected Ok"),
        }
    }

    #[test]
    fn test_compatible_unit_add_yields_finer_unit() {
        match quantity_add(&q(1.0, 0, "kg"), &q(500.0, 0, "g")) {
            QuantityResult::Ok(v) => assert_eq!(v, q(1500.0, 0, "g")),
            QuantityResult::Incompatible => panic!("Expected Ok"),
        }
        match quantity_sub(&q(1.0, 0, "m"), &q(1.0, 0, "cm")) {
            QuantityResult::Ok(v) => assert_eq!(v, q(99.0, 0, "cm")),
            QuantityResult::Incompatible => panic!("Expected Ok"),
        }
    }

    #[test]
    fn test_incompatible_units() {
        match quantity_add(&q(1.0, 0, "kg"), &q(1.0, 0, "mL")) {
            QuantityResult::Incompatible => {}
            QuantityResult::Ok(_) => panic!("Expected Incompatible"),
        }
    }

    #[test]
    fn test_quantity_cmp_compatible() {
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "kg"), &q(500.0, 0, "g")),
            Comparison::Greater
        );
        assert_eq!(
            quantity_cmp(&q(500.0, 0, "g"), &q(1.0, 0, "kg")),
            Comparison::Less
        );
        assert_eq!(
            quantity_cmp(&q(1000.0, 0, "g"), &q(1.0, 0, "kg")),
            Comparison::Equal
        );
        assert_eq!(
            quantity_cmp(&q(7.0, 0, "cm"), &q(70.0, 0, "mm")),
            Comparison::Equal
        );
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "mg"), &q(1.0, 0, "g")),
            Comparison::Less
        );
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "[in_i]"), &q(2.54, 2, "cm")),
            Comparison::Equal
        );
    }

    #[test]
    fn test_non_decimal_factors_are_exact() {
        for (a, b) in [
            (q(1.0, 0, "[lb_av]"), q(453.59237, 5, "g")),
            (q(16.0, 0, "[oz_av]"), q(1.0, 0, "[lb_av]")),
            (q(1.0, 0, "[ft_i]"), q(30.48, 2, "cm")),
            (q(1.0, 0, "[ft_i]"), q(12.0, 0, "[in_i]")),
            (q(1.0, 0, "a"), q(365.25, 2, "d")),
            (q(12.0, 0, "mo"), q(1.0, 0, "a")),
            (q(1.0, 0, "wk"), q(7.0, 0, "d")),
        ] {
            assert_eq!(quantity_cmp(&a, &b), Comparison::Equal, "{a:?} vs {b:?}");
            assert_eq!(quantity_cmp(&b, &a), Comparison::Equal, "{b:?} vs {a:?}");
        }
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "[lb_av]"), &q(453.592, 3, "g")),
            Comparison::Greater
        );
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "[oz_av]"), &q(28.3495, 4, "g")),
            Comparison::Greater
        );
        match quantity_add(&q(1.0, 0, "[lb_av]"), &q(0.0, 0, "g")) {
            QuantityResult::Ok(v) => assert_eq!(v, q(453.59237, 5, "g")),
            QuantityResult::Incompatible => panic!("Expected Ok"),
        }
    }

    #[test]
    fn test_conversion_stays_within_range_for_large_values() {
        assert_eq!(
            quantity_cmp(&q(100000000.0, 0, "kg"), &q(100000000000.0, 0, "g")),
            Comparison::Equal
        );
        assert_eq!(
            quantity_cmp(&q(1e20, 0, "kg"), &q(1e20, 0, "g")),
            Comparison::Greater
        );
        match quantity_add(&q(100000000.0, 0, "kg"), &q(0.0, 0, "g")) {
            QuantityResult::Ok(v) => assert_eq!(v, q(100000000000.0, 0, "g")),
            QuantityResult::Incompatible => panic!("Expected Ok"),
        }
        assert_eq!(
            quantity_div(&q(100000000.0, 0, "kg"), &q(100000000000.0, 0, "g")),
            Some(Decimal::ONE)
        );
        assert!(quantity_equivalent(
            &q(100000000.0, 0, "kg"),
            &q(100000000000.0, 0, "g")
        ));
        assert_eq!(
            reduced_ratio(1_000_000_000_000_000, 1_000_000_000_000),
            (Decimal::from(1000), Decimal::ONE)
        );
        assert_eq!(
            gcd(604_800_000_000_000, 86_400_000_000_000),
            86_400_000_000_000
        );
    }

    #[test]
    fn test_quantity_cmp_incompatible() {
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "kg"), &q(1.0, 0, "mL")),
            Comparison::Uncomparable
        );
        let age = Value::quantity(1.0, 0, "kg".to_string(), Some(QuantityType::Age));
        assert_eq!(quantity_cmp(&q(1.0, 0, "kg"), &age), Comparison::Unequal);
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "year"), &q(1.0, 0, "a")),
            Comparison::Uncomparable
        );
    }

    #[test]
    fn test_quantity_cmp_number_as_dimensionless() {
        assert_eq!(
            quantity_cmp(&Value::number(2.0, 0), &q(1.0, 0, "1")),
            Comparison::Greater
        );
        assert_eq!(
            quantity_cmp(&q(1.0, 0, "1"), &Value::number(2.0, 0)),
            Comparison::Less
        );
        assert_eq!(
            quantity_cmp(&Value::number(2.0, 0), &q(1.0, 0, "mg")),
            Comparison::Uncomparable
        );
        assert_eq!(
            quantity_cmp(&Value::number(1.0, 0), &Value::number(1.0, 0)),
            Comparison::Equal
        );
    }

    #[test]
    fn test_quantity_cmp_fhir_objects() {
        let coded = fhir_quantity(&[
            ("value", Value::number(1.0, 0)),
            ("code", Value::String("1".to_string())),
            ("system", Value::String(UCUM_SYSTEM.to_string())),
        ]);
        assert_eq!(
            quantity_cmp(&coded, &Value::number(1.0, 0)),
            Comparison::Equal
        );
        assert_eq!(quantity_cmp(&coded, &q(2.0, 0, "1")), Comparison::Less);

        let unit_only = fhir_quantity(&[
            ("value", Value::number(1.0, 0)),
            ("unit", Value::String("mg".to_string())),
        ]);
        assert_eq!(
            quantity_cmp(&unit_only, &q(1.0, 0, "mg")),
            Comparison::Equal
        );
        assert_eq!(
            quantity_cmp(&unit_only, &q(1000.0, 0, "ug")),
            Comparison::Uncomparable
        );

        let foreign = fhir_quantity(&[
            ("value", Value::number(1.0, 0)),
            ("code", Value::String("kg".to_string())),
            (
                "system",
                Value::String("http://example.org/not-ucum".to_string()),
            ),
        ]);
        assert_eq!(
            quantity_cmp(&foreign, &q(1000.0, 0, "g")),
            Comparison::Uncomparable
        );
        assert_eq!(
            quantity_cmp(&foreign, &q(1.0, 0, "kg")),
            Comparison::Uncomparable
        );

        let code_without_system = fhir_quantity(&[
            ("value", Value::number(1.0, 0)),
            ("code", Value::String("kg".to_string())),
        ]);
        assert_eq!(
            quantity_cmp(&code_without_system, &q(1.0, 0, "kg")),
            Comparison::Uncomparable
        );

        let unit_with_foreign_system = fhir_quantity(&[
            ("value", Value::number(1.0, 0)),
            ("unit", Value::String("kg".to_string())),
            ("code", Value::String("KGM".to_string())),
            (
                "system",
                Value::String("http://example.org/not-ucum".to_string()),
            ),
        ]);
        assert_eq!(
            quantity_cmp(&unit_with_foreign_system, &q(1.0, 0, "kg")),
            Comparison::Equal
        );
        assert_eq!(
            quantity_cmp(&unit_with_foreign_system, &q(1000.0, 0, "g")),
            Comparison::Uncomparable
        );

        let display_unit_with_ucum_code = fhir_quantity(&[
            ("value", Value::number(1.0, 0)),
            ("unit", Value::String("kilogram".to_string())),
            ("code", Value::String("kg".to_string())),
            ("system", Value::String(UCUM_SYSTEM.to_string())),
        ]);
        assert_eq!(
            quantity_cmp(&display_unit_with_ucum_code, &q(1000.0, 0, "g")),
            Comparison::Equal
        );

        assert_eq!(
            quantity_cmp(&coded, &Value::String("1".to_string())),
            Comparison::Uncomparable
        );
    }

    #[test]
    fn test_quantity_div() {
        assert_eq!(
            quantity_div(&q(6.0, 0, "kg"), &q(2.0, 0, "kg")),
            Decimal::parse("3")
        );
        assert_eq!(
            quantity_div(&q(1.0, 0, "kg"), &q(500.0, 0, "g")),
            Decimal::parse("2")
        );
        assert_eq!(quantity_div(&q(1.0, 0, "kg"), &q(0.0, 0, "kg")), None);
        assert_eq!(quantity_div(&q(1.0, 0, "kg"), &q(1.0, 0, "mL")), None);
    }

    #[test]
    fn test_quantity_equivalent_rounds_to_coarser_grain() {
        assert!(quantity_equivalent(&q(4.0, 0, "g"), &q(4040.0, 0, "mg")));
        assert!(quantity_equivalent(&q(4.0, 0, "kg"), &q(4040.0, 0, "g")));
        assert!(!quantity_equivalent(&q(4.0, 3, "g"), &q(4040.0, 3, "mg")));
        assert!(!quantity_equivalent(&q(1.0, 0, "kg"), &q(4040.0, 0, "g")));
        assert!(quantity_equivalent(&q(1.0, 1, "m"), &q(104.0, 0, "cm")));
    }

    #[test]
    fn test_quantity_equivalent_same_unit_precision() {
        assert!(quantity_equivalent(&q(1.0, 1, "g"), &q(1.04, 2, "g")));
        assert!(!quantity_equivalent(&q(1.0, 2, "g"), &q(1.04, 2, "g")));
    }

    #[test]
    fn test_quantity_equivalent_calendar_units() {
        assert!(quantity_equivalent(&q(7.0, 0, "days"), &q(1.0, 0, "week")));
        assert!(quantity_equivalent(&q(1.0, 0, "year"), &q(1.0, 0, "a")));
        assert!(quantity_equivalent(&q(1.0, 0, "cm"), &q(1.0, 0, "CM")));
    }

    #[test]
    fn test_quantity_equivalent_number_as_dimensionless() {
        assert!(quantity_equivalent(&Value::number(1.0, 0), &q(1.0, 1, "1")));
        assert!(!quantity_equivalent(
            &Value::number(1.0, 0),
            &q(1.0, 0, "mg")
        ));
    }
}
