use crate::ir::{ExprIR, Operator, SourceSpan};

#[derive(Debug, Clone)]
pub struct UnaryOpIR {
    pub op: Operator,
    pub operand: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}

impl UnaryOpIR {
    pub fn new(op: Operator, operand: ExprIR, span: Option<SourceSpan>) -> Self {
        Self {
            op,
            operand: Box::new(operand),
            span,
        }
    }
}
