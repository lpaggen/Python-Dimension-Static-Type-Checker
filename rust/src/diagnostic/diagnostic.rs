use crate::ir::span_ir::SourceSpan;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: Option<SourceSpan>,
    pub kind: DiagnosticKind,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    WARNING,
    ERROR,
}

#[derive(Debug, Serialize)]
pub enum DiagnosticKind {
    TypeError,
    ShapeError,
    MissingBindingValue,
    InvalidBindingIR,
    IOError,
    MismatchedAnnotationType,
    UnknownAssignValue,
    UnknownBindingKind,
    UnsupportedFeature,
}

impl Diagnostic {
    pub fn new(severity: Severity, span: SourceSpan, kind: DiagnosticKind, message: &str) -> Self {
        Self {
            severity,
            span: Some(span),
            kind,
            message: message.to_string(),
        }
    }
}

// impl Diagnostic {
//     pub fn report(&self) -> &str {

//     }
// }
