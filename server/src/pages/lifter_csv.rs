//! CSV data export for the Lifter page.

use csv;
use std::error;

use coefficients::mcculloch;
use opltypes::*;

use crate::opldb::{self, Entry, Lifter, Meet};

fn make_export_row<'a>(
    lifter: &'a Lifter,
    entry: &'a Entry,
    meet: &'a Meet,
) -> opltypes::ExportRow<'a> {
    // Convert from Option<String> to Option<&'a str> while hardcoding &'static "".
    let division: &'a str = if let Some(div) = &entry.division {
        div
    } else {
        ""
    };

    ExportRow {
        name: &lifter.name,
        sex: entry.sex,
        event: entry.event,
        equipment: entry.equipment,
        age: entry.age,
        ageclass: entry.ageclass,
        birthyearclass: entry.birthyearclass,
        division,
        bodyweightkg: entry.bodyweightkg,
        weightclasskg: entry.weightclasskg,
        squat1kg: entry.squat1kg,
        squat2kg: entry.squat2kg,
        squat3kg: entry.squat3kg,
        squat4kg: entry.squat4kg,
        best3squatkg: entry.best3squatkg,
        bench1kg: entry.bench1kg,
        bench2kg: entry.bench2kg,
        bench3kg: entry.bench3kg,
        bench4kg: entry.bench4kg,
        best3benchkg: entry.best3benchkg,
        deadlift1kg: entry.deadlift1kg,
        deadlift2kg: entry.deadlift2kg,
        deadlift3kg: entry.deadlift3kg,
        deadlift4kg: entry.deadlift4kg,
        best3deadliftkg: entry.best3deadliftkg,
        totalkg: entry.totalkg,
        place: entry.place,
        wilks: entry.wilks,
        mcculloch: mcculloch(entry.sex, entry.bodyweightkg, entry.totalkg, entry.age),
        glossbrenner: entry.glossbrenner,
        ipfpoints: entry.ipfpoints,
        tested: if entry.tested { "Yes" } else { "" },
        country: entry.lifter_country,
        federation: meet.federation,
        date: meet.date,
        meet_country: meet.country,
        meet_state: meet.state.clone(),
        meet_name: &meet.name,
    }
}

/// Export single-lifter data as a CSV file.
pub fn export_csv(
    opldb: &opldb::OplDb,
    lifter_id: u32,
    entry_filter: Option<fn(&opldb::OplDb, &Entry) -> bool>,
) -> Result<String, Box<dyn error::Error>> {
    let lifter = opldb.get_lifter(lifter_id);
    let mut entries = opldb.get_entries_for_lifter(lifter_id);

    // Filter and sort the entries, oldest entries first.
    if let Some(f) = entry_filter {
        entries = entries.into_iter().filter(|e| f(opldb, *e)).collect();
    }
    entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

    // Build the CSV output.
    let mut wtr = csv::Writer::from_writer(vec![]);
    for entry in entries.into_iter().rev() {
        let meet = opldb.get_meet(entry.meet_id);
        wtr.serialize(make_export_row(lifter, entry, meet))?;
    }

    Ok(String::from_utf8(wtr.into_inner()?)?)
}
