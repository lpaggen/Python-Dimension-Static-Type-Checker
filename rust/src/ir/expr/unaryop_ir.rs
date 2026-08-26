use crate::ir::{expr_ir::ExprIR, operator::Operator, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct UnaryOpIR {
    pub op: Operator,
    pub operand: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}
