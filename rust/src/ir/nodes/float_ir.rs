use crate::ir::nodes::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatIR {
    pub value: f64,
    pub span: Option<SourceSpan>,
}
