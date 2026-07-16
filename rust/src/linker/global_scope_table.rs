use std::{collections::HashMap};

use crate::{ir::nodes::ProgramIR, linker::symbol_ref::SymbolRef};



pub struct GlobalSymbolTable {
    /// Stores global scope of a program in a convenient lookup table so we don't need to iterate to find symbols by name
    pub by_name: HashMap<String, SymbolRef>,
}

impl GlobalSymbolTable {
    pub fn new() -> Self {
        Self {
            by_name: HashMap::new(),
        }
    }

    pub fn build(program_id: i64, program: &ProgramIR) -> Self {
        let mut by_name = HashMap::new();
        for symbol in &program.symbols {
            if symbol.scope_id != 0 { // 0 is global scope always, so consider only top-level symbols
                continue;
            }
            let symbol_ref = SymbolRef {
                program_id, 
                symbol_id: symbol.id
            };
            by_name.insert(symbol.name.clone(), symbol_ref);
        }
        return Self { 
            by_name 
        };
    }
}
