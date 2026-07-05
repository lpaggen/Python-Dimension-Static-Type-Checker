use crate::ir::{ExprIR, Operator, SourceSpan};

#[derive(Debug, Clone)]
pub struct BinOpIR {
    pub left: Box<ExprIR>,
    pub right: Box<ExprIR>,
    pub op: Operator,
    pub span: Option<SourceSpan>,
}

impl BinOpIR {
    pub fn new(left: ExprIR, right: ExprIR, op: Operator, span: Option<SourceSpan>) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            op,
            span,
        }
    }
}
