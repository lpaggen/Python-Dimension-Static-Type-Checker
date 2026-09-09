pub struct SymbolicContext {
    by_symbol: HashMap<SymbolRef, z3::ast::Bool>,
    by_z3_name: HashMap<String, SymbolRef>,
}