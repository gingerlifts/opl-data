//! Types for raw data interchange from Rust to JS.

use opltypes::*;
use serde::ser::{Serialize, SerializeSeq, Serializer};

use langpack::{self, LocalizeNumber, get_localized_name, Locale};
use opldb::{Entry, OplDb};
use pages::selection::SortSelection;

pub struct JsEntryRow<'db> {
    pub sorted_index: u32,

    pub name: &'db str,
    pub username: &'db str,
    pub instagram: &'db Option<String>,
    pub vkontakte: &'db Option<String>,
    pub color: &'db Option<String>,
    pub flair: &'db Option<String>,

    pub federation: Federation,
    pub date: String,
    pub country: &'db str,
    pub state: &'db Option<String>,
    pub path: &'db str,

    pub sex: &'db str,
    pub equipment: &'db str,
    pub age: Age,
    pub division: &'db Option<String>,
    pub bodyweight: langpack::LocalizedWeightAny,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,

    /// Any kind of points: Wilks, McCulloch, etc.
    /// Only one points system is used at a time.
    pub points: langpack::LocalizedPoints,
}

/// Serialize to a compact but definitely less-helpful format
/// for JS interchange.
impl<'db> Serialize for JsEntryRow<'db> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.sorted_index)?;

        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.username)?;
        seq.serialize_element(&self.instagram)?;
        seq.serialize_element(&self.vkontakte)?;
        seq.serialize_element(&self.color)?;
        seq.serialize_element(&self.flair)?;

        seq.serialize_element(&self.federation)?;
        seq.serialize_element(&self.date)?;
        seq.serialize_element(&self.country)?;
        seq.serialize_element(&self.state)?;
        seq.serialize_element(&self.path)?;

        seq.serialize_element(&self.sex)?;
        seq.serialize_element(&self.equipment)?;
        seq.serialize_element(&self.age)?;
        seq.serialize_element(&self.division)?;
        seq.serialize_element(&self.bodyweight)?;
        seq.serialize_element(&self.weightclass)?;
        seq.serialize_element(&self.squat)?;
        seq.serialize_element(&self.bench)?;
        seq.serialize_element(&self.deadlift)?;
        seq.serialize_element(&self.total)?;
        seq.serialize_element(&self.points)?;

        seq.end()
    }
}

impl<'db> JsEntryRow<'db> {
    pub fn from(
        opldb: &'db OplDb,
        locale: &'db Locale,
        entry: &'db Entry,
        sorted_index: u32,
        sort: SortSelection,
    ) -> JsEntryRow<'db> {
        let meet = opldb.get_meet(entry.meet_id);
        let lifter = opldb.get_lifter(entry.lifter_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        JsEntryRow {
            sorted_index,

            name: get_localized_name(lifter, locale.language),
            username: &lifter.username,
            instagram: &lifter.instagram,
            vkontakte: &lifter.vkontakte,
            color: &lifter.color,
            flair: &lifter.flair,

            federation: meet.federation,
            date: format!("{}", meet.date),
            country: strings.translate_country(meet.country),
            state: &meet.state,
            path: &meet.path,

            sex: strings.translate_sex(entry.sex),
            equipment: strings.translate_equipment(entry.equipment),
            age: entry.age,
            division: &entry.division,
            bodyweight: entry.bodyweightkg.as_type(units).in_format(number_format),
            weightclass: entry.weightclasskg.as_type(units).in_format(number_format),
            squat: entry
                .highest_squatkg()
                .as_type(units)
                .in_format(number_format),
            bench: entry
                .highest_benchkg()
                .as_type(units)
                .in_format(number_format),
            deadlift: entry
                .highest_deadliftkg()
                .as_type(units)
                .in_format(number_format),
            total: entry.totalkg.as_type(units).in_format(number_format),

            // This should mirror the logic in pages::rankings::Context::new().
            points: match sort {
                SortSelection::BySquat
                | SortSelection::ByBench
                | SortSelection::ByDeadlift
                | SortSelection::ByTotal
                | SortSelection::ByWilks => entry.wilks.in_format(number_format),
                SortSelection::ByMcCulloch => entry.mcculloch.in_format(number_format),
                SortSelection::ByGlossbrenner => entry.glossbrenner.in_format(number_format),
            },
        }
    }
}
