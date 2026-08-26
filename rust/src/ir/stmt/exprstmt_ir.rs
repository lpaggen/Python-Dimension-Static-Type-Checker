use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct ExprStmtIR {
    pub value: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}
