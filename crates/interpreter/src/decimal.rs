// Round `a` and `b` to the lesser of `pa`/`pb` decimal places, capped at 17 (f64's
// integer-safe exponent). Used by Number/Quantity equivalence where the 3.0 spec rounds to
// the least precision before comparing.
pub fn round_to_min_precision(a: f64, b: f64, pa: u8, pb: u8) -> (f64, f64) {
    let factor = 10_f64.powi(i32::from(pa.min(pb).min(17)));
    ((a * factor).round() / factor, (b * factor).round() / factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_to_two_decimal_places() {
        let (a, b) = round_to_min_precision(4.567, 3.219, 2, 2);
        assert!((a - 4.57).abs() < 1e-9);
        assert!((b - 3.22).abs() < 1e-9);
    }

    #[test]
    fn picks_min_of_two_precisions() {
        let (a, b) = round_to_min_precision(1.555, 1.554, 3, 1);
        assert!((a - 1.6).abs() < 1e-9);
        assert!((b - 1.6).abs() < 1e-9);
    }

    #[test]
    fn caps_at_17_digits() {
        let (a, b) = round_to_min_precision(1.23456789, 9.87654321, 200, 200);
        assert!((a - 1.23456789).abs() < 1e-9);
        assert!((b - 9.87654321).abs() < 1e-9);
    }
}
