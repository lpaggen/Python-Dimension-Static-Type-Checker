use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct IntegerIR {
    pub value: i64,
    pub span: Option<SourceSpan>,
}
