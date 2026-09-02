// in later build will just make this and SymbolRef UniqueID or something, so all can use it alike, since it's the same invariant

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeRef {
    pub program_id: i64,
    pub scope_id: i64,
}
