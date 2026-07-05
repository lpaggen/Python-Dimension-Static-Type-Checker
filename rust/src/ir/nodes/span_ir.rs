#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub line: i64,
    pub col: i64,
    pub end_line: i64,
    pub end_col: i64,
}

impl SourceSpan {
    pub fn new(line: i64, col: i64, end_line: i64, end_col: i64) -> Self {
        Self {
            line,
            col,
            end_line,
            end_col,
        }
    }
}
