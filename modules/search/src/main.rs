//! CLI driver for developing the search interface.

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::path::PathBuf;

const READLINE_PROMPT: &str = ">>> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lifters_csv: PathBuf = PathBuf::from("../../build/lifters.csv");
    let meets_csv: PathBuf = PathBuf::from("../../build/meets.csv");
    let entries_csv: PathBuf = PathBuf::from("../../build/entries.csv");

    let _db = opldb::OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv)?;

    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline(READLINE_PROMPT) {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(line.as_str());
                    println!("{}", line);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    std::process::exit(0); // Dropping the database takes a while.
}
