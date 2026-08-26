#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub file: String,
    pub lineno: i64,
    pub col_offset: i64,
    pub end_lineno: Option<i64>,
    pub end_col_offset: Option<i64>,
}
