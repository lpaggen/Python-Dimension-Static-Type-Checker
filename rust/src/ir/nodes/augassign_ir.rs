use crate::ir::{ExprIR, Operator, SourceSpan};

#[derive(Debug, Clone)]
pub struct AugAssignIR {
    // Python dump annotates target as str, but to_proto calls target.to_proto(),
    // so this is modeled as an expression target.
    pub target: Box<ExprIR>,
    pub op: Operator,
    pub value: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}

impl AugAssignIR {
    pub fn new(target: ExprIR, op: Operator, value: ExprIR, span: Option<SourceSpan>) -> Self {
        Self {
            target: Box::new(target),
            op,
            value: Box::new(value),
            span,
        }
    }
}
