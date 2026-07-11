use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct KeywordArgIR {
    pub name: String,
    pub value: Box<ExprIR>,
    pub span: Option<SourceSpan>,
}

impl KeywordArgIR {
    pub fn new(name: impl Into<String>, value: ExprIR, span: Option<SourceSpan>) -> Self {
        Self {
            name: name.into(),
            value: Box::new(value),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallExprIR {
    pub callee: Box<ExprIR>,
    pub args: Vec<ExprIR>,
    pub kwargs: Vec<KeywordArgIR>,
    pub span: Option<SourceSpan>,
}

impl CallExprIR {
    pub fn new(
        callee: ExprIR,
        args: Vec<ExprIR>,
        kwargs: Vec<KeywordArgIR>,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            callee: Box::new(callee),
            args,
            kwargs,
            span,
        }
    }
}
