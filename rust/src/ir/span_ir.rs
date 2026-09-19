use std::fmt;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SourceSpan {
    pub file: String,
    pub lineno: i64,
    pub col_offset: i64,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}

impl fmt::Display for SourceSpan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Python AST columns are zero-based; source locations shown to users are
        // conventionally one-based.
        write!(formatter, "{}:{}:{}", self.file, self.lineno, self.col_offset + 1)?;

        match (self.end_lineno, self.end_col_offset) {
            (Some(end_line), Some(end_col)) if end_line == self.lineno => {
                // `end_col_offset` is exclusive, so its zero-based value is
                // already the inclusive column number a user expects.
                write!(formatter, "-{}", end_col)
            }
            (Some(end_line), Some(end_col)) => {
                write!(formatter, "-{}:{}", end_line, end_col)
            }
            _ => Ok(()),
        }
    }
}
