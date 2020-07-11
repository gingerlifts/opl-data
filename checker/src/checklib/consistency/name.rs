//! Checks for consistency errors across entries per lifter.

use crate::{AllMeetData, Entry, EntryIndex, LifterMap, Report};

/// Checks that Name fields are consistent for this lifter.
fn check_name_one(indices: &[EntryIndex], meetdata: &AllMeetData, report: &mut Report) {
    let first_entry: &Entry = meetdata.get_entry(indices[0]);

    let name = &first_entry.name;
    let mut cyrillicname = &first_entry.cyrillicname;
    let mut greekname = &first_entry.greekname;
    let mut japanesename = &first_entry.japanesename;
    let mut koreanname = &first_entry.koreanname;

    for index in indices.iter().skip(1) {
        let entry = &meetdata.get_entry(*index);

        // The Name field must exactly match for the same username.
        if name != &entry.name {
            let msg = format!(
                "Name conflict for '{}': '{}' vs '{}'",
                entry.username, name, entry.name
            );
            report.error(msg);
        }

        // If this is the first time seeing an optional name field, remember it.
        if cyrillicname.is_none() && entry.cyrillicname.is_some() {
            cyrillicname = &entry.cyrillicname;
        }
        if greekname.is_none() && entry.greekname.is_some() {
            greekname = &entry.greekname;
        }
        if japanesename.is_none() && entry.japanesename.is_some() {
            japanesename = &entry.japanesename;
        }
        if koreanname.is_none() && entry.koreanname.is_some() {
            koreanname = &entry.koreanname;
        }

        // Check CyrillicName consistency.
        if let Some(entry_cr_name) = &entry.cyrillicname {
            if let Some(cr_name) = cyrillicname {
                if cr_name != entry_cr_name {
                    let msg = format!(
                        "CyrillicName conflict for {}: '{}' vs '{}'",
                        entry.username, cr_name, entry_cr_name
                    );
                    report.error(msg);
                }
            }
        }

        // Check GreekName consistency.
        if let Some(entry_el_name) = &entry.greekname {
            if let Some(el_name) = greekname {
                if el_name != entry_el_name {
                    let msg = format!(
                        "GreekName conflict for {}: '{}' vs '{}'",
                        entry.username, el_name, entry_el_name
                    );
                    report.error(msg);
                }
            }
        }

        // Check JapaneseName consistency.
        if let Some(entry_jp_name) = &entry.japanesename {
            if let Some(jp_name) = japanesename {
                if jp_name != entry_jp_name {
                    let msg = format!(
                        "JapaneseName conflict for {}: '{}' vs '{}'",
                        entry.username, jp_name, entry_jp_name
                    );
                    report.error(msg);
                }
            }
        }

        // Check KoreanName consistency.
        if let Some(entry_ko_name) = &entry.koreanname {
            if let Some(ko_name) = koreanname {
                if ko_name != entry_ko_name {
                    let msg = format!(
                        "KoreanName conflict for {}: '{}' vs '{}'",
                        entry.username, ko_name, entry_ko_name
                    );
                    report.error(msg);
                }
            }
        }
    }
}

/// Check name disambig consistency for one lifter 
fn check_disambig_consistency_one(username: &String, usernames_sorted: &Vec<String>, username_i: u32, disambig_count: u32, report: &mut Report) {

    if disambig_count > 1 {
        // from username_i the next disambig_count usernames should be "usernameN"
        for disambig_i in username_i..username_i + disambig_count {
            //TODO
        }
    }

}

/// Checks Name consistency for all lifters.
/// TODO - pass &lifterdatamap to check_name_all()
pub fn check_name_all(
    liftermap: &LifterMap,
    lifterdatamap: &LifterDataMap,
    meetdata: &AllMeetData,
    reports: &mut Vec<Report>,
) {
    let mut report = Report::new("[Name Consistency]".into());

    let mut usernames_sorted: Vec<String> = liftermap.keys().collect();
    usernames_sorted.sort_unstable_by_key(|k| k);

    for (username_i, sorted_username) in usernames_sorted.enumerate() {
        if let Some(lifterdata) = lifterdatamap.get(&sorted_username) {
            if lifterdata.disambiguation_count > 1 {
                check_disambig_consistency_one(&sorted_username, &usernames_sorted, username_i, disambig_count, &mut report);
            }
        };
    }


    for lifter_indices in liftermap.values() {
        check_name_one(&lifter_indices, meetdata, &mut report);
    }

    if report.has_messages() {
        reports.push(report);
    }
}
