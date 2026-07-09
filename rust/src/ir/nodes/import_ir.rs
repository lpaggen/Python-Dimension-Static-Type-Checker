use crate::ir::nodes::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportKind {
    Import,
    FromImport,
    Unknown(i32),
}

impl From<i32> for ImportKind {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Import,
            2 => Self::FromImport,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportIR {
    pub id: i64,
    pub local_symbol_id: i64,
    pub scope_id: i64,
    pub kind: ImportKind,
    pub module_name: String,
    pub imported_name: Option<String>,
    pub alias: Option<String>,
    pub relative_level: i64,
    pub span: Option<SourceSpan>,
}
