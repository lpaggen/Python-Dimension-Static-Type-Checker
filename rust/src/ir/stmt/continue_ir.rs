use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct ContinueIR {
    pub span: Option<SourceSpan>,
}
