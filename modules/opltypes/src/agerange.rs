//! Defines the `AgeRange` field for the `entries` table.

use std::fmt;

use crate::Age;

/// The AgeRange used by the checker for interpreting age data.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AgeRange {
    pub min: Age,
    pub max: Age,
}

impl Default for AgeRange {
    fn default() -> AgeRange {
        AgeRange {
            min: Age::None,
            max: Age::None,
        }
    }
}

impl From<Age> for AgeRange {
    /// Create an AgeRange based on a single Age.
    fn from(age: Age) -> AgeRange {
        let (min, max) = match age {
            Age::Exact(n) => (n, n),
            Age::Approximate(n) => (n, n + 1),
            Age::None => return AgeRange::default(),
        };

        AgeRange {
            min: Age::Exact(min),
            max: Age::Exact(max),
        }
    }
}

impl From<(Age, Age)> for AgeRange {
    /// Create an AgeRange based on a range of Ages.
    fn from(range: (Age, Age)) -> AgeRange {
        let min_age = match range.0 {
            Age::Exact(0) => Age::None,
            _ => range.0,
        };

        let max_age = match range.1 {
            Age::Exact(255) => Age::None,
            _ => range.1,
        };

        AgeRange {
            min: min_age,
            max: max_age,
        }
    }
}

impl AgeRange {
    /// Whether the AgeRange is the default, unassigned AgeRange.
    pub fn is_none(self) -> bool {
        self == AgeRange::default()
    }

    /// The opposite of AgeRange::is_none().
    pub fn is_some(self) -> bool {
        self != AgeRange::default()
    }

    /// Intersects this AgeRange with another.
    pub fn intersect(self, other: AgeRange) -> AgeRange {
        let mut acc = self;

        if other.min.is_some() {
            if acc.min.is_none() || other.min > acc.min {
                acc.min = other.min;
            }
        }

        if other.max.is_some() {
            if acc.max.is_none() || other.max < acc.max {
                acc.max = other.max;
            }
        }

        if acc.min.is_some() && acc.max.is_some() && acc.min.is_definitely_greater_than(acc.max) {
            AgeRange::default()
        } else {
            acc
        }
    }
}

impl fmt::Display for AgeRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.min.is_some(), self.max.is_some()) {
            (false, false) => Ok(()),
            (false, true) => write!(f, "0-{}", self.max),
            (true, false) => write!(f, "{}-999", self.min),
            (true, true) => write!(f, "{}-{}", self.min, self.max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_age() {
        let approx_40 = Age::Approximate(40);
        assert_eq!(
            AgeRange::from(approx_40),
            AgeRange::from((Age::Exact(40), Age::Exact(41)))
        );
    }
}
