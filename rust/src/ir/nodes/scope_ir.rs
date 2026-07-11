use crate::{ir::span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct ScopeIR {
    pub id: i64,
    pub name: String,
    pub kind: ScopeKind,
    pub parent_id: Option<i64>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub enum ScopeKind {
    Unknown,
    Class,
    Function,
    Module,
    Block
}

impl From<i32> for ScopeKind {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Module,
            2 => Self::Function,
            3 => Self::Class,
            4 => Self::Block,
            _ => Self::Unknown,
        }
    }
}
