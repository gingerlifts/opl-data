//! Defines the interface for human-readable errors and warnings.
//!
//! Unlike most libraries that surface error messages, this one explicitly
//! does not intend to visually show the context, other than by reporting
//! the exact line number on which the error occurred. This is because:
//!
//! 1. Each line is logically self-consistent.
//! 2. There are usually an extreme number of errors, and vertical density
//!    helps scan through them all.
//! 3. The context itself is generally not helpful, because the data is CSV.

use std::fmt;
use std::path::PathBuf;

use crate::editor::{CellIdentifier, Editor, UpdateError};
use crate::report_count::ReportCount;

#[derive(Clone, Debug, Serialize)]
pub enum FixableError {
    NameConflict {
        username: String,
        expected: String,
        found: String,
    },
}

impl FixableError {
    fn fix(&self, line_number: usize, editor: &mut Editor) -> Result<(), UpdateError> {
        match self {
            Self::NameConflict { expected, .. } => {
                editor.update(CellIdentifier::new("Name", line_number), &expected)
            }
        }
    }
}

impl fmt::Display for FixableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameConflict {
                username,
                expected,
                found,
            } => {
                write!(f, "Conflict for '{username}': '{expected}' vs '{found}'")
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FixableErrorDetails {
    /// The line the error was found on.
    line_number: usize,
    /// The details of the error.
    inner: FixableError,
}

impl fmt::Display for FixableErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl FixableErrorDetails {
    pub fn fix(&self, editor: &mut Editor) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.fix(self.line_number, editor)?;

        Ok(())
    }
}

/// A data error or warning message that should be reported.
#[derive(Debug, Serialize)]
pub enum Message {
    Error(String),
    FixableError(FixableErrorDetails),
    Warning(String),
}

/// Accumulates messages that should be reported as a single batch.
#[derive(Debug, Serialize)]
pub struct Report {
    /// Each report represents errors/warnings from a single file. This is its path.
    pub path: PathBuf,
    /// Any errors or warnings generated while reading that file.
    pub messages: Vec<Message>,
}

impl Report {
    /// Creates a new `Report` about the file at `path`.
    pub fn new(path: PathBuf) -> Self {
        Report {
            path,
            messages: Vec::new(),
        }
    }

    /// Reports an error, which causes checks to fail.
    pub fn error(&mut self, message: impl ToString) {
        self.messages.push(Message::Error(message.to_string()));
    }

    pub fn fixable_error(&mut self, line_number: usize, inner: FixableError) {
        let message = Message::FixableError(FixableErrorDetails { line_number, inner });

        self.messages.push(message);
    }

    /// Reports an error on a specific line.
    pub fn error_on(&mut self, line: u64, message: impl ToString) {
        let msg = format!(" Line {line}: {}", message.to_string());
        self.messages.push(Message::Error(msg));
    }

    /// Reports a warning, which allows checks to pass with a note.
    pub fn warning(&mut self, message: impl ToString) {
        self.messages.push(Message::Warning(message.to_string()));
    }

    /// Reports a warning on a specific line.
    pub fn warning_on(&mut self, line: u64, message: impl ToString) {
        let msg = format!(" Line {line}: {}", message.to_string());
        self.messages.push(Message::Warning(msg));
    }

    /// Whether a report has any messages.
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Returns how many messages there are of (errors, warnings).
    pub fn count_messages(&self) -> ReportCount {
        let mut errors = 0;
        let mut fixable_errors = 0;
        let mut warnings = 0;

        for message in &self.messages {
            match message {
                Message::Error(_) => errors += 1,
                Message::FixableError { .. } => fixable_errors += 1,
                Message::Warning(_) => warnings += 1,
            }
        }

        ReportCount::new(errors, fixable_errors, warnings)
    }

    /// Returns the name of the parent folder of the given file.
    pub fn parent_folder(&self) -> Result<&str, &str> {
        self.path
            .as_path()
            .parent()
            .and_then(|p| p.file_name().and_then(std::ffi::OsStr::to_str))
            .ok_or("Insufficient parent directories")
    }
}
