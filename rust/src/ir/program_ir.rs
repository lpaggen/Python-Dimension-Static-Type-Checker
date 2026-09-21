use crate::ir::{
    metadata::{ScopeIR, SymbolIR},
    stmt::{StmtIR, import_ir::ImportIR},
};

// pub decls: Vec<DeclIR>, // may have been useful to keep classes and functions signatures actually

#[derive(Debug, Clone)]
pub struct ProgramIR {
    pub module_name: String,
    pub file_path: String,
    pub scopes: Vec<ScopeIR>,
    pub symbols: Vec<SymbolIR>,
    pub imports: Vec<ImportIR>,
    pub body: Vec<StmtIR>,
}
