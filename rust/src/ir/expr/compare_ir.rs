use crate::ir::{expr_ir::ExprIR, operator::Operator, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct CompareIR {
    pub left: Box<ExprIR>,
    pub ops: Vec<Operator>,
    pub comparators: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
