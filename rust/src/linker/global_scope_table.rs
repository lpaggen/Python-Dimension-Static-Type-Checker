// use rayon::prelude::*;
use std::collections::HashMap;

use crate::linker::{program_table::ProgramTable, symbol_ref::SymbolRef};

pub struct ProgramSymbolTable {
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
    pub fn lookup_by_name(&self, program_id: i64, mut scope_id: i64, name: &str) -> Option<SymbolRef> {
        self.by_program_id
            .get(&program_id)?
            .by_scope_id
            .get(&scope_id)?
            .by_name
            .get(name)
            .copied()
    }

    pub fn global_lookup(&self, program_id: i64, name: &str) -> Option<SymbolRef> {
        self.lookup_by_name(
            program_id, 
            0, 
            name
        )
    }

    // !! i made this use threads, it only slows it down for small programs, from 10 microseconds to 1ms with my tests
    // depends, if programs are huge, maybe it makes sense, will see
    // keeping sequential for now
    pub fn build(&mut self, programs: &ProgramTable) {

        for (&program_id, program) in &programs.by_id {
            let mut program_symbols = ProgramSymbolTable {
                by_scope_id: HashMap::new()
            };

            for symbol in &program.symbols {
                // avoid overwriting when two symbols exist in a single scope
                let scope_symbol_table = program_symbols
                    .by_scope_id
                    .entry(symbol.scope_id)
                    .or_insert(ScopeSymbolTable {
                        by_name: HashMap::new(),
                    });

                scope_symbol_table.by_name.insert(
                    symbol.name.clone(), 
                    SymbolRef {
                    program_id,
                    symbol_id: symbol.id,
                });
            }

            self.by_program_id.insert(program_id, program_symbols);
        }
    }
}
