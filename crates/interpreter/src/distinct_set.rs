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
}

impl DistinctSet {
    pub fn insert(&mut self, value: Value) -> bool {
        let fingerprint = value.fingerprint(&self.state);
        let is_equal = |index: &usize| {
            self.items
                .get(*index)
                .is_some_and(|item| item.compare_equal(&value).is_equal())
        };
        let present = match fingerprint {
            Some(fingerprint) => {
                std::iter::successors(self.heads.get(&fingerprint).copied(), |index| {
                    self.next.get(*index).copied().flatten()
                })
                .any(|index| is_equal(&index))
                    || self.unhashed.iter().any(is_equal)
            }
            None => self
                .items
                .iter()
                .any(|item| item.compare_equal(&value).is_equal()),
        };
        if present {
            return false;
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
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_keeps_first_of_equal_values_in_order() {
        let mut set = DistinctSet::default();
        let inserted: Vec<bool> = [
            Value::number(1.0, 0),
            Value::number(1.0, 1),
            Value::number(2.0, 0),
            Value::quantity(100.0, 0, "%".to_string(), None),
            Value::String("a".to_string()),
            Value::collection(vec![Value::String("a".to_string())]),
        ]
        .into_iter()
        .map(|value| set.insert(value))
        .collect();
        assert_eq!(inserted, vec![true, false, true, false, true, false]);
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
        assert!(set.insert(pound.clone()));
        assert!(!set.insert(grams.clone()));

        let mut set = DistinctSet::default();
        assert!(set.insert(grams));
        assert!(!set.insert(pound));
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
        assert!(set.insert(tablets.clone()));
        assert!(set.insert(tablets));
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
        assert!(leaves.iter().all(|leaf| set.insert(leaf.clone())));
        assert!(!set.insert(leaves[0].clone()));
    }
}
