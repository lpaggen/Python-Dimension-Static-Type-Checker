use std::{collections::{HashMap, VecDeque}, ops::Deref};

use crate::{control_flow::{
    basic_block::BasicBlock, block_id::BlockID, cfg::Cfg, flowstate::FlowState, programcfg::ProgramCfg
}, ir::stmt::StmtIR};

pub struct BlockFlow {
    pub incoming: HashMap<BlockID, FlowState>,
    pub outgoing: HashMap<BlockID, FlowState>,
}

impl BlockFlow {
    pub fn new() -> Self {
        Self {
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
        }
    }

    // for every program, go one by one to resolve CFG instructions Bound, Unbound, MaybeUnbound and their type
    pub fn merge_incoming(&mut self, programcfg: &ProgramCfg) {

        let entry = BlockID {id: 0};  // start at entry always

        let graph = &programcfg.module;

        self.incoming.insert(entry, FlowState::new());

        let mut queue: VecDeque<BlockID> = VecDeque::new();
        queue.push_back(entry);

        // start with entry BlockID, push its successor BlockIDs, etc etc
        while let Some(id) = queue.pop_front() {
            let block = graph.blocks.get(&id).unwrap();

            // avoids &&
            for &stmt in &block.statements {
                self.analyze_stmt(stmt);
            }

            // surely there must be a better way? maybe "get_outgoing" function
            let successors = graph.get_outgoing_ids(&id);

            queue.extend(successors);
        }
    }

    pub fn analyze_stmt(&mut self, stmt: &StmtIR) {
        match stmt { // !! change back to Assign / AnnAssign, Binding is bad practice.
            StmtIR::Binding(binding_stmt) => {
                
            }

            _ => {

            }
        }
    }
}
