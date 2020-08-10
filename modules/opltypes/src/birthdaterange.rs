//! Defines the `BirthDateRange` type.

use crate::{Age, Date,BirthYearRange};

use std::fmt;
use std::cmp;

/// Holds a minimum and maximum possible BirthDate.
///
/// For purposes of simplicity, the internal Date logic here is not concerned
/// with whether or not a given Date actually exists, and assumes that every
/// month has exactly 31 days. This is valid because we are only concerned with
/// whether a given MeetDate is less than or greater than a (possibly
/// nonexistent) Date.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BirthDateRange {
    pub min: Date,
    pub max: Date,
}

/// An unrealistically low Date for use as a default minimum.
const BDR_DEFAULT_MIN: Date = Date::from_parts(1100, 01, 01);
/// An unrealistically high Date for use as a default maximum.
const BDR_DEFAULT_MAX: Date = Date::from_parts(9997, 06, 15);

impl Default for BirthDateRange {
    fn default() -> Self {
        BirthDateRange {
            min: BDR_DEFAULT_MIN,
            max: BDR_DEFAULT_MAX,
        }
    }
}

impl BirthDateRange {
    pub fn new(min: Date, max: Date) -> BirthDateRange {
        assert!(min <= max);
        BirthDateRange { min, max }
    }
}

impl fmt::Display for BirthDateRange {
    /// Used for --debug-age output.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

/// Named return enum from the BirthDateRange narrow functions, for clarity.
#[derive(Debug, PartialEq)]
pub enum NarrowResult {
    /// Returned if the new range information was successfully integrated.
    Integrated,
    /// Returned if the new data conflicted with the known range.
    Conflict,
}

/// Helper function: increments a Date by a single day.
///
/// For simplicity, because it doesn't matter in this context, every month
/// is assumed to have exactly 31 days.
fn next_day(date: Date) -> Date {
    let (mut year, mut month, mut day) = (date.year(), date.month(), date.day());
    day += 1;
    if day > 31 {
        day = 1;
        month += 1;
    }
    if month > 12 {
        month = 1;
        year += 1;
    }
    Date::from_parts(year, month, day)
}

impl BirthDateRange {
    /// Shorthand constructor for use in test code.
    #[cfg(test)]
    pub fn at(
        min: Option<(u32, u32, u32)>,
        max: Option<(u32, u32, u32)>,
    ) -> BirthDateRange {
        let default = BirthDateRange::default();
        BirthDateRange::new(
            min.map(|(y, m, d)| Date::from_parts(y, m, d))
                .unwrap_or(default.min),
            max.map(|(y, m, d)| Date::from_parts(y, m, d))
                .unwrap_or(default.max),
        )
    }

    /// Returns the Age on a given Date given the known range.
    pub fn age_on(&self, date: Date) -> Age {
        // Get exact ages with respect to the bounds.
        // The minimum Age comes from the maximum BirthDate.
        let min_inferred = self.max.age_on(date).unwrap_or(Age::None);
        // The maximum Age comes from the minimum BirthDate.
        let max_inferred = self.min.age_on(date).unwrap_or(Age::None);

        // If they match, return that Age::Exact.
        if min_inferred == max_inferred {
            return min_inferred;
        }

        // If they are off-by-one, return an Age::Approximate.
        let min_num = min_inferred.to_u8_option().unwrap_or(std::u8::MIN);
        let max_num = max_inferred.to_u8_option().unwrap_or(std::u8::MAX);
        if u32::from(min_num) + 1 == u32::from(max_num) {
            return Age::Approximate(min_num);
        }

        // The range was too wide to infer a specific Age.
        Age::None
    }

    /// Interprets the BirthDateRange as a BirthYearRange.
    pub fn as_birthyearrange(&self) -> BirthYearRange {
        BirthYearRange {
            min_year: self.min.year() as _,
            max_year: self.max.year() as _,
        }
    }

    /// Intersects this BirthDateRange with another.
    pub fn intersect(&mut self, other: &BirthDateRange) -> NarrowResult {
        if self.min > other.max || other.min > self.max {
            NarrowResult::Conflict
        } else {
            self.min = cmp::max(self.min, other.min);
            self.max = cmp::min(self.max, other.max);
            NarrowResult::Integrated
        }
    }

    /// Narrows the range by a known BirthDate.
    pub fn narrow_by_birthdate(&mut self, birthdate: Date) -> NarrowResult {
        if birthdate < self.min || birthdate > self.max {
            return NarrowResult::Conflict;
        }
        self.min = birthdate;
        self.max = birthdate;
        NarrowResult::Integrated
    }

    /// Narrows the range by a known BirthYear.
    pub fn narrow_by_birthyear_range(&mut self, range: BirthYearRange) -> NarrowResult {
        let min_yeardate = Date::from_parts(range.min_year as u32, 01, 01); // Jan 1.
        let max_yeardate = Date::from_parts(range.max_year as u32, 12, 31); // Dec 31.

        let birthyear_range = BirthDateRange::new(min_yeardate, max_yeardate);
        self.intersect(&birthyear_range)
    }

    /// Narrows the range by a known Age on a specific Date.
    pub fn narrow_by_age(&mut self, age: Age, on_date: Date) -> NarrowResult {
        let (year, month, day) = (on_date.year(), on_date.month(), on_date.day());
        match age {
            Age::Exact(age) => {
                let age = u32::from(age);

                // The greatest possible BirthDate is if their birthday is that day.
                let max = Date::from_parts(year - age, month, day);

                // The least possible BirthDate is if their birthday is the next day.
                let min = next_day(Date::from_parts(year - age - 1, month, day));

                self.intersect(&BirthDateRange::new(min, max))
            }
            Age::Approximate(age) => {
                let age = u32::from(age);

                // The greatest possible BirthDate is if the lifter is younger,
                // and that day is their birthday.
                let max = Date::from_parts(year - age, month, day);

                // The least possible BirthDate is if the lifter is older,
                // and their birthday is the next day.
                let min = next_day(Date::from_parts(year - age - 2, month, day));

                self.intersect(&BirthDateRange::new(min, max))
            }
            Age::None => NarrowResult::Integrated,
        }
    }

    /// Narrows the range by a known AgeRange on a specific Date.
    pub fn narrow_by_range(&mut self, min: Age, max: Age, on_date: Date) -> NarrowResult {
        let (year, month, day) = (on_date.year(), on_date.month(), on_date.day());

        // Determine the maximum BirthDate from the lower Age (they are younger).
        let birthdate_max = match min {
            Age::Exact(age) | Age::Approximate(age) => {
                // The greatest possible BirthDate is if their birthday is that day.
                // In the case of an Approximate, the lifter is the younger option.
                Date::from_parts(year - u32::from(age), month, day)
            }
            Age::None => BDR_DEFAULT_MAX,
        };

        // Determine the minimum BirthDate from the greater Age (they are older).
        let birthdate_min = match max {
            Age::Exact(age) => {
                let age = u32::from(age);
                // The least possible BirthDate is if their birthday is the next day.
                next_day(Date::from_parts(year - age - 1, month, day))
            }
            Age::Approximate(age) => {
                let age = u32::from(age);
                // The least possible BirthDate is if their birthday is the next day,
                // assuming that they are as old as allowed.
                next_day(Date::from_parts(year - age - 2, month, day))
            }
            Age::None => BDR_DEFAULT_MIN,
        };

        let range = BirthDateRange::new(birthdate_min, birthdate_max);
        self.intersect(&range)
    }
}