


pub struct ProgramIR {
    pub name: String,
    pub path: String, 
    pub scopes: Vec<ScopeIR>,
    pub symbols: Vec<SymbolIR>,
    pub imports: Vec<ImportIR>,
    pub declarations: Vec<BindingIR>
}