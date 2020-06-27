use std::path::PathBuf;

use crate::{
    LifterMap, AllMeetData, LifterDataMap, Report
};

pub struct LifterEntriesCheckResult {
    pub reports: Vec<Report>
}


// check for consistency of each lifter's indicated sex in their entries
// inconsistencies are flagged as errors unless the lifter has an entry in the 
// sex exemptions file
pub fn check_sex_errors(liftermap: &LifterMap, meetdata: &AllMeetData, lifterdata: &LifterDataMap, meet_data_root: PathBuf) -> LifterEntriesCheckResult {

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
pub fn check_japanese_names(liftermap: &LifterMap, meetdata: &AllMeetData, meet_data_root: PathBuf) -> LifterEntriesCheckResult {

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


