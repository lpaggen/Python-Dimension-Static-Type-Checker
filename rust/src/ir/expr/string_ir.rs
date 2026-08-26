use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct StringIR {
    pub value: String,
    pub span: Option<SourceSpan>,
}
