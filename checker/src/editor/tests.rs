use itertools::Itertools;

use crate::editor::{CellIdentifier, Editor, UpdateError};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn create_csv_file(header: &[&str], rows: &[&[&str]]) -> String {
    let header = header.join(",");
    let rows = rows.iter().map(|row| row.join(",")).join("\n");

    format!("{header}\n{rows}")
}

#[test]
fn cannot_edit_invalid_columns() -> TestResult {
    let csv = create_csv_file(&["Place", "Name"], &[&["1", "Alex"], &["2", "John"]]);
    let mut editor = Editor::new(csv);
    let ident = CellIdentifier::new("Something Else", 0);

    let result = editor.update(ident, "Mark");

    assert_eq!(result, Err(UpdateError::InvalidColumn));

    Ok(())
}

#[test]
fn cannot_edit_invalid_rows() -> TestResult {
    let csv = create_csv_file(&["Place", "Name"], &[&["1", "Alex"], &["2", "John"]]);
    let mut editor = Editor::new(csv);
    let ident = CellIdentifier::new("Place", 123);

    let result = editor.update(ident, "Mark");

    assert_eq!(result, Err(UpdateError::InvalidRow));

    Ok(())
}

#[test]
fn can_edit_basic_csv_files() -> TestResult {
    let csv = create_csv_file(&["Place", "Name"], &[&["1", "Alex"], &["2", "John"]]);
    let mut editor = Editor::new(csv);
    let ident = CellIdentifier::new("Name", 0);

    editor.update(ident, "Mark")?;

    let output = editor.finalize();
    let expected = create_csv_file(&["Place", "Name"], &[&["1", "Mark"], &["2", "John"]]);

    assert_eq!(output, expected);

    Ok(())
}
