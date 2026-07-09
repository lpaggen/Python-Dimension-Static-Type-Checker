use crate::ir::nodes::{ExprIR, SourceSpan, StmtIR};

#[derive(Debug, Clone)]
pub struct IfIR {
    pub test: Box<ExprIR>,
    pub scope_id: i64,
    pub then_scope_id: i64,
    pub else_scope_id: i64,
    pub body: Vec<StmtIR>,
    pub orelse: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}

impl IfIR {
    pub fn new(
        test: ExprIR,
        scope_id: i64,
        then_scope_id: i64,
        else_scope_id: i64,
        body: Vec<StmtIR>,
        orelse: Vec<StmtIR>,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            test: Box::new(test),
            scope_id,
            then_scope_id,
            else_scope_id,
            body,
            orelse,
            span,
        }
    }
}
