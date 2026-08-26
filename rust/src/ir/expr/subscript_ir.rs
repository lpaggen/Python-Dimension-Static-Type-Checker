use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct SubscriptIR {
    pub value: Box<ExprIR>,
    pub slice: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}
