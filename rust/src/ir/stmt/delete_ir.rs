use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct DeleteIR {
    pub targets: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
