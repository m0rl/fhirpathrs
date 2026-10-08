use std::cmp::Ordering;

pub fn try_sort_by<T: Copy, E>(
    items: &mut [T],
    mut compare: impl FnMut(&T, &T) -> Result<Ordering, E>,
) -> Result<(), E> {
    let len = items.len();
    if len < 2 {
        return Ok(());
    }
    let mut buffer = items.to_vec();
    let mut width = 1;
    while width < len {
        let mut start = 0;
        while start < len {
            let mid = (start + width).min(len);
            let end = (start + 2 * width).min(len);
            let (mut left, mut right, mut out) = (start, mid, start);
            while left < mid && right < end {
                if compare(&items[right], &items[left])? == Ordering::Less {
                    buffer[out] = items[right];
                    right += 1;
                } else {
                    buffer[out] = items[left];
                    left += 1;
                }
                out += 1;
            }
            buffer[out..out + mid - left].copy_from_slice(&items[left..mid]);
            out += mid - left;
            buffer[out..end].copy_from_slice(&items[right..end]);
            start = end;
        }
        items.copy_from_slice(&buffer);
        width *= 2;
    }
    Ok(())
}

pub fn sort_by<T: Copy>(items: &mut [T], mut compare: impl FnMut(&T, &T) -> Ordering) {
    let Ok(()) = try_sort_by(items, |a, b| {
        Ok::<_, std::convert::Infallible>(compare(a, b))
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_and_is_stable() {
        let mut items = vec![
            (3, 'a'),
            (1, 'b'),
            (2, 'c'),
            (1, 'd'),
            (3, 'e'),
            (2, 'f'),
            (1, 'g'),
        ];
        sort_by(&mut items, |a, b| a.0.cmp(&b.0));
        assert_eq!(
            items,
            vec![
                (1, 'b'),
                (1, 'd'),
                (1, 'g'),
                (2, 'c'),
                (2, 'f'),
                (3, 'a'),
                (3, 'e'),
            ]
        );
    }

    #[test]
    fn handles_small_inputs() {
        let mut empty: Vec<i32> = vec![];
        sort_by(&mut empty, i32::cmp);
        assert!(empty.is_empty());
        let mut one = vec![1];
        sort_by(&mut one, i32::cmp);
        assert_eq!(one, vec![1]);
        let mut two = vec![2, 1];
        sort_by(&mut two, i32::cmp);
        assert_eq!(two, vec![1, 2]);
        let mut three = vec![2, 3, 1];
        sort_by(&mut three, i32::cmp);
        assert_eq!(three, vec![1, 2, 3]);
    }

    #[test]
    fn matches_std_sort_on_large_input() {
        let mut seed: u32 = 12345;
        let mut items: Vec<u32> = (0..1001)
            .map(|_| {
                seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
                seed % 97
            })
            .collect();
        let mut expected = items.clone();
        expected.sort_unstable();
        sort_by(&mut items, u32::cmp);
        assert_eq!(items, expected);
    }

    #[test]
    fn propagates_comparator_error() {
        let mut items = vec![3, 1, 2];
        let result = try_sort_by(&mut items, |a, b| {
            if *a == 2 || *b == 2 {
                Err("bad")
            } else {
                Ok(a.cmp(b))
            }
        });
        assert_eq!(result, Err("bad"));
    }

    #[test]
    fn does_not_panic_on_inconsistent_comparator() {
        let mut items: Vec<u32> = (0..64).collect();
        sort_by(&mut items, |a, b| {
            if (a + b) % 3 == 0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let mut seen = items.clone();
        seen.sort_unstable();
        assert_eq!(seen, (0..64).collect::<Vec<u32>>());
    }
}
