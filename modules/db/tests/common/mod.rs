//! Common code for database integration tests.

use opldb::OplDb;

use std::path::PathBuf;
use std::sync::Once;

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = Once::new();

pub fn db() -> &'static OplDb {
    let lifters_csv = PathBuf::from("../../build/lifters.csv");
    let meets_csv = PathBuf::from("../../build/meets.csv");
    let entries_csv = PathBuf::from("../../build/entries.csv");

    unsafe {
        OPLDB_INIT.call_once(|| {
            OPLDB_GLOBAL = Some(OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv).unwrap());
        });

        OPLDB_GLOBAL.as_ref().unwrap()
    }
}
