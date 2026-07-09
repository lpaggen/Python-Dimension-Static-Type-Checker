use crate::ir::nodes::{AnnotationIR, ExprIR, SourceSpan};

#[derive(Debug, Clone)]
pub struct BindingIR {
    pub id: String,
    pub target_id: i64,
    pub annotation: Option<AnnotationIR>,
    pub kind: i64,
    pub value: Option<Box<ExprIR>>,
    pub scope_id: i64,
    pub span: Option<SourceSpan>,
}

impl BindingIR {
    pub fn new(
        id: impl Into<String>,
        target_id: i64,
        annotation: Option<AnnotationIR>,
        kind: i64,
        value: Option<ExprIR>,
        scope_id: i64,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            id: id.into(),
            target_id,
            annotation,
            kind,
            value: value.map(Box::new),
            scope_id,
            span,
        }
    }
}
