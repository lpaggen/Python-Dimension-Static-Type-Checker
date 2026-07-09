use crate::ir::nodes::{ExprIR, SourceSpan};

#[derive(Debug, Clone)]
pub struct SubscriptIR {
    pub target: Box<ExprIR>,
    pub subscript: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}

impl SubscriptIR {
    pub fn new(target: ExprIR, subscript: ExprIR, span: Option<SourceSpan>) -> Self {
        Self {
            target: Box::new(target),
            subscript: Box::new(subscript),
            span,
        }
    }
}
