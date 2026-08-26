use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct TupleIR {
    pub elts: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
