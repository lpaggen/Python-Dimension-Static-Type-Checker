use crate::ir::nodes::{ExprIR, SourceSpan};

#[derive(Debug, Clone)]
pub struct AnnotationIR {
    // Keep this flexible until the full annotation_ir.py is available.
    // For Tensor[Batch, In], this can point at a SubscriptIR/IdentifierIR tree.
    pub value: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

impl AnnotationIR {
    pub fn new(value: Option<ExprIR>, span: Option<SourceSpan>) -> Self {
        Self {
            value: value.map(Box::new),
            span,
        }
    }
}
