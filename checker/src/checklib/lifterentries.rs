use std::path::PathBuf;
use std::collections::HashSet;
use opltypes::{
    Date, WeightKg
}

use crate::{
    LifterMap, AllMeetData, LifterDataMap, Report, Entry
};

pub struct LifterEntriesCheckResult {
    pub reports: Vec<Report>
}

impl LifterEntriesCheckResult {

    pub fn new() -> LifterEntriesCheckResult {
 
       LifterEntriesCheckResult { reports: Vec::new() }
    }

    // we can borrow other seeing as we're going to drain
    // its reports vector
    pub fn push(&mut self, other: LifterEntriesCheckResult) {

        for report in other.reports {
            self.reports.push(report);
        }
    }
}


// run all the lifterentries checks in order
pub fn check_lifterentries(liftermap: &LifterMap, meetdata: &AllMeetData, lifterdata: &LifterDataMap, meet_data_root: PathBuf, lifterdir: &Path) -> LifterEntriesCheckResult {
 
    let mut ret_result = LifterEntriesCheckResult::new();

    let sex_err_result = check_sex_errors(liftermap, meetdata, lifterdata, meet_data_root.clone());
    let jp_name_result = check_japanese_names(liftermap, meetdata, meet_data_root.clone());
    let (bw_delta_usernames_result, bw_delta_exempt_usernames) = load_bw_delta_sanity_usernames(lifterdir);
    let check_bw_delta_result = check_bw_delta(bw_delta_exempt_usernames, liftermap, meetdata, lifterdata, meet_data_root.clone());

    ret_result.push(sex_err_result);
    ret_result.push(jp_name_result);
    ret_result.push(bw_delta_usernames_result);
    ret_result.push(check_bw_delta_result);

    ret_result
}



// check that changes in lifter's bodyweight between meets are sane
// we can take usernames, they aren't needed after this
fn check_bw_delta(exempt_usernames: HashSet<String>, liftermap: &LifterMap, meetdata: &AllMeetData, lifterdata: &LifterDataMap, meet_data_root: PathBuf) -> LifterEntriesCheckResult {
    

    for lifter_indices in liftermap.values() {

        let name = &meetdata.get_entry(lifter_indices[0]).name;
        let prev_entry_bw_date: Option<(&Entry, &WeightKg, &Date)> = None;

        // skip I. Nitial and Onename
        if name.contains(' ')
            && name.chars().skip(1).take(1).collect::<Vec<char>>() != ['.']
        {

            // sort the lifter's entries by meet date
            let lifter_entries_by_date: Vec<(&Entry, &Date)> = Vec::new();

            for index in lifter_indices.iter() {
                lifter_entries_by_date.push((meetdata.get_entry(*index), &meet_data.get_meet(*index).date));
            }

            // .cmp() wants the other value by ref
            lifter_entries.sort_unstable_by_key(|e, d| &d);

            // now iterate over the lifter's entries by date and check sanity of bodyweight
            // changes over time
            for entry_date_tup in &lifter_entries_by_date {
                let (entry, meet_date) = entry_date_tup;

                // if we have a previous entry for this lifter, compare it to the current one
                match lifter_prev_entry_bw_date {
                    Some(prev) => {

                        // ignore entries on the same date 
                        let (prev_entry, prev_bw, prev_date) = prev;

                        if prev_date != meet_date {
                            let bw_delta_pct: f32 = WeightKg::abs(&entry.bodyweightkg - prev_bw) * 100.0;
                            let date_delta = meet_date - prev_date;

                            //TODO - what type is date_delta and how do we get days?
                            //TODO - is this insane?  If so, by how far?
                            //TODO - if it's the lifter's worst so far, track that in a HashMap or
                            //something
                        }
                    }
                    None => (),
                }

                //TODO store refs in previous tuple
            }
        }
    }

    //TODO sort the HashMap of lifter -> worst bw delta error
    //TODO take the top n (define this up top of func) and report them
            


}


fn load_bw_delta_sanity_usernames(lifterdir: &Path) -> (LifterEntriesCheckResult, HashSet<String>) {

    let mut exempt_usernames: HashSet<String> = HashSet::new();
    let mut report = Report::new(lifterdir.join('bw-exemptions.csv'));

    // read the exemption usernames
    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: NameDisambiguationRow = result?;
        let username = match make_username(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(&username) {
            report.error_on(line, format!("Whitespace error in '{}'", &username));
        }

        if exempt_usernames.contains(&username) {
            report.error_on(line, format!("Lifter '{}' is duplicatied", &username));
        } else {
            exempt_usernames.insert(username);
        }
    }

    let mut reports: Vec<Report> = Vec::new();
    reports.push(report);

    (LifterEntriesCheckResult { reports }, exempt_usernames)
}


// check for consistency of each lifter's indicated sex in their entries
// inconsistencies are flagged as errors unless the lifter has an entry in the 
// sex exemptions file
fn check_sex_errors(liftermap: &LifterMap, meetdata: &AllMeetData, lifterdata: &LifterDataMap, meet_data_root: PathBuf) -> LifterEntriesCheckResult {

    let mut reports: Vec<Report> = Vec::new();

    for lifter_indices in liftermap.values() {
        
        let name = &meetdata.get_entry(lifter_indices[0]).name;

        // skip I. Nitial and Onename 
        if name.contains(' ')
            && name.chars().skip(1).take(1).collect::<Vec<char>>() != ['.']
        {
            let expected_sex = meetdata.get_entry(lifter_indices[0]).sex;

            for index in lifter_indices.iter().skip(1) {
                let sex = meetdata.get_entry(*index).sex;

                if sex != expected_sex {
                    let mut suppress_error = false;

                    let username = &meetdata.get_entry(*index).username;
                    if let Some(data) = lifterdata.get(username) {
                        if data.exempt_sex {
                            suppress_error = true;
                        }
                    }

                    if !suppress_error {
                        let url =
                            format!("https://www.openpowerlifting.org/u/{}", username);
                        let msg = format!("Sex conflict for '{}' - {}", name, url);
                        let mut report = Report::new(meet_data_root.clone());
                        report.error(msg);
                        reports.push(report);
                    }
                    break;
                }
            }
        }
    }

    LifterEntriesCheckResult { reports }
}

// Check that lifters with Japanese names have them used consistently across their entries
fn check_japanese_names(liftermap: &LifterMap, meetdata: &AllMeetData, meet_data_root: PathBuf) -> LifterEntriesCheckResult {

    let mut reports: Vec<Report> = Vec::new();

    for lifter_indices in liftermap.values() {

        let name = &meetdata.get_entry(lifter_indices[0]).name;
        let mut japanesename = &meetdata.get_entry(lifter_indices[0]).japanesename;

        for index in lifter_indices.iter().skip(1) {
            let entry = &meetdata.get_entry(*index);

            // The Name field must exactly match for the same username.
            if name != &entry.name {
                let msg = format!(
                    "Conflict for '{}': '{}' vs '{}'",
                    entry.username, name, entry.name
                );
                let mut report = Report::new(meet_data_root.clone());
                report.error(msg);
                reports.push(report);
            }

            // If this is the first time seeing a JapaneseName, remember it.
            if japanesename.is_none() && entry.japanesename.is_some() {
                japanesename = &entry.japanesename;
            }

            // Otherwise, they should match.
            if let Some(jp_name) = japanesename {
                if let Some(entry_jp_name) = &entry.japanesename {
                    if jp_name != entry_jp_name {
                        let msg = format!(
                            "Conflict for {}: '{}' vs '{}'",
                            entry.username, jp_name, entry_jp_name
                        );
                        let mut report = Report::new(meet_data_root.clone());
                        report.error(msg);
                        reports.push(report);
                    }
                }
            }
        }
    }
    LifterEntriesCheckResult { reports }
}


