//! Checks that bodyweight changes over time are plausible.

use crate::checklib::consistency::{self, date, ConsistencyResult};
use crate::{AllMeetData, Entry, EntryIndex, LifterDataMap, LifterMap, Report};

// These are extremely loose right now, reasonable values result in having to fix a ludicrous number of errors
const WEIGHT_CUT_CHANGE_THRESHOLD: f32 = 1.5;
const TISSUE_CHANGE_THRESHOLD: f32 = 0.5;

/// Get the relative change in bodyweight from `a` to `b`
fn calc_relative_bw_change(entry_from: &Entry, entry_to: &Entry) -> f32 {
    // Handle division-by-zero cases early.
    if entry_from.bodyweightkg.is_zero() || entry_to.bodyweightkg.is_zero() {
        return 0.0;
    }

    // Get the absolute change in bodyweight over the interval.
    let from_bw = f32::from(entry_from.bodyweightkg);
    let to_bw = f32::from(entry_to.bodyweightkg);
    let bw_delta = f32::abs(from_bw - to_bw);

    // Express that delta as a relative change with respect to Entry `a`.
    bw_delta / from_bw
}

/// Checks bodyweight consistency for a single lifter.
pub fn check_bodyweight_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    report: &mut Report,
) -> ConsistencyResult {
    if consistency::should_skip_lifter(meetdata.entry(indices[0])) {
        return ConsistencyResult::Skipped;
    }

    // Allow manually excluding lifters through `lifter-data/bw-exemptions.csv`.
    let username = &meetdata.entry(indices[0]).username;
    if let Some(data) = lifterdata.get(username) {
        if data.exempt_bodyweight {
            return ConsistencyResult::Skipped;
        }
    }

    // Entries in the LifterMap are already sorted by date.
    // Sort the entries by date.
    let entries: Vec<&Entry> = indices.iter().map(|i| meetdata.entry(*i)).collect();

    let mut prev: &Entry = entries[0];
    for entry in entries.iter().skip(1) {
        // Ignore entries with missing bodyweight.
        if entry.bodyweightkg.is_zero() {
            continue;
        }

        let prev_date = date(prev);
        let this_date = date(entry);

        let interval_days = ((this_date - prev_date) as f32).abs();

        // Get the relative change in bodyweight between `prev` and `entry`,
        // we can check if this change makes sense given that a lifter could potentially do back to back 24hour weigh ins
        // and with a rate of long term bodyweight change
        let relative_change = calc_relative_bw_change(prev, entry);

        // Number of days that it would take to go from the minimum allowed bodyweight (15kg) to the maximum (300kg)
        let max_interval: f32 =
            (f32::log2(300.0) - f32::log2(15.0) - f32::log2(1.0 + WEIGHT_CUT_CHANGE_THRESHOLD))
                / (f32::log2(1.0 + TISSUE_CHANGE_THRESHOLD));

        if relative_change.abs()
            > (WEIGHT_CUT_CHANGE_THRESHOLD
                + f32::min(max_interval, interval_days) * TISSUE_CHANGE_THRESHOLD)
        {
            let days = this_date - prev_date;
            let plural = if days > 1 { "s" } else { "" };
            let msg = format!(
                "www.openpowerlifting.org/u/{} ranged [{}, {}] in {} day{}",
                entry.username, prev.bodyweightkg, entry.bodyweightkg, days, plural
            );
            report.warning(msg);
            return ConsistencyResult::Inconsistent;
        }

        prev = entry;
    }

    ConsistencyResult::Consistent
}

/// Checks bodyweight consistency for all lifters.
pub fn check_bodyweight_all(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    reports: &mut Vec<Report>,
) {
    let mut report = Report::new("[Bodyweight Consistency]".into());

    for lifter_indices in liftermap.values() {
        check_bodyweight_one(lifter_indices, meetdata, lifterdata, &mut report);
    }

    if report.has_messages() {
        reports.push(report);
    }
}
