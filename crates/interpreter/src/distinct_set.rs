use crate::error::InterpreterError;
use crate::value::Value;
use std::collections::HashMap;

// Values in insertion order, unique under FHIRPath `=`. A lookup compares only against items
// with the same fingerprint (chained via `heads`/`next`) and items that have none (`unhashed`).
#[derive(Default)]
pub struct DistinctSet {
    items: Vec<Value>,
    next: Vec<Option<usize>>,
    heads: HashMap<u64, usize>,
    unhashed: Vec<usize>,
    state: std::hash::RandomState,
    comparisons: usize,
}

impl DistinctSet {
    pub fn insert(
        &mut self,
        value: Value,
        comparison_limit: usize,
    ) -> Result<bool, InterpreterError> {
        let fingerprint = value.fingerprint(&self.state);
        let budget = comparison_limit.saturating_sub(self.comparisons);
        let mut comparisons = 0;
        let mut exhausted = false;
        let mut is_equal = |index: usize| {
            if comparisons == budget {
                exhausted = true;
                return true;
            }
            comparisons += 1;
            self.items
                .get(index)
                .is_some_and(|item| item.compare_equal(&value).is_equal())
        };
        let present = match fingerprint {
            Some(fingerprint) => {
                std::iter::successors(self.heads.get(&fingerprint).copied(), |index| {
                    self.next.get(*index).copied().flatten()
                })
                .any(&mut is_equal)
                    || self.unhashed.iter().copied().any(&mut is_equal)
            }
            None => (0..self.items.len()).any(&mut is_equal),
        };
        self.comparisons += comparisons;
        if exhausted {
            return Err(InterpreterError::LimitExceeded(
                "repeat comparison limit".to_string(),
            ));
        }
        if present {
            return Ok(false);
        }
        let index = self.items.len();
        match fingerprint {
            Some(fingerprint) => self.next.push(self.heads.insert(fingerprint, index)),
            None => {
                self.next.push(None);
                self.unhashed.push(index);
            }
        }
        self.items.push(value);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_keeps_first_of_equal_values_in_order() {
        let mut set = DistinctSet::default();
        let inserted: Vec<Option<bool>> = [
            Value::number(1.0, 0),
            Value::number(1.0, 1),
            Value::number(2.0, 0),
            Value::quantity(100.0, 0, "%".to_string(), None),
            Value::String("a".to_string()),
            Value::collection(vec![Value::String("a".to_string())]),
        ]
        .into_iter()
        .map(|value| set.insert(value, usize::MAX).ok())
        .collect();
        assert_eq!(
            inserted,
            [true, false, true, false, true, false].map(Some).to_vec()
        );
        assert_eq!(
            set.items,
            vec![
                Value::number(1.0, 0),
                Value::number(2.0, 0),
                Value::String("a".to_string())
            ]
        );
    }

    #[test]
    fn insert_matches_unhashed_and_hashed_values_both_ways() {
        let pound = Value::quantity(1.0, 0, "[lb_av]".to_string(), None);
        let grams = Value::quantity(453.59237, 5, "g".to_string(), None);

        let mut set = DistinctSet::default();
        assert!(matches!(set.insert(pound.clone(), usize::MAX), Ok(true)));
        assert!(matches!(set.insert(grams.clone(), usize::MAX), Ok(false)));

        let mut set = DistinctSet::default();
        assert!(matches!(set.insert(grams, usize::MAX), Ok(true)));
        assert!(matches!(set.insert(pound, usize::MAX), Ok(false)));
    }

    #[test]
    fn insert_keeps_values_that_are_not_equal_to_themselves() {
        let tablets = Value::object(HashMap::from([
            ("value".to_string(), Value::number(2.0, 0)),
            (
                "system".to_string(),
                Value::String("http://snomed.info/sct".to_string()),
            ),
            ("code".to_string(), Value::String("385055001".to_string())),
        ]));
        let mut set = DistinctSet::default();
        assert!(matches!(set.insert(tablets.clone(), usize::MAX), Ok(true)));
        assert!(matches!(set.insert(tablets, usize::MAX), Ok(true)));
        assert_eq!(set.items.len(), 2);
    }

    #[test]
    fn insert_separates_values_differing_below_fingerprint_depth() {
        let mut leaves = [Value::Boolean(true), Value::Boolean(false)];
        for _ in 0..=Value::FINGERPRINT_DEPTH {
            leaves = leaves.map(|leaf| Value::object(HashMap::from([("next".to_string(), leaf)])));
        }
        let state = std::hash::RandomState::new();
        assert_eq!(leaves[0].fingerprint(&state), leaves[1].fingerprint(&state));
        let mut set = DistinctSet::default();
        assert!(
            leaves
                .iter()
                .all(|leaf| matches!(set.insert(leaf.clone(), usize::MAX), Ok(true)))
        );
        assert!(matches!(
            set.insert(leaves[0].clone(), usize::MAX),
            Ok(false)
        ));
    }

    #[test]
    fn comparisons_count_every_item_scanned_for_unhashed_values() {
        let mut set = DistinctSet::default();
        for pounds in [1.0, 2.0, 3.0] {
            let pound = Value::quantity(pounds, 0, "[lb_av]".to_string(), None);
            assert!(matches!(set.insert(pound, usize::MAX), Ok(true)));
        }
        assert_eq!(set.comparisons, 3);
    }

    #[test]
    fn comparisons_skip_items_with_other_fingerprints() {
        let mut set = DistinctSet::default();
        for number in 0..100 {
            let number = Value::number(f64::from(number), 0);
            assert!(matches!(set.insert(number, usize::MAX), Ok(true)));
        }
        assert_eq!(set.comparisons, 0);
    }

    #[test]
    fn insert_succeeds_with_exactly_the_comparison_limit() {
        let mut set = DistinctSet::default();
        for pounds in [1.0, 2.0, 3.0] {
            let pound = Value::quantity(pounds, 0, "[lb_av]".to_string(), None);
            assert!(matches!(set.insert(pound, 3), Ok(true)));
        }
        let pound = Value::quantity(4.0, 0, "[lb_av]".to_string(), None);
        assert!(matches!(
            set.insert(pound, 3),
            Err(InterpreterError::LimitExceeded(_))
        ));
        assert_eq!(set.comparisons, 3);
        assert_eq!(set.items.len(), 3);
    }

    #[test]
    fn insert_stops_unhashed_value_scan_at_the_comparison_limit() {
        let mut set = DistinctSet::default();
        for number in 0..10 {
            let number = Value::number(f64::from(number), 0);
            assert!(matches!(set.insert(number, usize::MAX), Ok(true)));
        }
        let pound = Value::quantity(1.0, 0, "[lb_av]".to_string(), None);
        assert!(matches!(
            set.insert(pound, 3),
            Err(InterpreterError::LimitExceeded(_))
        ));
        assert_eq!(set.comparisons, 3);
        assert_eq!(set.items.len(), 10);
    }

    #[test]
    fn insert_stops_unhashed_bucket_scan_at_the_comparison_limit() {
        let mut set = DistinctSet::default();
        for pounds in [1.0, 2.0, 3.0] {
            let pound = Value::quantity(pounds, 0, "[lb_av]".to_string(), None);
            assert!(matches!(set.insert(pound, usize::MAX), Ok(true)));
        }
        assert!(matches!(
            set.insert(Value::number(7.0, 0), 4),
            Err(InterpreterError::LimitExceeded(_))
        ));
        assert_eq!(set.comparisons, 4);
        assert_eq!(set.items.len(), 3);
    }

    #[test]
    fn insert_stops_fingerprint_chain_scan_at_the_comparison_limit() {
        let mut leaves = [Value::Boolean(true), Value::Boolean(false)];
        for _ in 0..=Value::FINGERPRINT_DEPTH {
            leaves = leaves.map(|leaf| Value::object(HashMap::from([("next".to_string(), leaf)])));
        }
        let mut set = DistinctSet::default();
        assert!(matches!(set.insert(leaves[0].clone(), 0), Ok(true)));
        assert!(matches!(
            set.insert(leaves[1].clone(), 0),
            Err(InterpreterError::LimitExceeded(_))
        ));
        assert_eq!(set.comparisons, 0);
        assert_eq!(set.items.len(), 1);
    }
}
