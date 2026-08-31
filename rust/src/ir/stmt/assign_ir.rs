use crate::ir::{expr::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]

pub struct AssignIR {
    pub targets: Vec<ExprIR>,
    pub value: ExprIR,
    pub type_comment: Option<String>,
    pub span: Option<SourceSpan>,
}
