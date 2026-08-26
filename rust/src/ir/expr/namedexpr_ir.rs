use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct NamedExprIR {
    pub target: Box<ExprIR>,
    pub value: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}
