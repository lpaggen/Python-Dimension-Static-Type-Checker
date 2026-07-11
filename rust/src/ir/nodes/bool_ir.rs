use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct BooleanIR {
    pub value: bool,
    pub span: Option<SourceSpan>,
}
