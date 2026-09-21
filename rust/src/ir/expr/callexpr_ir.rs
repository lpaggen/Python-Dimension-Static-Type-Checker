use crate::ir::{expr_ir::ExprIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct KeywordArgIR {
    pub name: String,
    pub value: Box<ExprIR>,
    pub span: SourceSpan,
}

impl KeywordArgIR {
    pub fn new(name: impl Into<String>, value: ExprIR, span: SourceSpan) -> Self {
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
    pub span: SourceSpan,
}

impl CallExprIR {
    pub fn new(
        callee: ExprIR,
        args: Vec<ExprIR>,
        kwargs: Vec<KeywordArgIR>,
        span: SourceSpan,
    ) -> Self {
        Self {
            callee: Box::new(callee),
            args,
            kwargs,
            span,
        }
    }
}
