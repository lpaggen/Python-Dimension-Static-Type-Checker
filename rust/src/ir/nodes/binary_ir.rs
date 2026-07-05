use crate::ir::expr_ir::ExprIR;
use crate::ir::operator::Operator;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct BinOpIR {
    pub left: Box<ExprIR>,
    pub op: BinaryOperatorIR,
    pub right: Box<ExprIR>,
    pub span: SourceSpan,
}