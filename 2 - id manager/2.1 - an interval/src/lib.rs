pub struct Interval {
    lower: u8,
    upper: u8,
}

impl Interval {
    pub fn new(lower: u8, upper: u8) -> Self {
        if upper < lower {
            panic!("upper must be >= lower");
        }

        Interval { lower, upper }
    }

    pub fn new_single_value_interval(value: u8) -> Self {
        Interval {
            lower: value,
            upper: value,
        }
    }

    pub fn lower(&self) -> u8 {
        self.lower
    }

    pub fn upper(&self) -> u8 {
        self.upper
    }

    pub fn contains_value(&self, value: u8) -> bool {
        value >= self.lower && value <= self.upper
    }

    pub fn overlaps(&self, value: &Self) -> bool {
        value.upper >= self.lower && value.lower <= self.upper
    }

    pub fn extends_lower(&self, value: &Self) -> bool {
        let next_value = value.upper + 1;

        next_value == self.lower
    }

    pub fn extends_upper(&self, value: &Self) -> bool {
        let next_value = value.lower - 1;

        next_value == self.upper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "upper must be >= lower")]
    fn test_create_upper_less_than_lower() {
        let _interval = Interval::new(12, 11);
    }

    #[test]
    fn test_lower() {
        let interval = Interval {
            lower: 10,
            upper: 11,
        };

        assert_eq!(interval.lower(), 10);
    }

    #[test]
    fn test_upper() {
        let interval = Interval {
            lower: 10,
            upper: 11,
        };

        assert_eq!(interval.upper(), 11);
    }

    #[test]
    fn test_contains_value() {
        let interval = Interval {
            lower: 10,
            upper: 12,
        };

        assert_eq!(interval.contains_value(9), false);
        assert_eq!(interval.contains_value(10), true);
        assert_eq!(interval.contains_value(11), true);
        assert_eq!(interval.contains_value(12), true);
        assert_eq!(interval.contains_value(13), false);
    }

    #[test]
    fn test_contains() {
        let interval1 = Interval {
            lower: 10,
            upper: 13,
        };

        let interval2 = Interval {
            lower: 11,
            upper: 12,
        };

        assert_eq!(interval1.overlaps(&interval1), true);
        assert_eq!(interval1.overlaps(&interval2), true);
        assert_eq!(interval2.overlaps(&interval1), true);

        let interval3 = Interval {
            lower: 11,
            upper: 14,
        };

        assert_eq!(interval1.overlaps(&interval3), true);
        assert_eq!(interval2.overlaps(&interval3), true);
        assert_eq!(interval3.overlaps(&interval1), true);
    }

    #[test]
    fn test_extends_lower() {
        let interval1 = Interval {
            lower: 10,
            upper: 13,
        };

        let interval2 = Interval { lower: 7, upper: 8 };

        assert_eq!(interval1.extends_lower(&interval2), false);

        let interval3 = Interval { lower: 7, upper: 9 };

        assert_eq!(interval1.extends_lower(&interval3), true);
    }

    #[test]
    fn test_extends_upper() {
        let interval1 = Interval {
            lower: 10,
            upper: 13,
        };

        let interval2 = Interval {
            lower: 15,
            upper: 17,
        };

        assert_eq!(interval1.extends_upper(&interval2), false);

        let interval3 = Interval {
            lower: 14,
            upper: 17,
        };

        assert_eq!(interval1.extends_upper(&interval3), true);
    }
}
