//! Checks for consistency errors across entries per lifter.

use std::cmp::Ordering;

use crate::{AllMeetData, Entry, EntryIndex, LifterMap, LifterDataMap, Report};

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
fn check_disambig_consistency_one(username: &str, usernames_sorted: &Vec<&str>, username_i: usize, disambig_count: usize, report: &mut Report) {

    if disambig_count > 1 {

        let mut drift: usize = 0;

        // from username_i the next disambig_count usernames should be "usernameN"
        for disambig_i in username_i..username_i + disambig_count as usize {

            let match_username = usernames_sorted[disambig_i];
            let match_variant_res = match_username.split_at(username.len()).1.parse::<usize>();

            if !match_username.starts_with(username) {
                //TODO - fill this in with more detail, use drift to work out how much fewer
                let msg = format!("{} should be disambiguated into {} variants, but there are fewer - already hit username {}", username, disambig_count, match_username);
                report.error(msg);
                break;
            }
            match match_variant_res {
                Ok(match_variant_n) => {
                    if match_variant_n > disambig_count {
                        let msg = format!("{} should be disambiguated into {} variants, but variant #{} exists", username, disambig_count, match_variant_n);
                        report.error(msg);
                    }
                    else {
                        // if the variant is higher than the loop index, we skipped >= 1.
                        // track how far we've drifted so that we don't keep flagging unnecessarily
                        // eg: for 1,2,4,5, only flag 3
                        if match_variant_n > (disambig_i + drift) {
                            let msg = format!("{} should be disambiguated into {} variants, but variant #{} appears to be missing", username, disambig_count, (disambig_i + drift));
                            report.error(msg);
                            drift += (match_variant_n - disambig_i);
                        }
                    }
                }
                Err(_) => {
                    //TODO - fill this in with more detail, use drift to figure out how many fewer
                    let msg = format!("{} should be disambiguated into {} variants, but there are fewer", username, disambig_count);
                    report.error(msg);
                    break;
                }
            }
        }
    }

}

/// Checks Name consistency for all lifters.
pub fn check_name_all(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    lifterdatamap: &LifterDataMap,
    reports: &mut Vec<Report>,
) {
    let mut report = Report::new("[Name Consistency]".into());

    //TODO - get usernames from entry indices instead
    let mut usernames_sorted: Vec<&str> = lifterdatamap.keys().map(|u| u.as_str()).collect();  

    //DEBUG
    println!("{} keys in lifterdatamap", usernames_sorted.len());


    let alpha_num_sort_closure = |a: &str, b: &str| -> Ordering {
        
        // we need these as bytes to index, since usernames are ASCII this
        // is ok
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut a_num: u32 = 0;
        let mut b_num: u32 = 0;
        let mut a_found_num: bool = false;
        let mut b_found_num: bool = false;
        let mut a_alpha: &str = "";
        let mut b_alpha: &str = "";

        //work backward through the slice to extract any numerical portion
        //and therefore also the alphabetical portion
        for a_i in (0..a.len()).rev() {
            if !char::from(a_bytes[a_i]).is_numeric() {
                if a_i < a.len() - 1 {
                    let (a_alpha_part, a_num_part) = a.split_at(a_i + 1);
                    a_found_num = true;
                    a_num = a_num_part.parse::<u32>().unwrap();
                    a_alpha = a_alpha_part;
                }
                break;
            }
        }

        if !a_found_num {
            a_alpha = a;
        }

        for b_i in (0..b.len()).rev() {
            if !char::from(b_bytes[b_i]).is_numeric() {
                if b_i < b.len() - 1 {
                    let (b_alpha_part, b_num_part) = b.split_at(b_i + 1);
                    b_found_num = true;
                    b_num = b_num_part.parse::<u32>().unwrap();
                    b_alpha = b_alpha_part;
                }
                break;
            }
        }

        if !b_found_num {
            b_alpha = b;
        }


        let alpha_cmp: Ordering = a_alpha.cmp(&b_alpha);
        
        // if the alphabetical portions are equal, compare the numerical
        // values of the numerical portions, this way joebloggs11 > joebloggs2
        if alpha_cmp == Ordering::Equal {
            return a_num.cmp(&b_num);
        }

        alpha_cmp
    };
                
    usernames_sorted.sort_unstable_by(|a, b| alpha_num_sort_closure(a, b));

    for (username_i, sorted_username) in usernames_sorted.iter().enumerate() {
        if let Some(lifterdata) = lifterdatamap.get(*sorted_username) {
            if lifterdata.disambiguation_count > 1 {
                check_disambig_consistency_one(sorted_username, &usernames_sorted, username_i, lifterdata.disambiguation_count as usize, &mut report);
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
