//! Tests search functionality.

use opldb::query::direct::RankingsQuery;
use opldb::{algorithms, OplDb};
use search::*;

use std::sync::Once;

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = Once::new();

pub fn db() -> &'static OplDb {
    const LIFTERS_CSV: &str = "../../build/lifters.csv";
    const MEETS_CSV: &str = "../../build/meets.csv";
    const ENTRIES_CSV: &str = "../../build/entries.csv";

    unsafe {
        OPLDB_INIT.call_once(|| {
            OPLDB_GLOBAL = Some(OplDb::from_csv(LIFTERS_CSV, MEETS_CSV, ENTRIES_CSV).unwrap());
        });

        OPLDB_GLOBAL.as_ref().unwrap()
    }
}

/// Checks that basic rankings search functionality works.
#[test]
fn basic_rankings_search() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings(&db, &rankings, 0, "Sean Stangl");
    let row = res.unwrap();

    // Check that the result is for the specified lifter.
    let list = algorithms::get_full_sorted_uniqued(&rankings, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}

/// Checks that searching in "Lastname Firstname" order works.
#[test]
fn backwards_name_search() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings(&db, &rankings, 0, "stangl sean");
    let row = res.unwrap();

    // Check that the result is for the specified lifter.
    let list = algorithms::get_full_sorted_uniqued(&rankings, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}

// Checks that searching by Instagram works.
#[test]
fn instagram_search() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings(&db, &rankings, 0, "Ferruix");
    let row = res.unwrap();

    // Check that the result is for the specified lifter.
    let list = algorithms::get_full_sorted_uniqued(&rankings, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}

/// Checks that basic searching in Cyrillic works.
#[test]
fn cyrillic_search() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings(&db, &rankings, 0, "Шон Стангл");
    let row = res.unwrap();

    // Check that the result is for the specified lifter.
    let list = algorithms::get_full_sorted_uniqued(&rankings, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}

#[test]
fn basic_rankings_search_tantivy() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings_tantivy(&db, &rankings, 0, "Sean Stangl", 10);
    let lifter_ids = res.unwrap();

    assert!(lifter_ids.len() > 0);
    // Check that the result is for the specified lifter.
    let lifter = db.get_lifter(*lifter_ids.get(0).unwrap());
    assert_eq!(lifter.name, "Sean Stangl");
}

#[test]
fn backwards_rankings_search_tantivy() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings_tantivy(&db, &rankings, 0, "Stangl Sean", 10);
    let lifter_ids = res.unwrap();

    assert!(lifter_ids.len() > 0);
    // Check that the result is for the specified lifter.
    let lifter = db.get_lifter(*lifter_ids.get(0).unwrap());
    assert_eq!(lifter.name, "Sean Stangl");
}

#[test]
fn cyrillic_rankings_search_tantivy() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings_tantivy(&db, &rankings, 0, "Шон Стангл", 10);
    let lifter_ids = res.unwrap();

    assert!(lifter_ids.len() > 0);
    // Check that the result is for the specified lifter.
    let lifter = db.get_lifter(*lifter_ids.get(0).unwrap());
    assert_eq!(lifter.name, "Sean Stangl");
}

#[test]
fn instagram_rankings_search_tantivy() {
    let db = db();
    let rankings = RankingsQuery::default();

    // Perform the search.
    let res = search_rankings_tantivy(&db, &rankings, 0, "ferruix", 10);
    let lifter_ids = res.unwrap();

    assert!(lifter_ids.len() > 0);
    // Check that the result is for the specified lifter.
    let lifter = db.get_lifter(*lifter_ids.get(0).unwrap());
    assert_eq!(lifter.name, "Sean Stangl");
}
