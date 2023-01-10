use std::cmp::Ordering;
use std::fmt;

#[derive(PartialOrd, Eq, PartialEq, Clone)]
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

    fn dump(&self) -> String {
        format!("{}", self)
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

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lower == self.upper {
            write!(f, "[{}]", self.lower)
        } else {
            write!(f, "[{},{}]", self.lower, self.upper)
        }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        let lower_is = self.lower.cmp(&other.lower);

        let upper_is = self.upper.cmp(&other.upper);

        if lower_is == Ordering::Less {
            return Ordering::Less;
        }

        if upper_is == Ordering::Greater {
            return Ordering::Greater;
        }

        if upper_is == Ordering::Less {
            return Ordering::Less;
        }

        if lower_is == Ordering::Greater {
            return Ordering::Greater;
        }

        Ordering::Equal
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
    fn test_equal() {
        let interval1 = Interval {
            lower: 10,
            upper: 12,
        };

        let interval2 = Interval {
            lower: 10,
            upper: 12,
        };

        let interval3 = Interval {
            lower: 11,
            upper: 11,
        };

        let interval4 = Interval {
            lower: 10,
            upper: 10,
        };

        let interval5 = Interval {
            lower: 9,
            upper: 15,
        };

        let interval6 = Interval {
            lower: 11,
            upper: 11,
        };

        assert_eq!(interval1.cmp(&interval2), Ordering::Equal);
        assert_eq!(interval2.cmp(&interval1), Ordering::Equal);
        assert_eq!(interval1.cmp(&interval3), Ordering::Less);
        assert_eq!(interval1.cmp(&interval4), Ordering::Greater);
        assert_eq!(interval1.cmp(&interval5), Ordering::Less);
        assert_eq!(interval1.cmp(&interval6), Ordering::Less);
    }

    #[test]
    fn test_dump() {
        {
            let interval = Interval {
                lower: 10,
                upper: 11,
            };

            assert_eq!(interval.dump(), "[10,11]");
        }
        {
            let interval = Interval::new(22, 33);

            assert_eq!(interval.dump(), "[22,33]");
        }

        {
            let interval = Interval {
                lower: 10,
                upper: 10,
            };

            assert_eq!(interval.dump(), "[10]");
        }
        {
            let interval = Interval::new_single_value_interval(255);

            assert_eq!(interval.dump(), "[255]");
        }
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
