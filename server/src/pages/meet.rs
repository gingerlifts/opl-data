//! Logic for each meet's individual results page.

use itertools::Itertools;
use opltypes::*;

use std::str::FromStr;

use langpack::{self, get_localized_name, Language, Locale, LocalizeNumber};
use opldb::{self, algorithms};

/// The context object passed to `templates/meet.html.tera`
#[derive(Serialize)]
pub struct Context<'db> {
    pub page_title: String,
    pub meet: MeetInfo<'db>,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: WeightUnits,
    pub points_column_title: &'db str,

    // Instead of having the JS try to figure out how to access
    // other sorts, just tell it what the paths are.
    pub path_if_by_wilks: String,
    pub path_if_by_glossbrenner: String,
    pub path_if_by_division: String,

    /// True iff the meet reported any age data.
    pub has_age_data: bool,
    pub sortselection: MeetSortSelection,
    pub rows: Vec<ResultsRow<'db>>,
}

/// A sort selection widget just for the meet page.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum MeetSortSelection {
    ByDivision,
    ByGlossbrenner,
    ByWilks,
}

impl FromStr for MeetSortSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "by-division" => Ok(MeetSortSelection::ByDivision),
            "by-glossbrenner" => Ok(MeetSortSelection::ByGlossbrenner),
            "by-wilks" => Ok(MeetSortSelection::ByWilks),
            _ => Err(()),
        }
    }
}

#[derive(Serialize)]
pub struct MeetInfo<'a> {
    pub path: &'a str,
    pub federation: Federation,
    pub date: String,
    pub country: &'a str,
    pub state: Option<&'a str>,
    pub town: Option<&'a str>,
    pub name: &'a str,
}

impl<'a> MeetInfo<'a> {
    pub fn from(
        meet: &'a opldb::Meet,
        strings: &'a langpack::Translations,
    ) -> MeetInfo<'a> {
        MeetInfo {
            path: &meet.path,
            federation: meet.federation,
            date: format!("{}", &meet.date),
            country: strings.translate_country(meet.country),
            state: match meet.state {
                None => None,
                Some(ref s) => Some(&s),
            },
            town: match meet.town {
                None => None,
                Some(ref s) => Some(&s),
            },
            name: &meet.name,
        }
    }
}

/// A row in the results table.
#[derive(Serialize)]
pub struct ResultsRow<'a> {
    /// The Place given by the federation.
    pub place: String,
    /// The rank in the ranking-by-points view (by Wilks).
    pub rank: u32,
    pub localized_name: &'a str,
    pub lifter: &'a opldb::Lifter,
    pub sex: &'a str,
    pub age: Age,
    pub equipment: &'a str,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub bodyweight: langpack::LocalizedWeightAny,

    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,
    pub points: langpack::LocalizedPoints,
}

impl<'a> ResultsRow<'a> {
    fn from(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        sort: MeetSortSelection,
        entry: &'a opldb::Entry,
        rank: u32,
    ) -> ResultsRow<'a> {
        let lifter: &'a opldb::Lifter = opldb.get_lifter(entry.lifter_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        ResultsRow {
            place: format!("{}", &entry.place),
            rank,
            localized_name: get_localized_name(&lifter, locale.language),
            lifter,
            sex: strings.translate_sex(entry.sex),
            age: entry.age,
            equipment: strings.translate_equipment(entry.equipment),
            weightclass: entry.weightclasskg.as_type(units).in_format(number_format),
            bodyweight: entry.bodyweightkg.as_type(units).in_format(number_format),

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
            points: match sort {
                MeetSortSelection::ByDivision
                | MeetSortSelection::ByWilks => entry.wilks.in_format(number_format),
                MeetSortSelection::ByGlossbrenner => {
                    entry.glossbrenner.in_format(number_format)
                }
            },
        }
    }
}

/// A table of results. There can be multiple tables on one page.
#[derive(Serialize)]
pub struct ResultsTable<'db> {
    pub rows: Vec<ResultsRow<'db>>,
    pub title: String,
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        meet_id: u32,
        sort: MeetSortSelection,
    ) -> Option<Context<'a>> {
        let meets = opldb.get_meets();
        let meet = opldb.get_meet(meet_id);

        // Display at most one entry for each lifter.
        let groups = opldb
            .get_entries_for_meet(meet_id)
            .into_iter()
            .group_by(|e| e.lifter_id);

        let mut entries: Vec<&opldb::Entry> = groups
            .into_iter()
            .map(|(_key, group)| group.max_by_key(|x| x.wilks).unwrap())
            .collect();

        // Does this meet contain age data?
        // If not, that column will be hidden.
        let mut has_age_data = false;
        for entry in &entries {
            if entry.age != Age::None {
                has_age_data = true;
                break;
            }
        }

        match sort {
            MeetSortSelection::ByDivision => return None,
            MeetSortSelection::ByGlossbrenner => {
                entries.sort_unstable_by(|a, b| {
                    algorithms::cmp_glossbrenner(&meets, a, b)
                });
            }
            MeetSortSelection::ByWilks => {
                entries.sort_unstable_by(|a, b| {
                    algorithms::cmp_wilks(&meets, a, b)
                });
            }
        };

        let points_column_title = match sort {
            MeetSortSelection::ByDivision
            | MeetSortSelection::ByWilks => &locale.strings.columns.wilks,
            MeetSortSelection::ByGlossbrenner => &locale.strings.columns.glossbrenner,
        };

        let rows = entries
            .into_iter()
            .zip(1..)
            .map(|(e, i)| ResultsRow::from(opldb, locale, sort, e, i))
            .collect();

        Some(Context {
            page_title: format!("{} {} {}", meet.date.year(), meet.federation, meet.name),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            points_column_title,
            sortselection: sort,
            meet: MeetInfo::from(&meet, locale.strings),
            has_age_data,
            rows,
            path_if_by_wilks: format!("/m/{}", meet.path.to_string()),
            path_if_by_glossbrenner: format!("/m/{}/by-glossbrenner", meet.path),
            path_if_by_division: format!("/m/{}/by-division", meet.path),
        })
    }
}
