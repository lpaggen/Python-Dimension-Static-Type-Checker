use crate::ir::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatIR {
    pub value: f64,
    pub span: Option<SourceSpan>,
}
