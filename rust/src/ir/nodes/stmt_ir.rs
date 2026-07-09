use crate::ir::nodes::{
    AugAssignIR, DeclIR, ExprStmtIR, ForLoopIR, IfIR, ImportIR, ReturnIR, SourceSpan, WhileLoopIR,
};

#[derive(Debug, Clone)]
pub enum StmtIR {
    ExprStmt(ExprStmtIR),
    Return(ReturnIR),
    AugAssign(AugAssignIR),
    If(IfIR),
    WhileLoop(WhileLoopIR),
    ForLoop(ForLoopIR),
    Import(ImportIR),

    // Useful for nested function/class/binding declarations inside bodies.
    Decl(DeclIR),
}

impl StmtIR {
    pub fn span(&self) -> Option<SourceSpan> {
        match self {
            Self::ExprStmt(node) => node.span,
            Self::Return(node) => node.span,
            Self::AugAssign(node) => node.span,
            Self::If(node) => node.span,
            Self::WhileLoop(node) => node.span,
            Self::ForLoop(node) => node.span,
            Self::Import(node) => node.span,
            Self::Decl(node) => node.span(),
        }
    }
}
