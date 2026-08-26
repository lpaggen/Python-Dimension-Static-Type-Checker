use crate::ir::{expr_ir::ExprIR, nodes::CompIR, span_ir::SourceSpan};

#[derive(Debug, Clone)]
pub struct GeneratorExprIR {
    pub elt: Box<ExprIR>,
    pub generators: Vec<CompIR>,
    pub span: Option<SourceSpan>,
}
