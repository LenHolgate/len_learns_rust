use std::collections::Bound::{Excluded, Included, Unbounded};
use std::collections::BTreeSet;
use std::fmt;

use crate::interval::Interval;

pub struct Intervals {
    intervals: BTreeSet<Interval>,
}

impl Intervals {
    pub fn new() -> Self {
        Intervals {
            intervals: BTreeSet::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    pub fn insert_interval(&mut self, lower: u8, upper: u8) -> bool {
        let interval = Interval::new(lower, upper);

        self.insert(interval)
    }

    pub fn insert_value(&mut self, value: u8) -> bool {
        let interval = Interval::new_single_value_interval(value);

        self.insert(interval)
    }

    pub fn dump(&self) -> String {
        format!("{}", self)
    }

    pub fn remove_first_interval(&mut self) -> Interval {
        let first_it = self.intervals.iter().next();

        if let Some(first_interval) = first_it {
            let ret = first_interval.clone();

            self.intervals.remove(&ret);

            return ret;
        }

        panic!("Empty!");
    }

    pub fn remove_first_value(&mut self) -> u8 {
        let first_interval = self.remove_first_interval();

        let first_value = first_interval.lower();

        if first_interval.lower() != first_interval.upper()
        {
            self.intervals
                .insert(Interval::new(first_value + 1, first_interval.upper()));
        }

        first_value
    }

    pub fn remove_value(&mut self, value: u8) -> bool {
        if let Some(interval) = self.find(&Interval::new_single_value_interval(value)) {
            if interval.lower() < value {
                self.intervals
                    .insert(Interval::new(interval.lower(), value - 1));
            }

            if value < interval.upper() {
                self.intervals
                    .insert(Interval::new(value + 1, interval.upper()));
            }

            self.intervals.remove(&interval);

            return true;
        }

        false
    }

    fn find(&self, interval: &Interval) -> Option<Interval> {
        let before = self.intervals.range((Unbounded, Included(interval)));

        let prev = before.max();

        if let Some(prev) = prev {
            if prev.overlaps(interval) {
                return Some(prev.clone());
            }
        }

        let after = self.intervals.range((Included(interval), Unbounded));

        let next = after.min();

        if let Some(next) = next {
            if next.overlaps(interval) {
                return Some(next.clone());
            }
        }

        None
    }

    fn insert(&mut self, interval: Interval) -> bool {
        let intervals_after = self.intervals.range((Included(&interval), Unbounded));

        let next_it = intervals_after.min();

        let next_is = next_it.is_some();

        if next_is {
            let next_ref = next_it.unwrap();

            if next_ref.overlaps(&interval)
            {
                return false;
            }
        }

        let intervals_before = self.intervals.range((Unbounded, Excluded(&interval)));

        let prev_it = intervals_before.max();

        let prev_is = prev_it.is_some();

        if prev_is {
            let prev_ref = prev_it.unwrap();

            if prev_ref.overlaps(&interval)
            {
                return false;
            }
        }

        if next_is && prev_is {
            self.insert_or_join_intervals(interval, next_it.unwrap().clone(), prev_it.unwrap().clone());
        } else if next_is {
            self.insert_or_merge_with_next(interval, next_it.unwrap().clone());
        } else if prev_is {
            self.insert_or_merge_with_prev(interval, prev_it.unwrap().clone());
        } else {
            self.intervals.insert(interval);
        }

        true
    }

    fn insert_or_join_intervals(& mut self, interval: Interval, next: Interval, prev: Interval) {
        let next_extends = next.extends_lower(&interval);

        let prev_extends = prev.extends_upper(&interval);

        if next_extends && prev_extends
        {
            // merges the prev and next intervals

            let new_interval = Interval::new(prev.lower(), next.upper());

            self.intervals.remove(&prev);
            self.intervals.remove(&next);
            self.intervals.insert(new_interval);
        } else if next_extends
        {
            // extends the next interval

            let new_interval = Interval::new(interval.lower(), next.upper());

            self.intervals.remove(&next);
            self.intervals.insert(new_interval);
        } else if prev_extends
        {
            // extends the previous interval

            let new_interval = Interval::new(prev.lower(), interval.upper());

            self.intervals.remove(&prev);
            self.intervals.insert(new_interval);
        } else {
            self.intervals.insert(interval);
        }
    }

    fn insert_or_merge_with_next(& mut self, interval: Interval, next: Interval) {
        if next.extends_lower(&interval) {

            // extends the next interval

            let new_interval = Interval::new(interval.lower(), next.upper());

            self.intervals.remove(&next);
            self.intervals.insert(new_interval);
        } else {
            self.intervals.insert(interval);
        }
    }

    fn insert_or_merge_with_prev(& mut self, interval: Interval, prev: Interval) {
        if prev.extends_upper(&interval) {

            // extends the previous interval

            let new_interval = Interval::new(prev.lower(), interval.upper());

            self.intervals.remove(&prev);
            self.intervals.insert(new_interval);
        } else {
            self.intervals.insert(interval);
        }
    }
}

impl fmt::Display for Intervals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for interval in self.intervals.iter() {
            if !first {
                write!(f, ", ")?;
            }

            write!(f, "{}", interval)?;

            if first {
                first = false;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _intervals = Intervals::new();
    }

    #[test]
    fn test_is_empty_when_empty() {
        let intervals = Intervals::new();

        assert_eq!(intervals.is_empty(), true);
    }

    #[test]
    fn test_insert_value() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(4), true);

        assert_eq!(intervals.dump(), "[4]");

        assert_eq!(intervals.insert_value(10), true);

        assert_eq!(intervals.dump(), "[4], [10]");
    }

    #[test]
    fn test_insert_interval() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 10), true);

        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.insert_interval(12, 20), true);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");
    }

    #[test]
    fn test_is_empty_when_not_empty() {
        let mut intervals = Intervals::new();

        intervals.insert_interval(4, 10);

        assert_eq!(intervals.is_empty(), false);
    }

    #[test]
    fn test_dump() {
        let mut intervals = Intervals::new();

        intervals.insert_interval(4, 10);

        assert_eq!(intervals.dump(), "[4,10]");
    }

    #[test]
    fn test_insert_duplicate_value() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(4), true);

        assert_eq!(intervals.dump(), "[4]");

        assert_eq!(intervals.insert_value(4), false);

        assert_eq!(intervals.dump(), "[4]");
    }

    #[test]
    fn test_insert_duplicate_value_requires_correct_sorting() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(10, 20), true);

        assert_eq!(intervals.dump(), "[10,20]");

        assert_eq!(intervals.insert_value(8), true);

        assert_eq!(intervals.dump(), "[8], [10,20]");

        assert_eq!(intervals.insert_value(11), false);

        assert_eq!(intervals.dump(), "[8], [10,20]");
    }

    #[test]
    fn test_insert_value_extends_lower() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(4), true);

        assert_eq!(intervals.dump(), "[4]");

        assert_eq!(intervals.insert_value(3), true);

        assert_eq!(intervals.dump(), "[3,4]");
    }

    #[test]
    fn test_insert_value_extends_upper() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(4), true);

        assert_eq!(intervals.dump(), "[4]");

        assert_eq!(intervals.insert_value(5), true);

        assert_eq!(intervals.dump(), "[4,5]");
    }

    #[test]
    fn test_insert_value_is_before_and_is_first() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(4), true);

        assert_eq!(intervals.dump(), "[4]");

        assert_eq!(intervals.insert_value(2), true);

        assert_eq!(intervals.dump(), "[2], [4]");
    }

    #[test]
    fn test_insert_value_is_before_and_is_not_first() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(10), true);

        assert_eq!(intervals.dump(), "[10]");

        assert_eq!(intervals.insert_value(2), true);

        assert_eq!(intervals.dump(), "[2], [10]");

        assert_eq!(intervals.insert_value(5), true);

        assert_eq!(intervals.dump(), "[2], [5], [10]");

        assert_eq!(intervals.insert_value(6), true);

        assert_eq!(intervals.dump(), "[2], [5,6], [10]");
    }

    #[test]
    fn test_insert_value_is_after_and_is_last() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(10), true);

        assert_eq!(intervals.dump(), "[10]");

        assert_eq!(intervals.insert_value(18), true);

        assert_eq!(intervals.dump(), "[10], [18]");
    }

    #[test]
    fn test_insert_value_is_after_and_is_not_last() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(10), true);

        assert_eq!(intervals.dump(), "[10]");

        assert_eq!(intervals.insert_value(18), true);

        assert_eq!(intervals.dump(), "[10], [18]");

        assert_eq!(intervals.insert_value(15), true);

        assert_eq!(intervals.dump(), "[10], [15], [18]");
    }

    #[test]
    fn test_insert_value_joins_intervals() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_value(4), true);

        assert_eq!(intervals.dump(), "[4]");

        assert_eq!(intervals.insert_value(6), true);

        assert_eq!(intervals.dump(), "[4], [6]");

        assert_eq!(intervals.insert_value(5), true);

        assert_eq!(intervals.dump(), "[4,6]");
    }

    #[test]
    fn test_insert_overlapping_interval() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(2, 6), true);

        assert_eq!(intervals.dump(), "[2,6]");

        assert_eq!(intervals.insert_interval(2, 6), false);

        assert_eq!(intervals.dump(), "[2,6]");

        assert_eq!(intervals.insert_interval(3, 4), false);

        assert_eq!(intervals.dump(), "[2,6]");

        assert_eq!(intervals.insert_interval(4, 5), false);

        assert_eq!(intervals.dump(), "[2,6]");

        assert_eq!(intervals.insert_interval(5, 6), false);

        assert_eq!(intervals.dump(), "[2,6]");

        assert_eq!(intervals.insert_interval(5, 7), false);

        assert_eq!(intervals.dump(), "[2,6]");
    }

    #[test]
    fn test_insert_interval_extends_lower() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 6), true);

        assert_eq!(intervals.dump(), "[4,6]");

        assert_eq!(intervals.insert_interval(1, 3), true);

        assert_eq!(intervals.dump(), "[1,6]");
    }

    #[test]
    fn test_insert_interval_extends_upper() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 6), true);

        assert_eq!(intervals.dump(), "[4,6]");

        assert_eq!(intervals.insert_interval(7, 9), true);

        assert_eq!(intervals.dump(), "[4,9]");
    }

    #[test]
    fn test_insert_interval_is_before_and_is_first() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 6), true);

        assert_eq!(intervals.dump(), "[4,6]");

        assert_eq!(intervals.insert_interval(1, 2), true);

        assert_eq!(intervals.dump(), "[1,2], [4,6]");
    }

    #[test]
    fn test_insert_interval_is_before_and_is_not_first() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(10, 12), true);

        assert_eq!(intervals.dump(), "[10,12]");

        assert_eq!(intervals.insert_interval(2, 3), true);

        assert_eq!(intervals.dump(), "[2,3], [10,12]");

        assert_eq!(intervals.insert_interval(5, 6), true);

        assert_eq!(intervals.dump(), "[2,3], [5,6], [10,12]");
    }

    #[test]
    fn test_insert_interval_is_after_and_is_last() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(10, 12), true);

        assert_eq!(intervals.dump(), "[10,12]");

        assert_eq!(intervals.insert_interval(18, 20), true);

        assert_eq!(intervals.dump(), "[10,12], [18,20]");
    }

    #[test]
    fn test_insert_interval_is_after_and_is_not_last() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(10, 12), true);

        assert_eq!(intervals.dump(), "[10,12]");

        assert_eq!(intervals.insert_interval(18, 20), true);

        assert_eq!(intervals.dump(), "[10,12], [18,20]");

        assert_eq!(intervals.insert_interval(15, 16), true);

        assert_eq!(intervals.dump(), "[10,12], [15,16], [18,20]");
    }

    #[test]
    fn test_insert_interval_joins_intervals() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(10, 12), true);

        assert_eq!(intervals.dump(), "[10,12]");

        assert_eq!(intervals.insert_interval(18, 20), true);

        assert_eq!(intervals.dump(), "[10,12], [18,20]");

        assert_eq!(intervals.insert_interval(13, 17), true);

        assert_eq!(intervals.dump(), "[10,20]");
    }

    #[test]
    fn test_insert_interval_sorting() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 10), true);

        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.insert_interval(5, 10), false);

        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.insert_interval(5, 12), false);

        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.insert_interval(12, 20), true);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");

        assert_eq!(intervals.insert_interval(10, 12), false);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");

        assert_eq!(intervals.insert_interval(9, 9), false);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");

        assert_eq!(intervals.insert_interval(4, 9), false);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");

        assert_eq!(intervals.insert_interval(8, 11), false);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");
    }

    #[test]
    #[should_panic(expected = "Empty!")]
    fn test_remove_first_interval_when_empty() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.is_empty(), true);

        intervals.remove_first_interval();
    }

    #[test]
    fn test_remove_first_interval() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 10), true);
        assert_eq!(intervals.insert_interval(12, 20), true);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");

        let first = intervals.remove_first_interval();

        assert_eq!(first.lower(), 4);
        assert_eq!(first.upper(), 10);

        assert_eq!(intervals.dump(), "[12,20]");
    }

    #[test]
    #[should_panic(expected = "Empty!")]
    fn test_remove_first_value_when_empty() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.is_empty(), true);

        intervals.remove_first_value();
    }

    #[test]
    fn test_remove_first_value() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 10), true);
        assert_eq!(intervals.insert_interval(12, 20), true);

        assert_eq!(intervals.dump(), "[4,10], [12,20]");

        assert_eq!(intervals.remove_first_value(), 4);

        assert_eq!(intervals.dump(), "[5,10], [12,20]");
    }

    #[test]
    fn test_remove_first_value_consumes_first_interval() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 5), true);
        assert_eq!(intervals.insert_interval(12, 20), true);

        assert_eq!(intervals.dump(), "[4,5], [12,20]");

        assert_eq!(intervals.remove_first_value(), 4);

        assert_eq!(intervals.dump(), "[5], [12,20]");

        assert_eq!(intervals.remove_first_value(), 5);

        assert_eq!(intervals.dump(), "[12,20]");
    }

    #[test]
    fn test_remove_first_value_to_remove_all_values() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(u8::MIN, u8::MAX), true);

        assert_eq!(intervals.dump(), "[0,255]");

        for i in u8::MIN..u8::MAX {
            assert_eq!(intervals.remove_first_value(), i);
        }
        assert_eq!(intervals.remove_first_value(), u8::MAX);

        assert_eq!(intervals.dump(), "");
    }

    #[test]
    fn test_remove_value_from_lowest_value_of_interval() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.remove_value(4), false);

        assert_eq!(intervals.insert_interval(4, 10), true);
        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.remove_value(4), true);
        assert_eq!(intervals.dump(), "[5,10]");
    }

    #[test]
    fn test_remove_value_from_inside_interval() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.remove_value(6), false);

        assert_eq!(intervals.insert_interval(4, 10), true);
        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.remove_value(6), true);
        assert_eq!(intervals.dump(), "[4,5], [7,10]");
    }

    #[test]
    fn test_remove_value_from_highest_value_of_interval() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.remove_value(10), false);

        assert_eq!(intervals.insert_interval(4, 10), true);
        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.remove_value(10), true);
        assert_eq!(intervals.dump(), "[4,9]");
    }

    #[test]
    fn test_remove_value_to_remove_all_values() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(u8::MIN, u8::MAX), true);

        assert_eq!(intervals.dump(), "[0,255]");

        for i in u8::MIN..u8::MAX {
            assert_eq!(intervals.remove_value(i), true);
        }
        assert_eq!(intervals.remove_value(u8::MAX), true);

        assert_eq!(intervals.dump(), "");
    }
}
