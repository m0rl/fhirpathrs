use std::fmt;

const SCALE: u32 = Decimal::MAX_PRECISION as u32;
const SCALE_FACTOR: i128 = 10_i128.pow(SCALE);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal {
    mantissa: i128,
}

fn round_div(numerator: i128, denominator: i128) -> Option<i128> {
    let quotient = numerator.checked_div(denominator)?;
    let remainder = numerator.checked_rem(denominator)?.unsigned_abs();
    if remainder == 0 || remainder < denominator.unsigned_abs() - remainder {
        return Some(quotient);
    }
    if (numerator < 0) == (denominator < 0) {
        quotient.checked_add(1)
    } else {
        quotient.checked_sub(1)
    }
}

fn next_decimal_digit(remainder: u128, denominator: u128) -> (u128, u128) {
    let mut digit = 0;
    let mut scaled = 0;
    for _ in 0..10 {
        scaled += remainder;
        if scaled >= denominator {
            scaled -= denominator;
            digit += 1;
        }
    }
    (digit, scaled)
}

impl Decimal {
    pub const MAX_PRECISION: u8 = 8;
    pub const ZERO: Decimal = Decimal { mantissa: 0 };
    pub const ONE: Decimal = Decimal {
        mantissa: SCALE_FACTOR,
    };

    pub fn from_i128(value: i128) -> Option<Decimal> {
        value
            .checked_mul(SCALE_FACTOR)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn parse(text: &str) -> Option<Decimal> {
        let text = text.trim();
        let (negative, unsigned) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text.strip_prefix('+').unwrap_or(text)),
        };
        let (int_digits, frac_digits) = unsigned.split_once('.').unwrap_or((unsigned, ""));
        if int_digits.is_empty() && frac_digits.is_empty() {
            return None;
        }
        if !int_digits.bytes().all(|b| b.is_ascii_digit())
            || !frac_digits.bytes().all(|b| b.is_ascii_digit())
        {
            return None;
        }
        let mut mantissa: i128 = 0;
        for digit in int_digits.bytes() {
            mantissa = mantissa
                .checked_mul(10)?
                .checked_add(i128::from(digit - b'0'))?;
        }
        let mut kept = 0;
        for digit in frac_digits.bytes() {
            if kept == SCALE {
                if digit >= b'5' {
                    mantissa = mantissa.checked_add(1)?;
                }
                break;
            }
            mantissa = mantissa
                .checked_mul(10)?
                .checked_add(i128::from(digit - b'0'))?;
            kept += 1;
        }
        mantissa = mantissa.checked_mul(10_i128.pow(SCALE - kept))?;
        Some(Decimal {
            mantissa: if negative { -mantissa } else { mantissa },
        })
    }

    pub fn text_precision(text: &str) -> u8 {
        text.split_once('.').map_or(0, |(_, fraction)| {
            u8::try_from(fraction.len())
                .map_or(Decimal::MAX_PRECISION, |p| p.min(Decimal::MAX_PRECISION))
        })
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn from_f64(value: f64) -> Option<Decimal> {
        if !value.is_finite() {
            return None;
        }
        let int_part = value.trunc();
        if int_part.abs() >= (i128::MAX / SCALE_FACTOR) as f64 {
            return None;
        }
        let fraction = ((value - int_part) * SCALE_FACTOR as f64).round();
        (int_part as i128)
            .checked_mul(SCALE_FACTOR)?
            .checked_add(fraction as i128)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn from_f64_result(value: f64) -> Option<Decimal> {
        let decimal = Decimal::from_f64(value)?;
        if decimal.is_zero() && value != 0.0 {
            return None;
        }
        Some(decimal)
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn to_f64(self) -> f64 {
        self.mantissa as f64 / SCALE_FACTOR as f64
    }

    pub fn trunc_to_i128(self) -> i128 {
        self.mantissa / SCALE_FACTOR
    }

    pub fn is_zero(self) -> bool {
        self.mantissa == 0
    }

    pub fn is_negative(self) -> bool {
        self.mantissa < 0
    }

    pub fn is_integer(self) -> bool {
        self.mantissa % SCALE_FACTOR == 0
    }

    pub fn precision(self) -> u8 {
        let mut fraction = self.mantissa % SCALE_FACTOR;
        let mut precision = Decimal::MAX_PRECISION;
        while precision > 0 && fraction % 10 == 0 {
            fraction /= 10;
            precision -= 1;
        }
        precision
    }

    pub fn checked_neg(self) -> Option<Decimal> {
        self.mantissa
            .checked_neg()
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn checked_abs(self) -> Option<Decimal> {
        self.mantissa
            .checked_abs()
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn checked_add(self, other: Decimal) -> Option<Decimal> {
        self.mantissa
            .checked_add(other.mantissa)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn checked_sub(self, other: Decimal) -> Option<Decimal> {
        self.mantissa
            .checked_sub(other.mantissa)
            .map(|mantissa| Decimal { mantissa })
    }

    fn from_magnitude(magnitude: u128, negative: bool) -> Option<Decimal> {
        let mantissa = i128::try_from(magnitude).ok()?;
        Some(Decimal {
            mantissa: if negative { -mantissa } else { mantissa },
        })
    }

    pub fn checked_mul(self, other: Decimal) -> Option<Decimal> {
        let scale = SCALE_FACTOR.unsigned_abs();
        let (a, b) = (self.mantissa.unsigned_abs(), other.mantissa.unsigned_abs());
        let (a_int, a_frac) = (a / scale, a % scale);
        let (b_int, b_frac) = (b / scale, b % scale);
        let frac_product = a_frac * b_frac;
        let rounded_frac = frac_product / scale + u128::from(frac_product % scale >= scale / 2);
        let magnitude = a_int
            .checked_mul(b_int)?
            .checked_mul(scale)?
            .checked_add(a_int.checked_mul(b_frac)?)?
            .checked_add(a_frac.checked_mul(b_int)?)?
            .checked_add(rounded_frac)?;
        if magnitude == 0 && a != 0 && b != 0 {
            return None;
        }
        Decimal::from_magnitude(magnitude, (self.mantissa < 0) != (other.mantissa < 0))
    }

    pub fn checked_div(self, other: Decimal) -> Option<Decimal> {
        if other.mantissa == 0 {
            return None;
        }
        let scale = SCALE_FACTOR.unsigned_abs();
        let (numerator, denominator) =
            (self.mantissa.unsigned_abs(), other.mantissa.unsigned_abs());
        let mut remainder = numerator % denominator;
        let mut digits: u128 = 0;
        for _ in 0..=SCALE {
            let (digit, rest) = next_decimal_digit(remainder, denominator);
            digits = digits * 10 + digit;
            remainder = rest;
        }
        let magnitude = (numerator / denominator)
            .checked_mul(scale)?
            .checked_add(digits / 10)?
            .checked_add(u128::from(digits % 10 >= 5))?;
        if magnitude == 0 && numerator != 0 {
            return None;
        }
        Decimal::from_magnitude(magnitude, (self.mantissa < 0) != (other.mantissa < 0))
    }

    pub fn checked_rem(self, other: Decimal) -> Option<Decimal> {
        self.mantissa
            .checked_rem(other.mantissa)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn div_trunc(self, other: Decimal) -> Option<Decimal> {
        Decimal::from_i128(self.mantissa.checked_div(other.mantissa)?)
    }

    pub fn trunc(self) -> Decimal {
        Decimal {
            mantissa: self.mantissa - self.mantissa % SCALE_FACTOR,
        }
    }

    pub fn floor(self) -> Decimal {
        Decimal {
            mantissa: self.mantissa.div_euclid(SCALE_FACTOR) * SCALE_FACTOR,
        }
    }

    pub fn ceil(self) -> Option<Decimal> {
        let remainder = self.mantissa.rem_euclid(SCALE_FACTOR);
        if remainder == 0 {
            return Some(self);
        }
        self.mantissa
            .checked_add(SCALE_FACTOR - remainder)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn round_dp(self, places: i32) -> Option<Decimal> {
        let dropped_digits = i32::try_from(SCALE).ok()? - places;
        if dropped_digits <= 0 {
            return Some(self);
        }
        let Some(unit) = u32::try_from(dropped_digits)
            .ok()
            .and_then(|exp| 10_i128.checked_pow(exp))
        else {
            return Some(Decimal::ZERO);
        };
        round_div(self.mantissa, unit)?
            .checked_mul(unit)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn round_to_grain(self, grain: Decimal) -> Option<Decimal> {
        if grain.mantissa == 0 {
            return Some(self);
        }
        round_div(self.mantissa, grain.mantissa)?
            .checked_mul(grain.mantissa)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn ten_pow(exponent: i32) -> Option<Decimal> {
        let shifted = exponent.checked_add(i32::try_from(SCALE).ok()?)?;
        10_i128
            .checked_pow(u32::try_from(shifted).ok()?)
            .map(|mantissa| Decimal { mantissa })
    }

    pub fn trunc_dp(self, places: u8) -> Decimal {
        let dropped = SCALE.saturating_sub(u32::from(places));
        let unit = 10_i128.pow(dropped);
        Decimal {
            mantissa: self.mantissa - self.mantissa % unit,
        }
    }

    pub fn low_boundary(self, precision: u8) -> Option<Decimal> {
        let half_unit = 5 * 10_i128.pow(SCALE.saturating_sub(u32::from(precision)));
        let scaled = self.mantissa.checked_mul(10)?.checked_sub(half_unit)?;
        Some(Decimal {
            mantissa: scaled.div_euclid(10),
        })
    }

    pub fn high_boundary(self, precision: u8) -> Option<Decimal> {
        let half_unit = 5 * 10_i128.pow(SCALE.saturating_sub(u32::from(precision)));
        let scaled = self.mantissa.checked_mul(10)?.checked_add(half_unit)?;
        Some(Decimal {
            mantissa: scaled.div_euclid(10) + i128::from(scaled.rem_euclid(10) != 0),
        })
    }

    pub fn format(self, precision: u8) -> String {
        let magnitude = self.mantissa.unsigned_abs();
        let int_part = magnitude / SCALE_FACTOR.unsigned_abs();
        let fraction = magnitude % SCALE_FACTOR.unsigned_abs();
        let digits = format!("{fraction:0width$}", width = SCALE as usize);
        let shown = digits
            .trim_end_matches('0')
            .len()
            .max(usize::from(precision.min(Decimal::MAX_PRECISION)));
        let sign = if self.mantissa < 0 { "-" } else { "" };
        if shown == 0 {
            format!("{sign}{int_part}")
        } else {
            format!("{sign}{int_part}.{}", &digits[..shown])
        }
    }
}

impl From<i64> for Decimal {
    fn from(value: i64) -> Self {
        Decimal {
            mantissa: i128::from(value) * SCALE_FACTOR,
        }
    }
}

impl From<i32> for Decimal {
    fn from(value: i32) -> Self {
        Decimal::from(i64::from(value))
    }
}

impl From<u32> for Decimal {
    fn from(value: u32) -> Self {
        Decimal::from(i64::from(value))
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format(self.precision()))
    }
}

impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format(self.precision()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(text: &str) -> Option<Decimal> {
        Decimal::parse(text)
    }

    fn add(a: &str, b: &str) -> Option<Decimal> {
        dec(a).zip(dec(b)).and_then(|(a, b)| a.checked_add(b))
    }

    fn sub(a: &str, b: &str) -> Option<Decimal> {
        dec(a).zip(dec(b)).and_then(|(a, b)| a.checked_sub(b))
    }

    fn mul(a: &str, b: &str) -> Option<Decimal> {
        dec(a).zip(dec(b)).and_then(|(a, b)| a.checked_mul(b))
    }

    fn div(a: &str, b: &str) -> Option<Decimal> {
        dec(a).zip(dec(b)).and_then(|(a, b)| a.checked_div(b))
    }

    #[test]
    fn parses_and_formats_round_trip() {
        for text in [
            "0",
            "1",
            "-1",
            "1.5",
            "-0.5",
            "123456789012.12345678",
            "0.00000001",
        ] {
            assert_eq!(dec(text).map(|d| d.to_string()), Some(text.to_string()));
        }
        assert_eq!(dec("+2.50").map(|d| d.format(2)), Some("2.50".to_string()));
        assert_eq!(dec("2.50").map(|d| d.to_string()), Some("2.5".to_string()));
        assert_eq!(
            dec("1").map(|d| d.format(8)),
            Some("1.00000000".to_string())
        );
        assert_eq!(dec(".5").map(|d| d.to_string()), Some("0.5".to_string()));
        assert_eq!(dec("5.").map(|d| d.to_string()), Some("5".to_string()));
        assert_eq!(dec("2"), Some(Decimal::from(2)));
        assert_eq!(dec("-3"), Some(Decimal::from(-3)));
    }

    #[test]
    fn parse_rounds_half_away_from_zero_beyond_max_precision() {
        assert_eq!(dec("1.000000001"), Some(Decimal::ONE));
        assert_eq!(dec("1.000000005"), dec("1.00000001"));
        assert_eq!(dec("-1.000000005"), dec("-1.00000001"));
        assert_eq!(dec("0.000000004"), Some(Decimal::ZERO));
        assert!(dec("1000000000.00000001").is_some());
        assert_ne!(dec("1000000000.00000001"), dec("1000000000.00000002"));
    }

    #[test]
    fn parse_rejects_invalid_and_out_of_range_text() {
        for text in ["", "abc", "1e3", "1.2.3", "inf", "nan", "--1", "."] {
            assert_eq!(dec(text), None, "{text}");
        }
        assert_eq!(dec(&"9".repeat(31)), None);
        assert!(dec(&"9".repeat(30)).is_some());
    }

    #[test]
    fn text_precision_counts_fraction_digits_up_to_max() {
        assert_eq!(Decimal::text_precision("1"), 0);
        assert_eq!(Decimal::text_precision("1.50"), 2);
        assert_eq!(Decimal::text_precision("1.000000001"), 8);
    }

    #[test]
    fn from_f64_is_exact_to_max_precision() {
        assert_eq!(Decimal::from_f64(0.1), dec("0.1"));
        assert_eq!(Decimal::from_f64(0.1 + 0.2), dec("0.3"));
        assert_eq!(Decimal::from_f64(1.5865), dec("1.5865"));
        assert_eq!(Decimal::from_f64(-2.5), dec("-2.5"));
        assert_eq!(Decimal::from_f64(3e-16), Some(Decimal::ZERO));
        assert_eq!(
            Decimal::from_f64(1e20),
            dec(&format!("1{}", "0".repeat(20)))
        );
        assert_eq!(Decimal::from_f64(1e301), None);
        assert_eq!(Decimal::from_f64(f64::NAN), None);
        assert_eq!(Decimal::from_f64(f64::INFINITY), None);
    }

    #[test]
    fn arithmetic_is_exact_decimal_arithmetic() {
        assert_eq!(add("0.1", "0.2"), dec("0.3"));
        assert_eq!(add("90071992.1", "0.1"), dec("90071992.2"));
        assert_eq!(sub("1.5", "2"), dec("-0.5"));
        assert_eq!(mul("1.5", "1.25"), dec("1.875"));
        assert_eq!(mul("100000000.1", "2.54"), dec("254000000.254"));
        assert_eq!(div("1", "3"), dec("0.33333333"));
        assert_eq!(div("1.2", "1.8"), dec("0.66666667"));
        assert_eq!(div("-1", "3"), dec("-0.33333333"));
        assert_eq!(Decimal::from(7).checked_rem(Decimal::from(3)), dec("1"));
        assert_eq!(Decimal::from(-7).checked_rem(Decimal::from(3)), dec("-1"));
        assert_eq!(Decimal::from(7).div_trunc(Decimal::from(2)), dec("3"));
        assert_eq!(Decimal::from(-7).div_trunc(Decimal::from(2)), dec("-3"));
    }

    #[test]
    fn intermediates_do_not_overflow_for_representable_results() {
        assert_eq!(
            mul("100000000000000000000", "1000"),
            dec("100000000000000000000000")
        );
        assert_eq!(
            mul("1000", "100000000000000000000"),
            dec("100000000000000000000000")
        );
        assert_eq!(
            mul("-100000000000000000000.5", "1000"),
            dec("-100000000000000000000500")
        );
        assert_eq!(mul("1.00000001", "1.00000001"), dec("1.00000002"));
        assert_eq!(mul("0.33333333", "3"), dec("0.99999999"));
        assert_eq!(mul("99999999999999999999", "99999999999999999999"), None);
        assert_eq!(
            div("100000000000000000000000", "100000000000000000000000"),
            Some(Decimal::ONE)
        );
        assert_eq!(
            div("100000000000000000000000", "1000"),
            dec("100000000000000000000")
        );
        assert_eq!(div("-7", "2"), dec("-3.5"));
        assert_eq!(div("2", "3"), dec("0.66666667"));
        assert_eq!(div("1", "8"), dec("0.125"));
        assert_eq!(div("0.00000001", "0.00000001"), Some(Decimal::ONE));
    }

    #[test]
    fn division_digits_do_not_overflow_near_the_mantissa_limit() {
        let wide = |lead: &str| format!("{lead:0<30}");
        let nines = "9".repeat(30);
        let one_e30 = format!("1{}", "0".repeat(30));
        assert_eq!(div(&wide("35"), &wide("4")), dec("0.875"));
        assert_eq!(div(&format!("-{}", wide("35")), &wide("4")), dec("-0.875"));
        assert_eq!(div(&wide("35"), &format!("-{}", wide("4"))), dec("-0.875"));
        assert_eq!(div(&wide("1"), &wide("3")), dec("0.33333333"));
        assert_eq!(div(&wide("2"), &wide("3")), dec("0.66666667"));
        assert_eq!(div(&nines, &one_e30), Some(Decimal::ONE));
        assert_eq!(div(&nines, &nines), Some(Decimal::ONE));
        assert_eq!(div(&one_e30, &nines), Some(Decimal::ONE));
        assert_eq!(div("1", &nines), None);
        let limit = i128::MAX.unsigned_abs();
        assert_eq!(next_decimal_digit(limit - 1, limit), (9, limit - 10));
        assert_eq!(next_decimal_digit(0, limit), (0, 0));
    }

    #[test]
    fn division_rounds_half_away_from_zero_against_a_reference() {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut verified = 0;
        for _ in 0..500 {
            let mut operands = [Decimal::ZERO; 2];
            for operand in &mut operands {
                let magnitude =
                    ((u128::from(next()) << 64) | u128::from(next())) >> (1 + next() % 120);
                let mantissa = i128::try_from(magnitude).unwrap_or(0);
                *operand = Decimal {
                    mantissa: if next() % 2 == 0 { mantissa } else { -mantissa },
                };
            }
            let [a, b] = operands;
            if b.is_zero() {
                continue;
            }
            let Some(quotient) = a.checked_div(b) else {
                continue;
            };
            let Some(product) = quotient.checked_mul(b) else {
                continue;
            };
            let error = product.mantissa.abs_diff(a.mantissa);
            assert!(
                error <= b.mantissa.unsigned_abs() / (2 * SCALE_FACTOR.unsigned_abs()) + 1,
                "{a} / {b} = {quotient}, back-multiplied {product}"
            );
            assert_eq!(
                quotient.is_negative(),
                !quotient.is_zero() && (a.is_negative() != b.is_negative())
            );
            verified += 1;
        }
        assert!(verified >= 200, "only {verified} pairs were representable");
    }

    #[test]
    fn from_f64_result_reports_underflow() {
        assert_eq!(Decimal::from_f64_result(0.0), Some(Decimal::ZERO));
        assert_eq!(Decimal::from_f64_result(1e-9), None);
        assert_eq!(Decimal::from_f64_result(-1e-9), None);
        assert_eq!(Decimal::from_f64_result(1e-8), dec("0.00000001"));
        assert_eq!(Decimal::from_f64_result(f64::NAN), None);
        assert_eq!(Decimal::from_f64(1e-9), Some(Decimal::ZERO));
    }

    #[test]
    fn overflow_and_underflow_return_none() {
        let huge = "9".repeat(30);
        assert_eq!(add(&huge, &huge), None);
        assert_eq!(mul(&huge, "10"), None);
        assert_eq!(div("0.00000001", "3"), None);
        assert_eq!(mul("0.00000001", "0.4"), None);
        assert_eq!(mul("0.00000001", "0.5"), dec("0.00000001"));
        assert_eq!(Decimal::ONE.checked_div(Decimal::ZERO), None);
        assert_eq!(Decimal::ONE.checked_rem(Decimal::ZERO), None);
        assert_eq!(mul("0", &huge), Some(Decimal::ZERO));
        assert_eq!(Decimal::from_i128(i128::MAX), None);
    }

    #[test]
    fn rounding_functions() {
        assert_eq!(dec("2.5").and_then(|d| d.round_dp(0)), dec("3"));
        assert_eq!(dec("-2.5").and_then(|d| d.round_dp(0)), dec("-3"));
        assert_eq!(dec("1.005").and_then(|d| d.round_dp(2)), dec("1.01"));
        assert_eq!(dec("1.2345").and_then(|d| d.round_dp(2)), dec("1.23"));
        assert_eq!(dec("1234").and_then(|d| d.round_dp(-2)), dec("1200"));
        assert_eq!(
            dec("1234").and_then(|d| d.round_dp(-50)),
            Some(Decimal::ZERO)
        );
        assert_eq!(dec("1.5").and_then(|d| d.round_dp(8)), dec("1.5"));
        assert_eq!(dec("1.5").and_then(|d| d.round_dp(20)), dec("1.5"));
        assert_eq!(dec("1.7").map(Decimal::trunc), dec("1"));
        assert_eq!(dec("-1.7").map(Decimal::trunc), dec("-1"));
        assert_eq!(dec("-1.2").map(Decimal::floor), dec("-2"));
        assert_eq!(dec("1.2").and_then(Decimal::ceil), dec("2"));
        assert_eq!(dec("-1.2").and_then(Decimal::ceil), dec("-1"));
        assert_eq!(
            dec("4040")
                .zip(dec("1000"))
                .and_then(|(v, g)| v.round_to_grain(g)),
            dec("4000")
        );
        assert_eq!(
            dec("2.5")
                .zip(dec("2.54"))
                .and_then(|(v, g)| v.round_to_grain(g)),
            dec("2.54")
        );
        assert_eq!(
            dec("1.26")
                .zip(dec("0.1"))
                .and_then(|(v, g)| v.round_to_grain(g)),
            dec("1.3")
        );
    }

    #[test]
    fn boundaries_and_truncation() {
        assert_eq!(Decimal::ten_pow(-2), dec("0.01"));
        assert_eq!(Decimal::ten_pow(3), dec("1000"));
        assert_eq!(Decimal::ten_pow(-9), None);
        assert_eq!(dec("1.587").map(|d| d.trunc_dp(2)), dec("1.58"));
        assert_eq!(dec("-1.587").map(|d| d.trunc_dp(2)), dec("-1.58"));
        assert_eq!(dec("1.587").and_then(|d| d.low_boundary(3)), dec("1.5865"));
        assert_eq!(dec("1.587").and_then(|d| d.high_boundary(3)), dec("1.5875"));
        assert_eq!(
            dec("-1.587").and_then(|d| d.low_boundary(3)),
            dec("-1.5875")
        );
        assert_eq!(
            dec("-1.587").and_then(|d| d.high_boundary(3)),
            dec("-1.5865")
        );
        assert_eq!(Decimal::ONE.low_boundary(0), dec("0.5"));
        assert_eq!(Decimal::ONE.high_boundary(0), dec("1.5"));
        assert_eq!(
            dec("1.12345678").and_then(|d| d.low_boundary(8)),
            dec("1.12345677")
        );
        assert_eq!(
            dec("1.12345678").and_then(|d| d.high_boundary(8)),
            dec("1.12345679")
        );
    }

    #[test]
    fn precision_ignores_trailing_zeros() {
        assert_eq!(dec("1").map(Decimal::precision), Some(0));
        assert_eq!(dec("1.50").map(Decimal::precision), Some(1));
        assert_eq!(dec("0.33333333").map(Decimal::precision), Some(8));
        assert_eq!(dec("-0.001").map(Decimal::precision), Some(3));
        assert_eq!(Decimal::ZERO.precision(), 0);
    }

    #[test]
    fn ordering_and_predicates() {
        assert!(dec("1.5") < dec("1.50000001"));
        assert!(Decimal::from(-1) < Decimal::ZERO);
        assert!(Decimal::from(2).is_integer());
        assert_eq!(dec("2.1").map(Decimal::is_integer), Some(false));
        assert_eq!(dec("-2.1").map(Decimal::is_negative), Some(true));
        assert_eq!(dec("-2.9").map(Decimal::trunc_to_i128), Some(-2));
        assert_eq!(dec("2.9").map(Decimal::to_f64), Some(2.9));
    }
}
