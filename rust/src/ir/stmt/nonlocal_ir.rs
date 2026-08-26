use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct NonlocalIR {
    pub names: Vec<String>,
    pub span: Option<SourceSpan>,
}
