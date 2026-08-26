use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct AssertIR {
    pub test: ExprIR,
    pub msg: Option<ExprIR>,
    pub span: Option<SourceSpan>,
}
