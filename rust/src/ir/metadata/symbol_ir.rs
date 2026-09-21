use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct SymbolIR {
    pub id: usize,
    pub name: String,
    pub kind: SymbolKind,
    pub scope_id: usize,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Unknown,
    Variable,
    Function,
    Class,
    Param,
    ModuleAlias,
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
