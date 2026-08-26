use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub struct CompIR {
    pub target: Box<ExprIR>,
    pub iter: Box<ExprIR>,
    pub ifs: Vec<ExprIR>,
    pub is_async: bool,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct ListCompIR {
    pub elt: Box<ExprIR>,
    pub generators: Vec<CompIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct SetCompIR {
    pub elt: Box<ExprIR>,
    pub generators: Vec<CompIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct DictCompIR {
    pub key: Box<ExprIR>,
    pub value: Box<ExprIR>,
    pub generators: Vec<CompIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct GeneratorExpIR {
    pub elt: Box<ExprIR>,
    pub generators: Vec<CompIR>,
    pub span: Option<SourceSpan>,
}
