use crate::{ir::span_ir::SourceSpan, linker::symbol_ref::SymbolRef};

#[derive(Debug, Clone)]
pub struct NameIR {
    pub id: String,
    pub use_scope_id: usize,
    pub symbol_ref: Option<SymbolRef>,
    pub span: SourceSpan,
}
