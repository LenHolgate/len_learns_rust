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

    fn insert(&mut self, interval: Interval) -> bool {
        self.intervals.insert(interval);

        true
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
    fn test_insert_interval_sorting() {
        let mut intervals = Intervals::new();

        assert_eq!(intervals.insert_interval(4, 10), true);

        assert_eq!(intervals.dump(), "[4,10]");

        assert_eq!(intervals.insert_interval(5, 10), true);

        assert_eq!(intervals.dump(), "[4,10], [5,10]");

        assert_eq!(intervals.insert_interval(5, 12), true);

        assert_eq!(intervals.dump(), "[4,10], [5,10], [5,12]");

        assert_eq!(intervals.insert_interval(12, 20), true);

        assert_eq!(intervals.dump(), "[4,10], [5,10], [5,12], [12,20]");

        assert_eq!(intervals.insert_interval(10, 12), true);

        assert_eq!(intervals.dump(), "[4,10], [5,10], [5,12], [10,12], [12,20]");

        assert_eq!(intervals.insert_interval(9, 9), true);

        assert_eq!(intervals.dump(), "[9], [4,10], [5,10], [5,12], [10,12], [12,20]");

        assert_eq!(intervals.insert_interval(4, 9), true);

        assert_eq!(intervals.dump(), "[4,9], [9], [4,10], [5,10], [5,12], [10,12], [12,20]");

        assert_eq!(intervals.insert_interval(8, 11), true);

        assert_eq!(intervals.dump(), "[4,9], [8,11], [9], [4,10], [5,10], [5,12], [10,12], [12,20]");
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
}
