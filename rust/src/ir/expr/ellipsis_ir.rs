use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct EllipsisIR {
    pub span: Option<SourceSpan>,
}
