use crate::ir::span_ir::SourceSpan;

pub struct Diagnostic {
    pub severity: Severity,
    pub span: Option<SourceSpan>,
    pub kind: DiagnosticKind,
    pub message: String,
}

pub enum Severity {
    WARNING,
    ERROR,
}

pub enum DiagnosticKind {
    TypeError,
    ShapeError,
    MissingBindingValue,
    InvalidBindingIR,
}

// impl Diagnostic {
//     pub fn report(&self) -> &str {
        
//     }
// }