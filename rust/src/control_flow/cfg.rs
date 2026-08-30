use std::{collections::HashMap, ops::Deref};

use crate::{
    control_flow::{
        block_id::{
            BlockID, 
        }, graph::Graph, programcfg::ProgramCfg,
    }, ir::stmt::StmtIR, linker::program_table::ProgramTable
};

pub struct Cfg<'a> { // check if we can't make it usize, or does this require a huge refactor ?
    pub programs: HashMap<i64, ProgramCfg<'a>>,
}

// !! CFG just wants to BUILD the graph, doesn't care WHICH path execution takes, only models all paths
impl<'a> Cfg<'a> {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
        }
    }

    pub fn build(&mut self, programs: &'a ProgramTable) {
        for (pid, program) in &programs.by_id {
            let mut program_cfg = ProgramCfg::new();
            program_cfg.build_program(&program.body);
            self.programs.insert(*pid, program_cfg);
        }
    }
}
