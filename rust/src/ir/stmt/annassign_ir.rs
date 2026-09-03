use crate::ir::{expr::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]

pub struct AnnAssignIR {
    pub target: ExprIR,
    pub annotation: ExprIR,
    pub value: Option<ExprIR>,
    pub simple: i64,
    pub span: Option<SourceSpan>
}
