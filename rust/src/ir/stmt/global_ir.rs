use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct GlobalIR {
    pub names: Vec<String>,
    pub span: Option<SourceSpan>,
}
