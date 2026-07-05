use crate::ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct SymbolIR {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub scope_id: i64,
    pub span: Option<SourceSpan>,
}
