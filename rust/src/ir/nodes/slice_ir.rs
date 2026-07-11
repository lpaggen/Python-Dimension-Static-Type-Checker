use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct SliceIR {
    pub lower: Option<Box<ExprIR>>,
    pub upper: Option<Box<ExprIR>>,
    pub step: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}
