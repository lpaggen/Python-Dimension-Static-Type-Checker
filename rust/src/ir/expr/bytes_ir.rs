use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct BytesIR {
    pub value: Vec<u8>,
    pub span: Option<SourceSpan>,
}
