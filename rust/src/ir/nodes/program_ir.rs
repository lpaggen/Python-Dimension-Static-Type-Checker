use crate::ir::nodes::{decl_ir::DeclIR, import_ir::ImportIR, scope_ir::ScopeIR, symbol_ir::SymbolIR};

#[derive(Debug, Clone)]
pub struct ProgramIR {
    pub module_name: String,
    pub file_path: String,
    pub scopes: Vec<ScopeIR>,
    pub symbols: Vec<SymbolIR>,
    pub imports: Vec<ImportIR>,
    pub decls: Vec<DeclIR>,
}
