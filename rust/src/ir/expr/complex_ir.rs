use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct ComplexIR {
    pub real: f64,
    pub imag: f64,
    pub span: Option<SourceSpan>,
}
