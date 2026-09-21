#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolRef {
    pub program_id: usize,
    pub symbol_id: usize,
}
