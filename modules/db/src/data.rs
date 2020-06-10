//! Defines the in-memory data format.

use opltypes::states::*;
use opltypes::*;

use crate::yesno::deserialize_yes_no;

/// Row for a  Data for a unique lifter.
///
/// Lifters are uniquely identified throughout the database by any of:
///  1. LifterID
///  2. Username
///  3. Name
///
/// Lifters are stored in a `Vec<Lifter>`.
/// The LifterID is the index of the struct in the vector.
#[derive(Serialize, Deserialize)]
pub struct Lifter {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "CyrillicName"))]
    pub cyrillic_name: Option<String>,
    #[serde(rename(deserialize = "GreekName"))]
    pub greek_name: Option<String>,
    #[serde(rename(deserialize = "JapaneseName"))]
    pub japanese_name: Option<String>,
    #[serde(rename(deserialize = "KoreanName"))]
    pub korean_name: Option<String>,
    #[serde(rename(deserialize = "Username"))]
    pub username: String,
    #[serde(rename(deserialize = "Instagram"))]
    pub instagram: Option<String>,
    #[serde(rename(deserialize = "VKontakte"))]
    pub vkontakte: Option<String>,
    #[serde(rename(deserialize = "Color"))]
    pub color: Option<String>,
    #[serde(rename(deserialize = "Flair"))]
    pub flair: Option<String>,
}

/// The definition of a Meet in the database.
///
/// Meets are stored in a `Vec<Meet>`.
/// The MeetID is the index of the struct in the vector.
#[derive(Serialize, Deserialize)]
pub struct Meet {
    #[serde(rename(deserialize = "MeetPath"))]
    pub path: String,
    #[serde(rename(deserialize = "Federation"))]
    pub federation: Federation,
    #[serde(rename(deserialize = "Date"))]
    pub date: Date,
    #[serde(rename(deserialize = "MeetCountry"))]
    pub country: Country,
    #[serde(rename(deserialize = "MeetState"))]
    pub state: Option<String>,
    #[serde(rename(deserialize = "MeetTown"))]
    pub town: Option<String>,
    #[serde(rename(deserialize = "MeetName"))]
    pub name: String,
    #[serde(rename(deserialize = "RuleSet"))]
    pub ruleset: RuleSet,

    /// Number of unique competitors, by LifterID.
    /// Calculated at load-time.
    #[serde(default)]
    pub num_unique_lifters: u32,
}

/// The definition of an Entry in the database.
///
/// Entries are stored in a `Vec<Entry>` such that all entries for a given `lifter_id`
/// are contiguous. This allows for very quickly determining a lifter's best Entry.
#[derive(Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename(deserialize = "MeetID"))]
    pub meet_id: u32,
    #[serde(rename(deserialize = "LifterID"))]
    pub lifter_id: u32,
    #[serde(rename(deserialize = "Sex"))]
    pub sex: Sex,
    #[serde(rename(deserialize = "Event"))]
    pub event: Event,
    #[serde(rename(deserialize = "Equipment"))]
    pub equipment: Equipment,
    #[serde(rename(deserialize = "Age"))]
    pub age: Age,
    #[serde(rename(deserialize = "Division"))]
    pub division: Option<String>,
    #[serde(rename(deserialize = "BodyweightKg"))]
    pub bodyweightkg: WeightKg,
    #[serde(rename(deserialize = "WeightClassKg"))]
    pub weightclasskg: WeightClassKg,
    #[serde(rename(deserialize = "Squat1Kg"))]
    pub squat1kg: WeightKg,
    #[serde(rename(deserialize = "Squat2Kg"))]
    pub squat2kg: WeightKg,
    #[serde(rename(deserialize = "Squat3Kg"))]
    pub squat3kg: WeightKg,
    #[serde(rename(deserialize = "Squat4Kg"))]
    pub squat4kg: WeightKg,
    #[serde(rename(deserialize = "Best3SquatKg"))]
    pub best3squatkg: WeightKg,
    #[serde(rename(deserialize = "Bench1Kg"))]
    pub bench1kg: WeightKg,
    #[serde(rename(deserialize = "Bench2Kg"))]
    pub bench2kg: WeightKg,
    #[serde(rename(deserialize = "Bench3Kg"))]
    pub bench3kg: WeightKg,
    #[serde(rename(deserialize = "Bench4Kg"))]
    pub bench4kg: WeightKg,
    #[serde(rename(deserialize = "Best3BenchKg"))]
    pub best3benchkg: WeightKg,
    #[serde(rename(deserialize = "Deadlift1Kg"))]
    pub deadlift1kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift2Kg"))]
    pub deadlift2kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift3Kg"))]
    pub deadlift3kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift4Kg"))]
    pub deadlift4kg: WeightKg,
    #[serde(rename(deserialize = "Best3DeadliftKg"))]
    pub best3deadliftkg: WeightKg,
    #[serde(rename(deserialize = "TotalKg"))]
    pub totalkg: WeightKg,
    #[serde(rename(deserialize = "Place"))]
    pub place: Place,
    #[serde(rename(deserialize = "Wilks"))]
    pub wilks: Points,
    #[serde(rename(deserialize = "McCulloch"))]
    pub mcculloch: Points,
    #[serde(rename(deserialize = "Glossbrenner"))]
    pub glossbrenner: Points,
    #[serde(rename(deserialize = "Goodlift"))]
    pub goodlift: Points,
    #[serde(rename(deserialize = "IPFPoints"))]
    pub ipfpoints: Points,
    #[serde(rename(deserialize = "Dots"))]
    pub dots: Points,
    #[serde(
        rename(deserialize = "Tested"),
        deserialize_with = "deserialize_yes_no"
    )]
    pub tested: bool,
    #[serde(rename(deserialize = "AgeClass"))]
    pub ageclass: AgeClass,
    #[serde(rename(deserialize = "BirthYearClass"))]
    pub birthyearclass: BirthYearClass,
    #[serde(rename(deserialize = "Country"))]
    pub lifter_country: Option<Country>,
    #[serde(rename(deserialize = "State"))]
    pub lifter_state: Option<State>,
}

impl Entry {
    /// Returns `max(best3squatkg, squat4kg)`.
    #[inline]
    pub fn highest_squatkg(&self) -> WeightKg {
        self.best3squatkg.max(self.squat4kg)
    }

    /// Returns `max(best3benchkg, bench4kg)`.
    #[inline]
    pub fn highest_benchkg(&self) -> WeightKg {
        self.best3benchkg.max(self.bench4kg)
    }

    /// Returns `max(best3deadliftkg, deadlift4kg)`.
    #[inline]
    pub fn highest_deadliftkg(&self) -> WeightKg {
        self.best3deadliftkg.max(self.deadlift4kg)
    }

    /// Borrows the Division string.
    #[inline]
    pub fn get_division(&self) -> Option<&str> {
        self.division.as_deref()
    }

    /// Calculates the Entry's points.
    pub fn points(&self, system: PointsSystem, units: WeightUnits) -> Points {
        let sex = self.sex;
        let bw = self.bodyweightkg;
        let total = self.totalkg;

        match system {
            PointsSystem::AH => coefficients::ah(sex, bw, total),
            PointsSystem::Dots => self.dots,
            PointsSystem::Glossbrenner => self.glossbrenner,
            PointsSystem::Goodlift => self.goodlift,
            PointsSystem::IPFPoints => self.ipfpoints,
            PointsSystem::McCulloch => self.mcculloch,
            PointsSystem::NASA => coefficients::nasa(bw, total),
            PointsSystem::Reshel => coefficients::reshel(sex, bw, total),
            PointsSystem::SchwartzMalone => coefficients::schwartzmalone(sex, bw, total),
            PointsSystem::Total => self.totalkg.as_type(units).as_points(),
            PointsSystem::Wilks => self.wilks,
            PointsSystem::Wilks2020 => coefficients::wilks2020(sex, bw, total),
        }
    }
}
