// use rayon::prelude::*;
use std::collections::HashMap;

use crate::linker::{program_table::ProgramTable, symbol_ref::SymbolRef};

pub struct ProgramSymbolTable {
    pub by_scope_id: HashMap<i64, ScopeSymbolTable>
}

pub struct ScopeSymbolTable {
    pub parent_id: Option<i64>,
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

    pub fn lookup_by_name(
        &self,
        program_id: i64,
        mut scope_id: i64,
        name: &str,
    ) -> Option<SymbolRef> {
        loop {
            let program = self.by_program_id.get(&program_id)?;

            if let Some(symbol_ref) = program
                .by_scope_id
                .get(&scope_id)
                .and_then(|scope| scope.by_name.get(name))
                .copied()
            {
                return Some(symbol_ref);
            }

            let scope = program.by_scope_id.get(&scope_id)?;

            match scope.parent_id {
                Some(parent_id) => scope_id = parent_id,
                None => return None,
            }
        }
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

            for scope in &program.scopes {
                program_symbols.by_scope_id.insert(
                    scope.id,
                    ScopeSymbolTable {
                        parent_id: scope.parent_id,
                        by_name: HashMap::new(),
                    },
                );
            }

            for symbol in &program.symbols {
                let scope = program_symbols
                    .by_scope_id
                    .get_mut(&symbol.scope_id)
                    .expect("symbol references nonexistent scope");

                scope.by_name.insert(
                    symbol.name.clone(),
                    SymbolRef {
                        program_id,
                        symbol_id: symbol.id,
                    },
                );
            }

            self.by_program_id.insert(program_id, program_symbols);
        }
    }
}
