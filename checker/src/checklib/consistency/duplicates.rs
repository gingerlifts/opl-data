//! checks for potential duplicate meets

use crate::checklib::consistency::{self, get_date, ConsistencyResult};
use crate::{AllMeetData, Entry, EntryIndex, LifterMap, Meet, Report};

/// find pairs of entries with the same total, same events entered,
/// same date, different paths, for a given lifter's array of EntryIndex
pub fn check_duplicates_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    reports: &mut Vec<Report>,
    ei_by_date: &mut Vec<EntryIndex>,
) -> ConsistencyResult {
    if consistency::should_skip_lifter(&meetdata.get_entry(indices[0])) {
        return ConsistencyResult::Skipped;
    }

    let mut result = ConsistencyResult::Consistent;
    let username = &meetdata.get_entry(indices[0]).username;

    for index in indices.iter() {
        ei_by_date.push(*index);
    }

    let date_sort_closure = |ei_a: &EntryIndex, ei_b: &EntryIndex| {
        let entry_a = meetdata.get_entry(*ei_a);
        let entry_b = meetdata.get_entry(*ei_b);
        let date_a = get_date(meetdata, &entry_a);
        let date_b = get_date(meetdata, &entry_b);

        date_a.cmp(&date_b)
    };

    ei_by_date.sort_unstable_by(|a, b| date_sort_closure(a, b));

    // since ei_by_date is sorted, EntryIndex i has potential
    // matches from i+1 to .len().
    // The last EntryIndex will either
    // have been included in matches already, or have no matches,
    // so it's excluded from the outer loop
    for i in 0..ei_by_date.len() - 1 {
        let cur_entry: &Entry = meetdata.get_entry(ei_by_date[i]);
        let cur_meet: &Meet = meetdata.get_meet(ei_by_date[i]);

        for inner_i in i + 1..ei_by_date.len() {
            let match_entry: &Entry = meetdata.get_entry(ei_by_date[inner_i]);
            let match_meet: &Meet = meetdata.get_meet(ei_by_date[inner_i]);

            if cur_meet.date == match_meet.date {
                if (cur_entry.event == match_entry.event)
                    && (!cur_entry.totalkg.is_zero())
                    && (cur_entry.totalkg == match_entry.totalkg)
                    && (cur_meet.path != match_meet.path)
                {
                    let msg = format!(
                        "Possible duplicate meets for {} - total {} on {} in {} and {}",
                        username,
                        cur_entry.totalkg,
                        cur_meet.date,
                        cur_meet.path,
                        match_meet.path
                    );
                    let mut report = Report::new("[Consistency]".into());
                    report.warning(msg);
                    reports.push(report);
                    result = ConsistencyResult::Inconsistent;
                }

            // if we find an entry with a non-match date, we won't have any more
            } else {
                break;
            }
        }
    }

    result
}

/// Check for potential duplicate entries for all lifters
pub fn check_duplicates_all(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    reports: &mut Vec<Report>,
) {
    let mut ei_by_date: Vec<EntryIndex> = Vec::new();

    for lifter_indices in liftermap.values() {
        check_duplicates_one(lifter_indices, meetdata, reports, &mut ei_by_date);
        ei_by_date.clear();
    }
}
