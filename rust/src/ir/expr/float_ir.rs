use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct FloatIR {
    pub value: f64,
    pub span: Option<SourceSpan>,
}
