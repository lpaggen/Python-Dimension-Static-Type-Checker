use rayon::prelude::*;
use std::collections::HashMap; // 1. Bring Rayon's parallel iterators into scope

use crate::linker::{program_table::ProgramTable, symbol_ref::SymbolRef};

pub struct ProgramSymbolTable {
    pub globals: HashMap<String, SymbolRef>,
    pub by_scope_id: HashMap<i64, ScopeSymbolTable>
}

// by name lookup for later phases in which statements do NOT own an ID
pub struct ScopeSymbolTable {
    pub by_name: HashMap<String, SymbolRef>,
}

pub struct GlobalSymbolTable {
    pub by_program_id: HashMap<i64, ProgramSymbolTable>,
}

impl GlobalSymbolTable {
    pub fn new() -> Self {
        Self {
            by_program_id: HashMap::new(),
        }
    }

    // we need some "scopes" structure too, HashMap of ID -> scope, TODO in ProgramTable maybe
    pub fn lookup_by_name(&self, program_id: i64, name: &str, mut scope_id: i64) -> Option<SymbolRef> {
        todo!()
    }

    pub fn global_lookup(&self, program_id: i64, name: &str) -> Option<&SymbolRef> {
        self.by_program_id.get(&program_id)?.globals.get(name)
    }

    pub fn build_global_symbols(&mut self, programs: &ProgramTable) {
        self.by_program_id = programs
            .by_id
            .par_iter()
            .map(|(&program_id, program)| {
                let mut globals = HashMap::new();
                for symbol in &program.symbols {
                    if symbol.scope_id != 0 {
                        continue;
                    }
                    let symbol_ref = SymbolRef {
                        program_id,
                        symbol_id: symbol.id,
                    };
                    globals.insert(symbol.name.clone(), symbol_ref);
                }

                (program_id, ProgramSymbolTable { globals, by_scope_id: HashMap::new() })
            })
            .collect();
    }
}
