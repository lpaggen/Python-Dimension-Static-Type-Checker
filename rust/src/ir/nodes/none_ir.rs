use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct NoneIR {
    pub span: Option<SourceSpan>,
}
