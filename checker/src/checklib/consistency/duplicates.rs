//! checks for potential duplicate meets

use std::collections::HashMap;

use crate::checklib::consistency::{self, ConsistencyResult};
use crate::{AllMeetData, Entry, EntryIndex, LifterMap, Meet, Report};

/// find pairs of entries with the same total, same events entered,
/// same date, different paths, for a given lifter's array of EntryIndex
pub fn check_duplicates_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    reports: &mut Vec<Report>,
) -> ConsistencyResult {
    if consistency::should_skip_lifter(&meetdata.get_entry(indices[0])) {
        return ConsistencyResult::Skipped;
    }

    // EntryIndex implements Copy and Clone so we don't need references
    // as values in the HM
    let mut date_ei_map: HashMap<String, EntryIndex> = HashMap::new();

    for index in indices.iter() {
        let cur_entry: &Entry = meetdata.get_entry(*index);
        let cur_meet: &Meet = meetdata.get_meet(*index);
        let cur_date_str: String = cur_meet.date.to_string();

        match date_ei_map.get(&cur_date_str) {
            Some(match_ei) => {
                let match_meet: &Meet = meetdata.get_meet(*match_ei);
                let match_entry: &Entry = meetdata.get_entry(*match_ei);

                if (cur_entry.event == match_entry.event)
                    && (!cur_entry.totalkg.is_zero())
                    && (cur_entry.totalkg == match_entry.totalkg)
                    && (cur_meet.path != match_meet.path)
                {
                    let msg = format!(
                        "Possible duplicate meets for {} - total {} on {} in {} and {}",
                        cur_entry.name,
                        cur_entry.totalkg,
                        cur_meet.date,
                        cur_meet.path,
                        match_meet.path
                    );
                    let mut report = Report::new("[Consistency]".into());
                    report.warning(msg);
                    reports.push(report);
                }
            }
            None => {
                date_ei_map.insert(cur_date_str, *index);
            }
        }
    }

    if reports.len() > 0 {
        ConsistencyResult::Inconsistent
    } else {
        ConsistencyResult::Consistent
    }
}

/// Check for potential duplicate entries for all lifters
pub fn check_duplicates_all(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    reports: &mut Vec<Report>,
) {
    for lifter_indices in liftermap.values() {
        check_duplicates_one(lifter_indices, meetdata, reports);
    }
}
