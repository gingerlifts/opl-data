//! Search engine for all of powerlifting.

#[macro_use]
extern crate tantivy;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;

use opldb::query::direct::RankingsQuery;
use opldb::{algorithms, Entry, OplDb};
use opltypes::*;

/// Searches the given rankings by lifter information.
///
/// Returns the index of the first match from `start_row`, or `None`.
pub fn search_rankings(
    db: &OplDb,
    rankings: &RankingsQuery,
    start_row: usize, // Inclusive.
    query: &str,
) -> Option<usize> {
    // FIXME: Hacky solution to "#" ,"'"" & "." being replaced by underscores
    // in the query string. The client code makes that replacement in order
    // to ensure that the URL is valid, since this is accessed via a GET parameter.
    // We could do something craftier, like base-64 encode it.
    let query = query.replace("_", "");

    let system = infer_writing_system(&query);

    // Convert the query string to a normalized form.
    // This tries to make it look like a username, since we're
    // just doing comparisons on the username.
    let normalized_latin: String = match Username::from_name(&query) {
        Ok(u) => u.into(),
        Err(_) => String::new(),
    };

    // Disallow bogus queries.
    if normalized_latin.is_empty() && system == WritingSystem::Latin {
        return None;
    }

    let backwards: String = query
        .to_ascii_lowercase()
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join("");

    let backwards_with_space: String = query
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ");

    // TODO: Use a better algorithm, don't generate everything.
    let list = algorithms::get_full_sorted_uniqued(rankings, db);

    // Handle out-of-bounds requests.
    if start_row >= list.0.len() {
        return None;
    }

    // TODO: Use a better algorithm; this is really a MVP.
    for i in start_row..list.0.len() {
        let entry = db.get_entry(list.0[i]);
        let lifter = db.get_lifter(entry.lifter_id);

        // First, check if there's a match based on the username or IG.
        if !normalized_latin.is_empty()
            && (lifter.username.as_str().contains(&normalized_latin)
                || lifter.username.as_str().contains(&backwards)
                || lifter
                    .instagram
                    .as_ref()
                    .map_or(false, |ig| ig.contains(&normalized_latin)))
        {
            return Some(i);
        }

        // Otherwise, check based on the writing system.
        let localized_name: Option<&String> = match system {
            WritingSystem::Cyrillic => lifter.cyrillic_name.as_ref(),
            WritingSystem::Greek => lifter.greek_name.as_ref(),
            WritingSystem::Japanese => lifter.japanese_name.as_ref(),
            WritingSystem::Korean => lifter.korean_name.as_ref(),
            WritingSystem::Latin => Some(&lifter.name),
        };

        if let Some(name) = localized_name {
            if name.contains(&query) || name.contains(&backwards_with_space) {
                return Some(i);
            }
        }
    }

    None
}

/// Searches the given rankings by lifter information using tantivy.
///
/// Returns the index of the first match from `start_row`, or `None`.
pub fn search_rankings_tantivy(
    db: &OplDb,
    rankings: &RankingsQuery,
    start_row: usize,
    query: &str,
    num_results: usize,
) -> Option<Vec<u32>> {
    let query = query.replace("_", "");
    let system = infer_writing_system(&query);

    let mut schema_builder = Schema::builder();

    // Define the schema.
    schema_builder.add_u64_field("id", STORED);
    schema_builder.add_text_field("name", TEXT);
    schema_builder.add_text_field("normalized_latin", STRING);
    schema_builder.add_text_field("instagram", STRING);
    schema_builder.add_text_field("localized_name", TEXT);

    let list = algorithms::get_full_sorted_uniqued(rankings, db);

    // Build the schema and create in dir.
    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema.clone());

    // TODO(lukeyeh): Think of a better size for heap for the indexer and make
    // it a constant.
    let mut index_writer = match index.writer(50_000_000) {
        Ok(index_writer) => index_writer,
        Err(_) => return None,
    };

    // Get field for later use when executing search.
    let name_field = schema.get_field("name").unwrap();
    let normalized_latin_field = schema.get_field("normalized_latin").unwrap();
    let instagram_field = schema.get_field("instagram").unwrap();
    let localized_name_field = schema.get_field("localized_name").unwrap();

    // Sort entries by lifter_id. We do this such that the index is ordered in
    // ascending order by lifter_id. This allows us to use the DocAddress as a
    // placeholder for lifter_id.
    let mut entries = (start_row..list.0.len())
        .map(|i| db.get_entry(list.0[i]))
        .collect::<Vec<&Entry>>();
    entries.sort_by(|a, b| a.lifter_id.partial_cmp(&b.lifter_id).unwrap());

    // Create index with name, normalized_latin, instagram, and localized name.
    entries.iter().for_each(|entry| {
        let lifter = db.get_lifter(entry.lifter_id);
        let localized_name: Option<&String> = match system {
            WritingSystem::Cyrillic => lifter.cyrillic_name.as_ref(),
            WritingSystem::Greek => lifter.greek_name.as_ref(),
            WritingSystem::Japanese => lifter.japanese_name.as_ref(),
            WritingSystem::Korean => lifter.korean_name.as_ref(),
            WritingSystem::Latin => Some(&lifter.name),
        };
        index_writer.add_document(doc!(
            name_field => lifter.name.as_str(),
            normalized_latin_field => lifter.username.as_str(),
            instagram_field =>  match &lifter.instagram {
                Some(instagram_username) => instagram_username.as_str(),
                None => ""
            },
            localized_name_field => match localized_name {
                Some(name) => name,
                None => ""
            }
        ));
    });
    // Commit index.
    index_writer
        .commit()
        .expect("Failed to flush index to disk.");

    // Create reader.
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .expect("Failed to create reader.");

    // Create searcher that encompasses name, instagram, normalized latin, and
    // localized names.
    let searcher = reader.searcher();
    let query = QueryParser::for_index(
        &index,
        vec![
            name_field,
            instagram_field,
            normalized_latin_field,
            localized_name_field,
        ],
    )
    .parse_query(query.as_str())
    .expect("Failed to parse query.");

    // Execute search.
    Some(
        searcher
            .search(&query, &TopDocs::with_limit(num_results))
            .expect("Failed to retrieve top docs.")
            .into_iter()
            .map(|(_score, doc_address)| doc_address.doc())
            .collect::<Vec<u32>>(),
    )
}
