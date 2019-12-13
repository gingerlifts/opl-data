//! Implements the /api/search endpoints.

use usernames::contains_writing_system;
use usernames::make_username;
use usernames::WritingSystem;

use crate::opldb::{algorithms, OplDb};
use crate::pages::selection::Selection;

/// JSON return from the /api/search/rankings/ endpoint.
#[derive(Serialize)]
pub struct SearchRankingsResult {
    /// The next index of a search result to which the viewport should update.
    pub next_index: Option<usize>,
}

pub fn search_rankings<'db>(
    opldb: &'db OplDb,
    selection: &Selection,
    start_row: usize, // Inclusive.
    query: &str,
) -> SearchRankingsResult {
    // Hacky solution to "#" ,"'"" & "." being replaced by underscores in the query
    // string
    let query_no_us = query.replace("_", "");

    // Convert the query string to a normalized form.
    // This tries to make it look like a username, since we're
    // just doing comparisons on the username.
    let normalized: String = match make_username(&query_no_us) {
        Ok(s) => s,
        Err(_) => String::new(),
    };

    let backwards: String = query_no_us
        .to_ascii_lowercase()
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join("");

    let backwards_with_space: String = query_no_us
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ");

    // Disallow bogus searches.
    if normalized.is_empty()
        && contains_writing_system(&normalized) == WritingSystem::Latin
    {
        return SearchRankingsResult { next_index: None };
    }

    // TODO: Use a better algorithm, don't generate everything.
    let list = algorithms::get_full_sorted_uniqued(selection, opldb);

    // Handle out-of-bounds requests.
    if start_row >= list.0.len() {
        return SearchRankingsResult { next_index: None };
    }

    // TODO: Use a better algorithm; this is really a MVP.
    for i in start_row..list.0.len() {
        let entry = opldb.get_entry(list.0[i]);
        let lifter = opldb.get_lifter(entry.lifter_id);

        if !normalized.is_empty()
            && (lifter.username.contains(&normalized)
                || lifter.username.contains(&backwards)
                || lifter
                    .instagram
                    .as_ref()
                    .map_or(false, |ig| ig.contains(&normalized)))
        {
            return SearchRankingsResult {
                next_index: Some(i),
            };
        } else if let Some(cyr_name) = &lifter.cyrillic_name {
            if cyr_name.contains(&query_no_us) || cyr_name.contains(&backwards_with_space)
            {
                return SearchRankingsResult {
                    next_index: Some(i),
                };
            }
        } else if let Some(jp_name) = &lifter.japanese_name {
            if jp_name.contains(&query_no_us) || jp_name.contains(&backwards_with_space) {
                return SearchRankingsResult {
                    next_index: Some(i),
                };
            }
        } else if let Some(el_name) = &lifter.greek_name {
            if el_name.contains(&query_no_us) || el_name.contains(&backwards_with_space) {
                return SearchRankingsResult {
                    next_index: Some(i),
                };
            }
        }
    }

    SearchRankingsResult { next_index: None }
}
