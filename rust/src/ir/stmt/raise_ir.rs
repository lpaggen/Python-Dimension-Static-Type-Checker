use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct RaiseIR {
    pub exc: Option<ExprIR>,
    pub cause: Option<ExprIR>,
    pub span: Option<SourceSpan>,
}
