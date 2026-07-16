use std::{collections::HashMap, hash::Hash};

use crate::ir::nodes::ProgramIR;

pub struct ProgramTable {
    /// ProgramTable is a global table used to resolve imports
    pub by_id: HashMap<i64, ProgramIR>,
    pub by_name: HashMap<String, i64>,
}

impl ProgramTable {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_name: HashMap::new(),
        }
    }

    pub fn build_tables(&mut self, programs: Vec<ProgramIR>) {
        for (id, program) in programs.into_iter().enumerate() {

            let id = id as i64;
            let module_name = program.module_name.clone();

            self.by_name.insert(module_name, id);
            self.by_id.insert(id, program);
        }
    }

    pub fn get_by_id(&self, id: i64) -> Option<&ProgramIR> {
        self.by_id.get(&id)
    }

    pub fn get_by_name(&self, module_name: &str) -> Option<&ProgramIR> {
        let id: &i64 = self.by_name.get(module_name)?;
        self.by_id.get(id)
    }
}
