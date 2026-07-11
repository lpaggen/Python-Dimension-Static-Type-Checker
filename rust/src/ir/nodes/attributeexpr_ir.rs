use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct AttributeExprIR {
    pub target: Box<ExprIR>,
    pub attr: String,
    pub span: Option<SourceSpan>,
}

impl AttributeExprIR {
    pub fn new(target: ExprIR, attr: impl Into<String>, span: Option<SourceSpan>) -> Self {
        Self {
            target: Box::new(target),
            attr: attr.into(),
            span,
        }
    }
}
