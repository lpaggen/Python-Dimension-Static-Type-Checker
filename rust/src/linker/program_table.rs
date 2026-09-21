use std::collections::HashMap;

use crate::ir::program_ir::ProgramIR;

// use rayon::prelude::*;

pub struct ProgramTable {
    pub by_id: HashMap<usize, ProgramIR>,
    pub by_name: HashMap<String, usize>,
}

impl ProgramTable {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_name: HashMap::new(),
            // scope_map: HashMap::new(),
        }
    }

    pub fn build_tables(&mut self, programs: Vec<ProgramIR>) {
        for program in programs.into_iter() {
            let module_name = program.module_name.clone();

            self.by_name.insert(module_name, program.id);
            self.by_id.insert(program.id, program);
        }
    }

    pub fn get_by_id(&self, id: usize) -> Option<&ProgramIR> {
        self.by_id.get(&id)
    }

    pub fn get_by_name(&self, module_name: &str) -> Option<&ProgramIR> {
        let id: &usize = self.by_name.get(module_name)?;
        self.by_id.get(id)
    }
}
