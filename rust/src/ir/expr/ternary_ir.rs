use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct IfExprIR {
    pub test: Box<ExprIR>,
    pub body: Box<ExprIR>,
    pub orelse: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}
