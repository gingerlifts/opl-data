//! Implementation of Country interpolation.

use colored::*;
use opltypes::Country;

use crate::{AllMeetData, EntryIndex, LifterMap};

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_found_initial(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Found initial Country".green(),
            country,
            "in".green(),
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_matched(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Matched Country".green(),
            country,
            "in".green(),
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_conflict(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Conflict with Country".bold().red(),
            country,
            "in".bold().red(),
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_inferred(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Inferred Country".bold().green(),
            country,
            "in".bold().green(),
            path.as_ref().unwrap()
        );
    }
}

/// Returns a single Country that is consistent for all the Entries.
fn get_consistent_country(
    meetdata: &AllMeetData,
    indices: &[EntryIndex],
    debug: bool,
) -> Option<Country> {
    let mut acc: Option<Country> = None;

    for &index in indices {
        // Get the MeetPath for more helpful debugging output.
        let path: Option<String> = if debug {
            Some(meetdata.get_meet(index).path.clone())
        } else {
            None
        };

        let entry = meetdata.get_entry(index);

        if let Some(country) = entry.country {
            if let Some(acc_country) = acc {
                if country != acc_country {
                    // Countries within the UK are compatible with Country::UK.
                    if country == Country::UK && acc_country.is_in_uk() {
                        trace_matched(debug, country, &path);
                    } else if acc_country == Country::UK && country.is_in_uk() {
                        acc = Some(country);
                        trace_matched(debug, country, &path);
                    } else {
                        trace_conflict(debug, country, &path);
                        return None;
                    }
                } else {
                    trace_matched(debug, country, &path);
                }
            } else {
                trace_found_initial(debug, country, &path);
                acc = Some(country);
            }
        }
    }
    acc
}

/// Country interpolation for a single lifter's entries.
fn interpolate_country_single_lifter(
    meetdata: &mut AllMeetData,
    indices: &[EntryIndex],
    debug: bool,
) {
    if let Some(country) = get_consistent_country(&meetdata, &indices, debug) {
        for &index in indices {
            // Get the MeetPath for more helpful debugging output.
            let path: Option<String> = if debug {
                Some(meetdata.get_meet(index).path.clone())
            } else {
                None
            };

            trace_inferred(debug, country, &path);
            meetdata.get_entry_mut(index).country = Some(country);
        }
    }
}

/// Public-facing entry point for debugging a single lifter's interpolation.
pub fn interpolate_country_debug_for(
    meetdata: &mut AllMeetData,
    liftermap: &LifterMap,
    username: &str,
) {
    match liftermap.get(username) {
        Some(indices) => interpolate_country_single_lifter(meetdata, indices, true),
        None => println!("Username '{}' not found", username),
    }
}

/// Attempts to infer a Country for a lifter from surrounding Entry data.
pub fn interpolate_country(meetdata: &mut AllMeetData, liftermap: &LifterMap) {
    for indices in liftermap.values() {
        // Interpolation requires multiple entries.
        if indices.len() >= 2 {
            interpolate_country_single_lifter(meetdata, indices, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checklib::{Entry, Meet};
    use crate::SingleMeetData;

    /// Helper function to generate a single-meet AllMeetData struct
    /// from a list of entries.
    fn meetdata_from_vec(entries: Vec<Entry>) -> AllMeetData {
        let meet = Meet::test_default();
        let singlemeetdata = SingleMeetData { meet, entries };
        AllMeetData::from(vec![singlemeetdata])
    }

    /// If no entries have a set Country, interpolation should not do anything.
    #[test]
    fn all_none() {
        let entries = vec![Entry::default(), Entry::default()];
        let mut meetdata = meetdata_from_vec(entries);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.get_entry_at(0, 0).country, None);
        assert_eq!(meetdata.get_entry_at(0, 1).country, None);
    }

    /// If only one entry has a set Country, propagate that Country.
    #[test]
    fn one_some() {
        let usa = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut meetdata =
            meetdata_from_vec(vec![Entry::default(), usa, Entry::default()]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.get_entry_at(0, 0).country, Some(Country::USA));
        assert_eq!(meetdata.get_entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.get_entry_at(0, 2).country, Some(Country::USA));
    }

    /// If two entries conflict, don't propagate a Country.
    #[test]
    fn conflict() {
        let usa = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let russia = Entry {
            country: Some(Country::Russia),
            ..Entry::default()
        };

        let mut meetdata =
            meetdata_from_vec(vec![Entry::default(), usa, Entry::default(), russia]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.get_entry_at(0, 0).country, None);
        assert_eq!(meetdata.get_entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.get_entry_at(0, 2).country, None);
        assert_eq!(meetdata.get_entry_at(0, 3).country, Some(Country::Russia));
    }

    /// Countries within the UK are compatible with Country:UK.
    #[test]
    fn uk_subsets() {
        let uk = Entry {
            country: Some(Country::UK),
            ..Entry::default()
        };
        let scotland = Entry {
            country: Some(Country::Scotland),
            ..Entry::default()
        };

        let mut meetdata =
            meetdata_from_vec(vec![Entry::default(), uk, Entry::default(), scotland]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.get_entry_at(0, 0).country, Some(Country::Scotland));
        assert_eq!(meetdata.get_entry_at(0, 1).country, Some(Country::Scotland));
        assert_eq!(meetdata.get_entry_at(0, 2).country, Some(Country::Scotland));
        assert_eq!(meetdata.get_entry_at(0, 3).country, Some(Country::Scotland));
    }
}
