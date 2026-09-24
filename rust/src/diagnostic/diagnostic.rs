use std::fmt;

use crate::ir::span_ir::SourceSpan;

#[derive(Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: SourceSpan,
    pub kind: DiagnosticKind,
    pub message: String,
}

#[derive(Debug)]
pub enum Severity {
    WARNING,
    ERROR,
}

#[derive(Debug)]
pub enum DiagnosticKind {
    TypeError,
    ShapeError,
    MissingBindingValue,
    InvalidBindingIR,
    IOError,
    MismatchedAnnotationType,
    UnknownAssignValue,
    UnknownBindingKind,
}

impl Diagnostic {
    pub fn new(severity: Severity, span: SourceSpan, kind: DiagnosticKind, message: &str) -> Self {
        Self {
            severity,
            span,
            kind,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {:?}[{:?}]: {}",
            self.span, self.severity, self.kind, self.message,
        )
    }
}
