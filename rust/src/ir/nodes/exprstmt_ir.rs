use crate::ir::{ExprIR, SourceSpan};

#[derive(Debug, Clone)]
pub struct ExprStmtIR {
    pub value: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

impl ExprStmtIR {
    pub fn new(value: Option<ExprIR>, span: Option<SourceSpan>) -> Self {
        Self {
            value: value.map(Box::new),
            span,
        }
    }
}
