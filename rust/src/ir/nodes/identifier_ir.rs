use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct IdentifierIR {
    pub name: String,
    pub use_scope_id: i64,
    pub span: Option<SourceSpan>,
}
