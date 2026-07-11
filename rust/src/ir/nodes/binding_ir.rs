use crate::ir::nodes::{annotation_ir::AnnotationIR};
use crate::ir::{span_ir::SourceSpan, expr_ir::ExprIR};

#[derive(Debug, Clone)]
pub struct BindingIR {
    pub id: i64,
    pub target_id: i64,
    pub annotation: Option<AnnotationIR>,
    pub kind: BindingKind,
    pub value: Option<Box<ExprIR>>,
    pub scope_id: i64,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    Unknown,
    Assign,
    AnnAssign,
}

impl BindingIR {
    pub fn new(
        id: i64,
        target_id: i64,
        annotation: Option<AnnotationIR>,
        kind: BindingKind,
        value: Option<ExprIR>,
        scope_id: i64,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            id,
            target_id,
            annotation,
            kind,
            value: value.map(Box::new),
            scope_id,
            span,
        }
    }
}

impl From<i32> for BindingKind {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Assign,
            2 => Self::AnnAssign,
            _ => Self::Unknown,
        }
    }
}
