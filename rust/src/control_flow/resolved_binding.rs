use crate::{linker::symbol_ref::SymbolRef, types::types::Type};

pub struct ResolvedBinding {
    pub symbol_ref: SymbolRef,
    pub ty: Type,
}
