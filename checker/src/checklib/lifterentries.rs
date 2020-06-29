use std::path::{
    Path, PathBuf
};
use std::collections::{
    HashSet, HashMap
};
use usernames::{
    make_username
};

use opltypes::{
    Date, WeightKg
};
use crate::{
    LifterMap, AllMeetData, LifterDataMap, Report, Entry
};
use crate::checklib::lifterdata::{
    has_whitespace_errors
};

#[derive(Deserialize)]
struct BwExemptionRow {
    #[serde(rename = "Name")]
    pub name: String,
}


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


//check a bodyweight/time delta to see if it's sane.  Return a Result with how far from the limit
//it was
fn check_individual_bw_delta(first_entry: &Entry, second_entry: &Entry, delta_days: u32) -> Result<f32, f32> {

    let first_bw: WeightKg = first_entry.bodyweightkg;
    let second_bw: WeightKg = second_entry.bodyweightkg;
    let bw_delta: WeightKg = WeightKg::abs(first_bw - second_bw);

    let bw_pct_delta: f32 = (bw_delta / first_bw) * 100.0;
    let bw_pct_delta_per_day: f32 = bw_pct_delta / delta_days;

    // 25% represents a lifter in old weight classes in a meet we only have weight classes for and
    // no actual weigh in data, competing in 125kg one day and 100kg the next or vice versa
    // (potentially 99kg and 101kg).  This drops off over a few days through the range where
    // lifters could cut for one meet and compete again soon without cutting, and stabilises
    // into a near-constant rate of sustainable weight loss
    let max_bw_pct_delta_per_day: f32 = (25.0 / delta_days) + 0.15;

    if bw_pct_delta_per_day > max_bw_pct_delta_per_day {
        Err(bw_pct_delta_per_day - max_bw_pct_delta_per_day)
    } else {
        Ok(max_bw_pct_delta_per_day - bw_pct_delta_per_day)
    }
}


// check that changes in lifter's bodyweight between meets are sane
// we can take usernames, they aren't needed after this
fn check_bw_delta(exempt_usernames: HashSet<String>, liftermap: &LifterMap, meetdata: &AllMeetData, lifterdata: &LifterDataMap, meet_data_root: PathBuf) -> LifterEntriesCheckResult {
    
    let mut lifter_worst_err_map: HashMap<String, (WeightKg, WeightKg, u32, f32)> = HashMap::new();

    for lifter_indices in liftermap.values() {

        let name = &meetdata.get_entry(lifter_indices[0]).name;
        let mut prev_entry: Option<&Entry> = None;

        // skip I. Nitial and Onename
        if name.contains(' ')
            && name.chars().skip(1).take(1).collect::<Vec<char>>() != ['.']
        {

            // sort the lifter's entries by meet date
            let mut lifter_entries_by_date: Vec<&Entry> = Vec::new();

            for index in lifter_indices.iter() {
                lifter_entries_by_date.push(meetdata.get_entry(*index));
            }

            // .cmp() wants the other value by ref
            lifter_entries_by_date.sort_unstable_by_key(|e| meetdata.get_meet(*e.index).date);

            // now iterate over the lifter's entries by date and check sanity of bodyweight
            // changes over time
            for entry in &lifter_entries_by_date {

                // if we have a previous entry for this lifter, compare it to the current one
                match prev_entry {
                    Some(prev) => {

                        let prev_entry_date: Date = meetdata.get_meet(*prev_entry.index).date;  
                        let entry_date: Date = meetdata.get_meet(*entry.index).date;

                        let delta_days: u32 = (entry_date - prev_entry_date) / (60 * 60 * 24);

                        if delta_days > 0 {
                            // is this insane?  If so, by how far?
                            match check_individual_bw_delta(prev_entry, entry, delta_days) {
                                Ok(err) => (),
                                Err(err) => {

                                    // if it's the lifter's worst so far, track it
                                    match lifter_worst_err_map.get(name) {
                                        Some(worst) => {
                                            if err > worst.err {
                                                lifter_worst_err_map.insert(name, (prev_entry.bodyweightkg, entry.bodyweightkg, delta_days, err));
                                            }
                                        }
                                        None => {
                                            lifter_worst_err_map.insert(name, (prev_entry.bodyweightkg, entry.bodyweightkg, delta_days, err));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // no previous entry, must be first entry, keep on truckin
                    None => (),
                }
                prev_entry = entry;
            }
        }
    }

    //sort the HashMap of lifter -> worst bw delta error
    //take the top n (define this up top of func) and report them
    let mut sorted_lifter_worst_bw_delta: Vec<(String, WeightKg, WeightKg, u32, f32)> = lifter_worst_err_map.iter().collect();
    sorted_lifter_worst_bw_delta.sort_unstable_by_key(|n, a, b, e, d| e).reverse();

    let mut result = LifterEntriesCheckResult::new();

    for i in 0..10 {
        let (name, first_weight, second_weight, delta_days, err) = sorted_lifter_worst_bw_delta[i];
        let mut report = Report::new(meet_data_root.clone());
        let msg = format!("Anomalous bodyweight change for '{}' - from {} to {} in {} days", 
            name, first_weight, second_weight, delta_days
        );
        report.warning(msg);
        result.reports.push(report);
    }         

    result

}


fn load_bw_delta_sanity_usernames(lifterdir: &Path) -> (LifterEntriesCheckResult, HashSet<String>) {

    let mut exempt_usernames: HashSet<String> = HashSet::new();
    let mut report = Report::new(lifterdir.join("bw-exemptions.csv"));

    // read the exemption usernames
    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: BwExemptionRow = result?;
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
            report.error_on(line, format!("Lifter '{}' is duplicated", &username));
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


