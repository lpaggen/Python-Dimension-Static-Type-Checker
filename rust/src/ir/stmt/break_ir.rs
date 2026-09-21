use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct BreakIR {
    pub span: SourceSpan,
}
