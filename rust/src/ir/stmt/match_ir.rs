use crate::ir::{expr_ir::ExprIR, nodes::PatternIR, span_ir::SourceSpan, stmt_ir::StmtIR};

#[derive(Debug, Clone)]
pub struct MatchIR {
    pub subject: Box<ExprIR>,
    pub cases: Vec<MatchCaseIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct MatchCaseIR {
    pub scope_id: i64,
    pub pattern: PatternIR,
    pub guard: Option<ExprIR>,
    pub body: Vec<StmtIR>,
    pub span: Option<SourceSpan>,
}
