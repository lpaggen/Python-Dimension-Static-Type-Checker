use crate::ir::{AnnotationIR, ExprIR, SourceSpan, StmtIR};

#[derive(Debug, Clone)]
pub struct ParamIR {
    pub symbol_id: i64,
    pub name: String,
    pub annotation: Option<AnnotationIR>,
    pub default: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

impl ParamIR {
    pub fn new(
        symbol_id: i64,
        name: impl Into<String>,
        annotation: Option<AnnotationIR>,
        default: Option<ExprIR>,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            symbol_id,
            name: name.into(),
            annotation,
            default: default.map(Box::new),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnIR {
    pub value: Option<Box<ExprIR>>,
    pub span: Option<SourceSpan>,
}

impl ReturnIR {
    pub fn new(value: Option<ExprIR>, span: Option<SourceSpan>) -> Self {
        Self {
            value: value.map(Box::new),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionIR {
    pub id: i64,
    pub symbol_id: i64,
    pub name: String,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub params: Vec<ParamIR>,
    pub body: Vec<StmtIR>,
    pub returns: Option<AnnotationIR>,
    pub decorators: Vec<ExprIR>,
    pub span: Option<SourceSpan>,
}
