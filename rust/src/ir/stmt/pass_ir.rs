use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct PassIR {
    pub span: Option<SourceSpan>,
}
