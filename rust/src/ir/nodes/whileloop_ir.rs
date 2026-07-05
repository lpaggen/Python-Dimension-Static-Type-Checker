use crate::ir::{ExprIR, SourceSpan, StmtIR};

#[derive(Debug, Clone)]
pub struct WhileLoopIR {
    pub test: Box<ExprIR>,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub body: Vec<StmtIR>,
    pub orelse: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}

impl WhileLoopIR {
    pub fn new(
        test: ExprIR,
        scope_id: i64,
        body_scope_id: i64,
        body: Vec<StmtIR>,
        orelse: Vec<StmtIR>,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            test: Box::new(test),
            scope_id,
            body_scope_id,
            body,
            orelse,
            span,
        }
    }
}
