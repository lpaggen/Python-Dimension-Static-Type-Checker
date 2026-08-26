use crate::ir::{
    metadata::{ScopeIR, SymbolIR},
    stmt::{decl_ir::DeclIR, import_ir::ImportIR, StmtIR},
};

#[derive(Debug, Clone)]
pub struct ProgramIR {
    pub module_name: String,
    pub file_path: String,
    pub scopes: Vec<ScopeIR>,
    pub symbols: Vec<SymbolIR>,
    pub imports: Vec<ImportIR>,
    pub decls: Vec<DeclIR>,
    pub body: Vec<StmtIR>,
}
