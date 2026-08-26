use crate::ir::nodes::annotation_ir::AnnotationIR;
use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

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

impl From<i32> for BindingKind {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Assign,
            2 => Self::AnnAssign,
            _ => Self::Unknown,
        }
    }
}
