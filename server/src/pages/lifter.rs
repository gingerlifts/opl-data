//! Logic for each lifter's personal page.

use opldb;
use opldb::fields;
use langpack::{self, Language};

/// The context object passed to `templates/lifter.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: String,
    pub lifter: &'a opldb::Lifter,
    pub lifter_sex: &'a str,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: opldb::WeightUnits,

    pub meet_results: Vec<MeetResultsRow<'a>>,
}

/// A row in the meet results table.
#[derive(Serialize)]
pub struct MeetResultsRow<'a> {
    pub place: String,
    pub federation: &'a fields::Federation,
    pub date: String,
    pub country: &'a str,
    pub state: Option<&'a str>,
    pub meet_name: &'a str,
    pub meet_path: &'a str,
    pub division: Option<&'a str>,
    pub age: fields::Age,
    pub equipment: &'a str,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub bodyweight: langpack::LocalizedWeightAny,

    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,
    pub wilks: langpack::LocalizedPoints,
}

impl<'a> MeetResultsRow<'a> {
    fn from(
        opldb: &'a opldb::OplDb,
        strings: &'a langpack::Translations,
        number_format: langpack::NumberFormat,
        units: opldb::WeightUnits,
        entry: &'a opldb::Entry,
    ) -> MeetResultsRow<'a> {
        let meet: &'a opldb::Meet = opldb.get_meet(entry.meet_id);

        MeetResultsRow {
            place: format!("{}", &entry.place),
            federation: &meet.federation,
            date: format!("{}", meet.date),
            country: &meet.country,
            state: match meet.state {
                None => None,
                Some(ref s) => Some(&s),
            },
            meet_name: &meet.name,
            meet_path: &meet.path,
            division: match entry.division {
                None => None,
                Some(ref s) => Some(&s),
            },
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
            wilks: entry.wilks.in_format(number_format),
        }
    }
}


impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        language: Language,
        langinfo: &'a langpack::LangInfo,
        units: opldb::WeightUnits,
        lifter_id: u32,
    ) -> Context<'a> {
        let lifter = opldb.get_lifter(lifter_id);
        let strings = langinfo.get_translations(language);
        let number_format = language.number_format();

        // Get a list of the entries for this lifter, oldest entries first.
        let mut entries = opldb.get_entries_for_lifter(lifter_id);
        entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

        let lifter_sex = strings.translate_sex(entries[0].sex);


        // Display the meet results, most recent first.
        let meet_results = entries
            .into_iter()
            .map(|e| {
                MeetResultsRow::from(opldb, strings, number_format, units, e)
            })
            .rev()
            .collect();

        Context {
            page_title: format!("{}", lifter.name),
            language: language,
            strings: strings,
            units: units,
            lifter: lifter,
            lifter_sex: lifter_sex,
            meet_results: meet_results,
        }
    }
}
