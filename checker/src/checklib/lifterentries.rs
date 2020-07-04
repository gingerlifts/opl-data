use std::path::{
    Path, PathBuf
};
use std::collections::{
    HashSet, HashMap
};

use std::cmp::{
    Ordering
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

#[derive(Default)]
pub struct LifterEntriesCheckResult {
    pub reports: Vec<Report>
}

impl LifterEntriesCheckResult {

    // we can borrow other seeing as we're going to drain
    // its reports vector
    pub fn append(&mut self, mut other: LifterEntriesCheckResult) {

        self.reports.append(&mut other.reports);
    }
}


// run all the lifterentries checks in order
pub fn check_lifterentries(liftermap: &LifterMap, meetdata: &AllMeetData, lifterdata: &LifterDataMap, meet_data_root: PathBuf, lifterdir: &Path) -> LifterEntriesCheckResult {
 
    let mut ret_result = LifterEntriesCheckResult::default();

    let sex_err_result = check_sex_errors(liftermap, meetdata, lifterdata, meet_data_root.clone());
    ret_result.append(sex_err_result);

    let jp_name_result = check_japanese_names(liftermap, meetdata, meet_data_root.clone());
    ret_result.append(jp_name_result);

    let (bw_delta_usernames_result, bw_delta_exempt_usernames) = load_bw_delta_sanity_usernames(lifterdir);
    ret_result.append(bw_delta_usernames_result);

    let check_bw_delta_result = check_bw_delta(bw_delta_exempt_usernames, liftermap, meetdata, meet_data_root.clone());
    ret_result.append(check_bw_delta_result);

    ret_result
}


fn get_entry_meet_date(meetdata: &AllMeetData, entry: &Entry) -> Date {

    // should be safe to panic if we don't have an EntryIndex at this point
    meetdata.get_meet(entry.index.unwrap()).date
}

//check a bodyweight/time delta to see if it's sane.  Return a Result with how far from the limit
//it was
fn check_individual_bw_delta(first_entry: &Entry, second_entry: &Entry, delta_days: i32) -> Result<f32, f32> {

    let first_bw: f32 = f32::from(first_entry.bodyweightkg);
    let second_bw: f32 = f32::from(second_entry.bodyweightkg);
    let bw_delta: f32 = f32::abs(first_bw - second_bw);

    let bw_pct_delta: f32 = (bw_delta / first_bw) * 100.0;
    let bw_pct_delta_per_day: f32 = bw_pct_delta / delta_days as f32;

    // 25% represents a lifter in old weight classes in a meet we only have weight classes for and
    // no actual weigh in data, competing in 125kg one day and 100kg the next or vice versa
    // (potentially 99kg and 101kg).  This drops off over a few days through the range where
    // lifters could cut for one meet and compete again soon without cutting, and stabilises
    // into a near-constant rate of sustainable weight loss
    let max_bw_pct_delta_per_day: f32 = (25.0 / delta_days as f32) + 0.15;

    if bw_pct_delta_per_day > max_bw_pct_delta_per_day {
        Err(bw_pct_delta_per_day - max_bw_pct_delta_per_day)
    } else {
        Ok(max_bw_pct_delta_per_day - bw_pct_delta_per_day)
    }
}


// check that changes in lifter's bodyweight between meets are sane
// we can take usernames, they aren't needed after this
fn check_bw_delta(exempt_usernames: HashSet<String>, liftermap: &LifterMap, meetdata: &AllMeetData, meet_data_root: PathBuf) -> LifterEntriesCheckResult {
    
    let mut result = LifterEntriesCheckResult::default();
    let mut lifter_worst_err_map: HashMap<&String, (WeightKg, WeightKg, i32, f32)> = HashMap::new();

    // limit the loop that generates Reports for the top n errors
    let top_n_err_limit: usize = 10;

    let sort_entries_by_date_closure = |a: &Entry, b: &Entry| -> Ordering {
        let ad: Date = get_entry_meet_date(meetdata, a);
        let bd: Date = get_entry_meet_date(meetdata, b);

        ad.cmp(&bd)
    };

    for lifter_indices in liftermap.values() {

        let name = &meetdata.get_entry(lifter_indices[0]).name;
        let mut prev: Option<&Entry> = None;

        //skip lifters for whom we can't make a username for some reason
        let username: String = match make_username(&name) {
            Ok(u) => u,
            Err(e) => {
                let mut report = Report::new(meet_data_root.clone());
                report.error(e);
                result.reports.push(report);
                continue;
            }
        };

        // skip I. Nitial and Onename, also skip any names
        // marked for exemption
        if name.contains(' ')
            && name.chars().skip(1).take(1).collect::<Vec<char>>() != ['.']
            && !exempt_usernames.contains(&username)
        {

            // sort the lifter's entries by meet date
            let mut lifter_entries_by_date: Vec<&Entry> = Vec::new();

            for index in lifter_indices.iter() {
                    lifter_entries_by_date.push(meetdata.get_entry(*index));
            }

            lifter_entries_by_date.sort_unstable_by(|a, b| sort_entries_by_date_closure(a, b));

            // now iterate over the lifter's entries by date and check sanity of bodyweight
            // changes over time
            for entry in &lifter_entries_by_date {

                //Ignore entries with no bodyweight / blank bodyweight
                if entry.bodyweightkg.is_zero() {
                    continue;
                }

                // if we have a previous entry for this lifter, compare it to the current one
                if let Some(prev_entry) = prev {

                    let prev_entry_date: Date = get_entry_meet_date(meetdata, prev_entry);
                    let entry_date: Date = get_entry_meet_date(meetdata, entry);

                    let delta_days: i32 = i32::abs(entry_date - prev_entry_date);

                    // ignore if entries are from the same day or from our catchall date
                    if delta_days > 0 {
                        // is this insane?  If so, by how far?
                        match check_individual_bw_delta(prev_entry, entry, delta_days) {
                            Ok(_err) => (),
                            Err(err) => {

                                // if it's the lifter's worst so far, track it
                                match lifter_worst_err_map.get(name) {
                                    Some(worst) => {
                                        let (_prev_bw, _bw, _delta_days, worst_err) = *worst;
                                        if err > worst_err {
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
                prev = Some(entry);
            }
        }
    }

    //sort the HashMap of lifter -> worst bw delta error
    //take the top n (define this up top of func) and report them
    let sort_worst_bw_delta_closure = |
        a: &(&String, WeightKg, WeightKg, i32, f32),
        b: &(&String, WeightKg, WeightKg, i32, f32)| -> Ordering {

        let (_a_name, _a_from_bw, _a_to_bw, _a_delta_days, a_err) = a;
        let (_b_name, _b_from_bw, _b_to_bw, _b_delta_days, b_err) = b;

        //ok to panic if this is None
        b_err.partial_cmp(&a_err).unwrap() 
        
    };       

    let mut sorted_lifter_worst_bw_delta: Vec<(&String, WeightKg, WeightKg, i32, f32)> = Vec::new();

    for (name, tup) in lifter_worst_err_map.iter() {
        let (from_bw, to_bw, delta_days, err) = *tup;
        sorted_lifter_worst_bw_delta.push((*name, from_bw, to_bw, delta_days, err));
    }  

    sorted_lifter_worst_bw_delta.sort_unstable_by(|a, b| sort_worst_bw_delta_closure(a, b));

    //.enumerate() isn't implemented for our tuple
    let mut i = 0;
    for sorted_tup in sorted_lifter_worst_bw_delta {
        if i >= top_n_err_limit {
            break;
        } else {
            let (name, from_bw, to_bw, delta_days, _err) = sorted_tup;
            let mut report = Report::new(meet_data_root.clone());
            let msg = format!("Anomalous bodyweight change for '{}' - from {} to {} in {} days", 
                name, from_bw, to_bw, delta_days
            );
            report.warning(msg);
            result.reports.push(report);
        }
        i += 1;
    }         

    result

}


fn load_bw_delta_sanity_usernames(lifterdir: &Path) -> (LifterEntriesCheckResult, HashSet<String>) {

    let mut exempt_usernames: HashSet<String> = HashSet::new();
    let mut report = Report::new(lifterdir.join("bw-exemptions.csv"));
    let mut result: LifterEntriesCheckResult = LifterEntriesCheckResult::default();


    // We don't use the '?' suffix because we don't want to return a straight Result
    // read the exemption usernames
    let rdr_result = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path);
 

    let mut rdr = match rdr_result {
        Ok(rb) => {
            rb
        }
        Err(_err) => {
            let msg = "Unable to create CSV reader";
            report.error(msg);
            result.reports.push(report);
            return (result, exempt_usernames)
        }
    };

    for (rownum, row_result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: BwExemptionRow = match row_result {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };
        
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

    result.reports.push(report);

    (result, exempt_usernames)
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


