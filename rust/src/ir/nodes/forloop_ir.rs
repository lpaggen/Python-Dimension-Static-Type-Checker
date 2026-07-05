use crate::ir::{ExprIR, SourceSpan, StmtIR};

#[derive(Debug, Clone)]
pub struct ForLoopIR {
    pub target: Box<ExprIR>,
    pub iter: Box<ExprIR>,
    pub scope_id: i64,
    pub body_scope_id: i64,
    pub body: Vec<StmtIR>,
    pub orelse: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}

impl ForLoopIR {
    pub fn new(
        target: ExprIR,
        iter: ExprIR,
        scope_id: i64,
        body_scope_id: i64,
        body: Vec<StmtIR>,
        orelse: Vec<StmtIR>,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            target: Box::new(target),
            iter: Box::new(iter),
            scope_id,
            body_scope_id,
            body,
            orelse,
            span,
        }
    }
}
