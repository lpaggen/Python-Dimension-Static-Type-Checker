use std::collections::HashMap;

use crate::{control_flow::cfg::Cfg, linker::program_table::ProgramTable};

pub struct CfgTable<'a> {
    pub programs: HashMap<usize, Cfg<'a>>,
}

impl<'a> CfgTable<'a> {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
        }
    }

    pub fn build(&mut self, table: &'a ProgramTable) {
        for (id, program) in &table.by_id {
            let mut cfg = Cfg::new(*id);

            cfg.build_program(&program.body);

            self.programs.insert(*id, cfg);
        }
    }
}
