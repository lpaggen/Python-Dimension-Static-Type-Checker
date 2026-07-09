use crate::ir::nodes::expr_ir::{ExprIR, Operator, SourceSpan};

#[derive(Debug, Clone)]
pub struct BinOpIR {
    pub left: Box<ExprIR>,
    pub op: Operator,
    pub right: Box<ExprIR>,
    pub span: SourceSpan,
}