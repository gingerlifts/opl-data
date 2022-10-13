//! Implementation of Country (and State) interpolation.

use std::collections::VecDeque;
use colored::*;
use opltypes::{Country, Username};

use crate::{AllMeetData, Entry, EntryIndex, LifterMap};

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

/// Get the country that should be considered current given
/// the lifter's previous country, and an entry 
/// we supply the previous country so that we can correctly
/// handle UK-style country containment, eg: a Scottish lifter
/// subsequently entered as UK stays Scottish
fn current_country(
    entry: &Entry,
    prev_country: Option<Country>,
    path: &Option<String>,
    debug: bool
) -> Option<Country> {

    match (entry.country, prev_country) {
        (Some(country), None) => {
            trace_found_initial(debug, country, path);
            return Some(country);
        },
        // we prefer the contained, more specific country
        (Some(country), Some(prev)) => {
            if country == prev || country.contains(prev) {
                trace_matched(debug, prev, path);
                return Some(prev)
            } else if prev.contains(country) {
                trace_matched(debug, country, path);
                return Some(country);
            } else {
                trace_conflict(debug, country, path);
                return Some(country);
            }
        },
        (None, Some(prev)) => {
            trace_inferred(debug, prev, path);
            return Some(prev);
        },
        (None, None) => None
    }
}

/// Track countries specified in lifter's entries with their ordinal index
/// so we can fill in the gaps later
fn make_entry_num_country_map(
    meetdata: &AllMeetData,
    indices: &[EntryIndex],
    debug: bool
) -> VecDeque<(u16, Country)> {

    let mut entry_num_countries: VecDeque<(u16, Country)> = VecDeque::new();
    let mut cur_country: Option<Country> = None;
    let mut prev_country: Option<Country> = cur_country;

    let mut entry_i: u16 = 0;

    for &index in indices {
        let entry = meetdata.entry(index);

        // Get the MeetPath for more helpful debugging output.
        let path: Option<String> = if debug {
            Some(meetdata.meet(index).path.clone())
        } else {
            None
        };

        cur_country = current_country(entry, prev_country, &path, debug);

        match (cur_country, prev_country) {
            (Some(cur), Some(prev)) => {
                // if prev_country contains cur_country, change the previous
                // map entry to cur_country eg: we have UK, Scotland, then
                // UK->Scotland
                if prev.contains(cur) {
                    let (prev_vec_entry_i, _) = entry_num_countries.pop_back().unwrap();
                    entry_num_countries.push_back((prev_vec_entry_i, cur));
                }
                // countries changed
                else if prev != cur {
                    entry_num_countries.push_back((entry_i, cur));
                }
            },
            // previous and no current, leave it alone
            (None, Some(_prev)) => (),
            // current and no previous, first country
            (Some(cur), None) => {
                entry_num_countries.push_back((entry_i, cur));
            },
            // no previous and no current, leave it alone
            (None, None) => ()
        }
        entry_i += 1; 
        prev_country = cur_country;
    }
    entry_num_countries
}

/// Country interpolation, fill in gaps
fn interpolate_country_single_lifter(
    meetdata: &mut AllMeetData,
    indices: &[EntryIndex],
    debug: bool
) {
    let mut entry_num_countries = make_entry_num_country_map(meetdata, indices, debug);
    
    // bail early if there are no countries
    if entry_num_countries.len() < 1 {
        return;
    }

    // get the first country now so we can fill in until the second
    let (_, mut cur_country) = entry_num_countries.pop_front().unwrap();

    // get the next so we know when we're there
    let mut next_entry_i: Option<u16> = None;
    let mut next_country: Option<Country> = None;
    match entry_num_countries.pop_front() {
        Some((_next_entry_i, _next_country)) => {
            (next_entry_i, next_country) = (Some(_next_entry_i), Some(_next_country));
        },
        None => ()
    }

    let mut entry_i: u16 = 0;
    for &index in indices {

        // if we're up to a country change, do so
        if next_entry_i.is_some() && next_country.is_some() && Some(entry_i) == next_entry_i {
            cur_country = next_country.unwrap();

            match entry_num_countries.pop_front() {
                Some((_next_entry_i, _next_country)) => {
                    (next_entry_i, next_country) = (Some(_next_entry_i), Some(_next_country));
                },
                None => {
                    (next_entry_i, next_country) = (None, None);
                }

            }
        }

        meetdata.entry_mut(index).country = Some(cur_country);
        entry_i += 1;
    }
}

/// Public-facing entry point for debugging a single lifter's interpolation.
pub fn interpolate_country_debug_for(
    meetdata: &mut AllMeetData,
    liftermap: &LifterMap,
    username: &Username,
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

        assert_eq!(meetdata.entry_at(0, 0).country, None);
        assert_eq!(meetdata.entry_at(0, 1).country, None);
    }

    /// If only one entry has a set Country, propagate that Country.
    #[test]
    fn one_some() {
        let usa = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut meetdata = meetdata_from_vec(vec![Entry::default(), usa, Entry::default()]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 2).country, Some(Country::USA));
    }

    /// If there are multiple countries, use the first country from the first 
    /// entry until the second country appears, and so on
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
        let aus: Entry = Entry {
            country: Some(Country::Australia),
            ..Entry::default()
        };

        let mut meetdata = meetdata_from_vec(vec![Entry::default(), usa, Entry::default(), russia, aus, Entry::default()]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 2).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 3).country, Some(Country::Russia));
        assert_eq!(meetdata.entry_at(0, 4).country, Some(Country::Australia));
        assert_eq!(meetdata.entry_at(0, 5).country, Some(Country::Australia));

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

        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::Scotland));
        assert_eq!(meetdata.entry_at(0, 1).country, Some(Country::Scotland));
        assert_eq!(meetdata.entry_at(0, 2).country, Some(Country::Scotland));
        assert_eq!(meetdata.entry_at(0, 3).country, Some(Country::Scotland));
    }
}
