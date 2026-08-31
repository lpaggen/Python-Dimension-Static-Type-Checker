use crate::ir::{expr::ExprIR, nodes::AnnotationIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]

pub struct AnnAssignIR {
    pub target: ExprIR,
    pub annotation: AnnotationIR,
    pub value: Option<ExprIR>,
    pub simple: i64,
    pub span: Option<SourceSpan>
}
