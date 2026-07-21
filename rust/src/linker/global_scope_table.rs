use std::{collections::HashMap};

use crate::{linker::{program_table::ProgramTable, symbol_ref::SymbolRef}};

pub struct ProgramSymbolTable {  // global scope PER PROGRAM
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

    pub fn lookup(
        &self,
        program_id: i64,
        name: &str,
    ) -> Option<&SymbolRef> {
        self.by_program_id
            .get(&program_id)?
            .by_name
            .get(name)
    }

    pub fn build(&mut self, programs: &ProgramTable) {
        for (&program_id, program) in &programs.by_id {
            let mut by_name = HashMap::new();
            for symbol in &program.symbols {
                if symbol.scope_id != 0 {
                    continue;
                }
                let symbol_ref = SymbolRef {
                    program_id,
                    symbol_id: symbol.id,
                };
                by_name.insert(symbol.name.clone(), symbol_ref);
            }
            self.by_program_id
                .insert(program_id, ProgramSymbolTable { by_name });
        }
    }
}
