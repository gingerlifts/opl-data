//! A collection of datatypes used in the OpenPowerlifting database.

#![feature(const_fn)]

// External dependencies.
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;

// External dependencies used only in testing code.
#[cfg(test)]
#[macro_use]
extern crate serde_json;

// Public exports.
mod age;
pub use self::age::{Age, PrettyAge};
mod ageclass;
pub use self::ageclass::AgeClass;
mod birthyearclass;
pub use self::birthyearclass::BirthYearClass;
mod country;
pub use self::country::Country;
mod date;
pub use self::date::Date;
mod equipment;
pub use self::equipment::Equipment;
mod event;
pub use self::event::Event;
mod export;
pub use self::export::ExportRow;
mod federation;
pub use self::federation::Federation;
mod place;
pub use self::place::Place;
mod points;
pub use self::points::{Points, PointsSystem};
mod ruleset;
pub use self::ruleset::{Rule, RuleSet};
mod sex;
pub use self::sex::Sex;
mod state;
pub use self::state::*;
mod weightkg;
pub use self::weightkg::{WeightAny, WeightKg};
mod weightclasskg;
pub use self::weightclasskg::{WeightClassAny, WeightClassKg};
mod yesno;
pub use self::yesno::deserialize_yes_no;

/// Units of weight.
#[derive(Copy, Clone, Debug, EnumString, Serialize)]
pub enum WeightUnits {
    #[strum(serialize = "kg")]
    Kg,
    #[strum(serialize = "lbs")]
    Lbs,
}
