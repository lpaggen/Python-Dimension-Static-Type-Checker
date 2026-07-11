use crate::ir::{span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct SymbolIR {
    pub id: i64,
    pub name: String,
    pub kind: SymbolKind,
    pub scope_id: i64,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Unknown,
    Variable,
    Function,
    Class,
    Param,
    ModuleAlias
}

impl From<i32> for SymbolKind {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Variable,
            2 => Self::Function,
            3 => Self::Class,
            4 => Self::Param,
            5 => Self::ModuleAlias,
            _ => Self::Unknown,
        }
    }
}
