use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct ListIR {
    pub elements: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
