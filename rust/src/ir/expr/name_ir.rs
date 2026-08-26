use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct NameIR {
    pub id: String,
    pub use_scope_id: i64,
    pub span: Option<SourceSpan>,
}
