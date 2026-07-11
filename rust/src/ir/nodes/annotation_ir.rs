use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct AnnotationIR {
    pub head: AnnotationHeadIR,
    pub args: Vec<ExprIR>,
}

#[derive(Debug, Clone)]
pub struct AnnotationHeadIR {
    pub root: String,
    pub attrs: Vec<String>,
    pub scope_id: i64,
    pub span: Option<SourceSpan>,
}

impl AnnotationIR {
    pub fn new(head: AnnotationHeadIR, args: Vec<ExprIR>) -> Self {
        Self {
            head,
            args,
        }
    }
}