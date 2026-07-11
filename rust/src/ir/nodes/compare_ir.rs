use crate::ir::{expr_ir::ExprIR, operator::Operator, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct CompareIR {
    pub left: Box<ExprIR>,
    pub ops: Vec<Operator>,
    pub comparators: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}

impl CompareIR {
    pub fn new(
        left: ExprIR,
        ops: Vec<Operator>,
        comparators: Vec<ExprIR>,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            left: Box::new(left),
            ops,
            comparators,
            span,
        }
    }
}
