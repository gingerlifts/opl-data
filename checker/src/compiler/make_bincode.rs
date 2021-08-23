//! Loads the database from CSV files, then serializes its in-memory representation using bincode.

use bincode::Options;
use opldb::OplDb;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Serializes the entire in-memory database to a bincode file.
pub fn make_bincode(buildpath: &Path) -> Result<(), String> {
    let entries_path = buildpath.join("entries.csv");
    let lifters_path = buildpath.join("lifters.csv");
    let meets_path = buildpath.join("meets.csv");

    if !entries_path.exists() || !lifters_path.exists() || !meets_path.exists() {
        return Err("make_bincode() requires running make_csv() first".into());
    }

    // Use the CSV files to load the full database into memory.
    let db =
        OplDb::from_csv(&lifters_path, &meets_path, &entries_path).map_err(|e| e.to_string())?;

    // Now serialize the entire in-memory database to a file.
    let bincode_path = buildpath.join("opldb.bc");
    let bincode_file = File::create(&bincode_path).map_err(|e| e.to_string())?;
    let buf = BufWriter::new(bincode_file);

    bincode::DefaultOptions::new()
        .serialize_into(buf, &db)
        .map_err(|e| e.to_string())?;

    Ok(())
}
