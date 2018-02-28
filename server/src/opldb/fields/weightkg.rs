//! Defines fields that represent weights.

use serde;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::f32;
use std::fmt;
use std::str::FromStr;
use std::num;

use opldb::WeightUnits;

/// Represents numbers describing absolute weights.
///
/// The database only tracks weights to two decimal places.
/// Instead of storing as `f32`, we can store as `u32 * 100`,
/// allowing the use of normal registers for what are effectively
/// floating-point operations, and removing all `dtoa()` calls.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct WeightKg(pub i32);

/// Represents numbers describing absolute weights in their final
/// format for printing (either Kg or Lbs).
///
/// Because the type of the weight is forgotten, these weights
/// are incomparable with each other.
#[derive(Copy, Clone, Debug)]
pub struct WeightAny(pub i32);

impl Serialize for WeightAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Write into a stack-allocated fixed-size buffer.
        serializer.serialize_str(&format!("{}", self))
    }
}

impl WeightKg {
    pub fn as_kg(&self) -> WeightAny {
        WeightAny(self.0)
    }

    pub fn as_lbs(&self) -> WeightAny {
        let f = (self.0 as f32) * 2.20462262;
        WeightAny(f.round() as i32)
    }

    pub fn as_type(&self, unit: WeightUnits) -> WeightAny {
        match unit {
            WeightUnits::Kg => self.as_kg(),
            WeightUnits::Lbs => self.as_lbs(),
        }
    }
}

impl fmt::Display for WeightKg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        WeightAny(self.0).fmt(f)
    }
}

impl fmt::Display for WeightAny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Don't display empty weights.
        if self.0 == 0 {
            Ok(())
        } else {
            // Displaying a weight only shows a single decimal place.
            // Truncate the last number.
            let integer = self.0 / 100;
            let decimal = (self.0.abs() % 100) / 10;

            // If the decimal can be avoided, don't write it.
            if decimal != 0 {
                write!(f, "{}.{}", integer, decimal)
            } else {
                write!(f, "{}", integer)
            }
        }
    }
}

impl FromStr for WeightKg {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(WeightKg(0))
        } else {
            let f = s.parse::<f32>()?;

            if f.is_finite() {
                Ok(WeightKg((f * 100.0).round() as i32))
            } else {
                Ok(WeightKg(0))
            }
        }
    }
}

struct WeightKgVisitor;

impl<'de> Visitor<'de> for WeightKgVisitor {
    type Value = WeightKg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A floating-point value or the empty string.")
    }

    fn visit_str<E>(self, value: &str) -> Result<WeightKg, E>
    where
        E: de::Error,
    {
        WeightKg::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightKg {
    fn deserialize<D>(deserializer: D) -> Result<WeightKg, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(WeightKgVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weightkg_basic() {
        let w = "".parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = "789".parse::<WeightKg>().unwrap();
        assert!(w.0 == 78900);

        let w = "123.45".parse::<WeightKg>().unwrap();
        assert!(w.0 == 12345);

        let w = "-123.45".parse::<WeightKg>().unwrap();
        assert!(w.0 == -12345);
    }

    #[test]
    fn test_weightkg_f32_edgecases() {
        // Test some special f32 values.
        let w = "-0".parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = "NaN".parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = format!("{}", f32::INFINITY).parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = format!("{}", f32::NEG_INFINITY)
            .parse::<WeightKg>()
            .unwrap();
        assert!(w.0 == 0);
    }

    #[test]
    fn test_weightkg_rounding() {
        // If extra decimal numbers are reported, round appropriately.
        let w = "123.456".parse::<WeightKg>().unwrap();
        assert!(w.0 == 12346);
        let w = "-123.456".parse::<WeightKg>().unwrap();
        assert!(w.0 == -12346);
    }

    #[test]
    fn test_weightkg_errors() {
        assert!("..".parse::<WeightKg>().is_err());
        assert!("123.45.6".parse::<WeightKg>().is_err());
        assert!("notafloat".parse::<WeightKg>().is_err());
        assert!("--123".parse::<WeightKg>().is_err());
    }

    #[test]
    fn test_weightkg_display() {
        let w = "123.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "123.4");

        let w = "100.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "100.4");

        let w = "100.056".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "100");

        let w = "-123.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "-123.4");

        let w = "-123.000".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "-123");

        let w = "-0.000".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "");
    }

    #[test]
    fn test_weightkg_ordering() {
        let w1 = "100".parse::<WeightKg>().unwrap();
        let w2 = "200".parse::<WeightKg>().unwrap();
        assert!(w1 < w2);
    }
}
