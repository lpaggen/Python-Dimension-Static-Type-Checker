use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct DictIR {
    pub keys: Vec<Option<ExprIR>>,
    pub values: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
