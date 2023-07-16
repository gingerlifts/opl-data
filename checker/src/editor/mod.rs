use std::cmp::max;
use std::fmt;

/// Identifies a given cell in a CSV.
pub struct CellIdentifier<'a> {
    column_name: &'a str,
    row_number: usize,
}

impl<'a> CellIdentifier<'a> {
    pub fn new(column_name: &'a str, row_number: usize) -> Self {
        Self {
            column_name,
            row_number,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateError {
    InvalidColumn,
    InvalidRow,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Self::InvalidColumn => "invalid column",
            Self::InvalidRow => "invalid row",
        };

        write!(f, "{variant}")
    }
}

impl std::error::Error for UpdateError {}

/// Provides functionality to edit a CSV without parsing it strictly.
pub struct Editor {
    header: String,
    rows: Vec<String>,
}

impl Editor {
    pub fn new(inner: String) -> Self {
        let mut lines = inner.split('\n');
        let header = lines.next().unwrap().to_owned();
        let rows = lines.map(|line| line.to_owned()).collect();

        Self { header, rows }
    }

    pub fn update(&mut self, ident: CellIdentifier<'_>, value: &str) -> Result<(), UpdateError> {
        // Validate we have the header
        if !self.header.contains(ident.column_name) {
            return Err(UpdateError::InvalidColumn);
        }

        // Validate we have the row
        if self.rows.len() < ident.row_number {
            return Err(UpdateError::InvalidRow);
        }

        // Find where we need to update
        let header_index = self
            .header
            .split(',')
            .position(|h| h == ident.column_name)
            .expect("Failed to find column index");

        let row = &mut self.rows[ident.row_number];
        let mut comma_indices = row.match_indices(',').skip(max(0, header_index - 1));

        let (start, _) = comma_indices.next().unwrap();
        let end = comma_indices.next().map(|x| x.0).unwrap_or(row.len());

        row.replace_range(start + 1..end, value);

        Ok(())
    }

    pub fn finalize(self) -> String {
        let header = self.header;
        let rows = self.rows.join("\n");

        format!("{header}\n{rows}")
    }
}

#[cfg(test)]
mod tests;
