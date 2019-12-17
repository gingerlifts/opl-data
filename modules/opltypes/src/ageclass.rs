//! Defines the `AgeClass` field for the `entries` table.

use crate::Age;

/// The AgeClass used by the server for partitioning into age categories.
#[derive(Copy, Clone, Debug, Deserialize, EnumString, Serialize, PartialEq)]
pub enum AgeClass {
    #[serde(rename = "5-12")]
    #[strum(serialize = "5-12")]
    Class5_12,
    #[serde(rename = "13-15")]
    #[strum(serialize = "13-15")]
    Class13_15,
    #[serde(rename = "16-17")]
    #[strum(serialize = "16-17")]
    Class16_17,
    #[serde(rename = "18-19")]
    #[strum(serialize = "18-19")]
    Class18_19,
    #[serde(rename = "20-23")]
    #[strum(serialize = "20-23")]
    Class20_23,
    #[serde(rename = "24-34")]
    #[strum(serialize = "24-34")]
    Class24_34,
    #[serde(rename = "35-39")]
    #[strum(serialize = "35-39")]
    Class35_39,
    #[serde(rename = "40-44")]
    #[strum(serialize = "40-44")]
    Class40_44,
    #[serde(rename = "45-49")]
    #[strum(serialize = "45-49")]
    Class45_49,
    #[serde(rename = "50-54")]
    #[strum(serialize = "50-54")]
    Class50_54,
    #[serde(rename = "55-59")]
    #[strum(serialize = "55-59")]
    Class55_59,
    #[serde(rename = "60-64")]
    #[strum(serialize = "60-64")]
    Class60_64,
    #[serde(rename = "65-69")]
    #[strum(serialize = "65-69")]
    Class65_69,
    #[serde(rename = "70-74")]
    #[strum(serialize = "70-74")]
    Class70_74,
    #[serde(rename = "75-79")]
    #[strum(serialize = "75-79")]
    Class75_79,
    #[serde(rename = "80-999")]
    #[strum(serialize = "80-999")]
    Class80_999,
    #[serde(rename = "")]
    #[strum(serialize = "")]
    None,
}

impl AgeClass {
    /// Assign an AgeClass based on Age.
    ///
    /// Ambiguous cases get assigned to the pessimal class (closest to Senior).
    pub fn from_age(age: Age) -> AgeClass {
        let (min, max) = match age {
            Age::Exact(n) => (n, n),
            Age::Approximate(n) => (n, n + 1),
            Age::None => {
                return AgeClass::None;
            }
        };

        // Handle the sub-senior classes, which round up.
        if max < 30 {
            match max {
                05..=12 => AgeClass::Class5_12,
                13..=15 => AgeClass::Class13_15,
                16..=17 => AgeClass::Class16_17,
                18..=19 => AgeClass::Class18_19,
                20..=23 => AgeClass::Class20_23,
                24..=34 => AgeClass::Class24_34,
                _ => AgeClass::None,
            }
        } else {
            match min {
                24..=34 => AgeClass::Class24_34,
                35..=39 => AgeClass::Class35_39,
                40..=44 => AgeClass::Class40_44,
                45..=49 => AgeClass::Class45_49,
                50..=54 => AgeClass::Class50_54,
                55..=59 => AgeClass::Class55_59,
                60..=64 => AgeClass::Class60_64,
                65..=69 => AgeClass::Class65_69,
                70..=74 => AgeClass::Class70_74,
                75..=79 => AgeClass::Class75_79,
                80..=255 => AgeClass::Class80_999,
                _ => AgeClass::None,
            }
        }
    }

    /// Assign an AgeClass based on a range of Ages.
    ///
    /// The range generally comes from a configured Division.
    ///
    /// TODO: Note that because of the limitation in AgeClass, this cannot
    /// TODO: handle Divisions like 40-49.
    pub fn from_range(min: Age, max: Age) -> AgeClass {
        let class_min = AgeClass::from_age(min);
        let class_max = AgeClass::from_age(max);
        if class_min == class_max {
            class_min
        } else {
            AgeClass::None
        }
    }
}
