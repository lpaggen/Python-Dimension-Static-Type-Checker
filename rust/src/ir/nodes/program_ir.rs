use crate::ir::{DeclIR, ImportIR, ScopeIR, SymbolIR};

#[derive(Debug, Clone)]
pub struct ProgramIR {
    pub module_name: String,
    pub file_path: String,
    pub scopes: Vec<ScopeIR>,
    pub symbols: Vec<SymbolIR>,
    pub imports: Vec<ImportIR>,
    pub decls: Vec<DeclIR>,
}
