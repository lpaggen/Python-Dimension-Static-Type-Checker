use crate::ir::{expr_ir::ExprIR, operator::Operator, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct BoolOpIR {
    pub values: Vec<ExprIR>,
    pub op: Operator,
    pub span: Option<SourceSpan>,
}
