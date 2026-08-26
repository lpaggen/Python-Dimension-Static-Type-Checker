use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct YieldIR {
    pub value: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}
