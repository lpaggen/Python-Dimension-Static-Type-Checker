use crate::ir::span_ir::SourceSpan;

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
    pub id: usize,
    pub local_symbol_id: usize,
    pub scope_id: usize,
    pub kind: ImportKind,
    pub module_name: String,
    pub imported_name: Option<String>,
    pub alias: Option<String>,
    pub relative_level: usize,
    pub span: SourceSpan, // ? needed ?
}
