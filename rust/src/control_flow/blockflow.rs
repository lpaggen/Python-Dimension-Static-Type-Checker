use std::{collections::{HashMap, VecDeque}, ops::Deref};

use crate::{control_flow::{
    basic_block::BasicBlock, block_id::BlockID, cfg::Cfg, flowstate::FlowState, programcfg::ProgramCfg
}, ir::stmt::{StmtIR, annassign_ir}, type_resolver::type_resolver::TypeResolver};

pub struct BlockFlow<'flow, 'ctx> {
    pub incoming: HashMap<BlockID, FlowState>,
    pub outgoing: HashMap<BlockID, FlowState>,

    type_resolver: &'flow TypeResolver<'ctx>,
}

impl<'flow, 'ctx> BlockFlow<'flow, 'ctx> {
    pub fn new(type_resolver: &'flow TypeResolver<'ctx>) -> Self {
        Self {
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
            type_resolver,
        }
    }

    // for every program, go one by one to resolve CFG instructions Bound, Unbound, MaybeUnbound and their type
    // we want to end up with something like: Bound(int | float), etc., so we need bound status + type inference
    pub fn merge_incoming(&mut self, programcfg: &ProgramCfg, symbol_types: &TypeResolver) {

        let entry = BlockID {id: 0};  // start at entry always

        let graph = &programcfg.module;

        self.incoming.insert(entry, FlowState::new());

        let mut queue: VecDeque<BlockID> = VecDeque::new();
        queue.push_back(entry);

        // start with entry BlockID, push its successor BlockIDs, etc etc
        while let Some(id) = queue.pop_front() {
            let block = graph.blocks.get(&id).unwrap();

            // should be cheap to clone
            let mut state = self.incoming[&id].clone();

            // avoids &&
            for &stmt in &block.statements {
                self.analyze_stmt(stmt, &mut state, symbol_types, programcfg.id);
            }

            let successors = graph.get_outgoing_ids(&id);

            queue.extend(successors);
        }
    }

    pub fn analyze_stmt(&mut self, stmt: &StmtIR, state: &mut FlowState, symbol_types: &TypeResolver, program_id: i64) {
        match stmt {
            // both these should call parse_stmt from symbol_type_table.rs
            StmtIR::Assign(_) => {
                // !!!!! needs support for MULTIPLE TARGETS, so multiple types maybe
            }

            StmtIR::AnnAssign(_) => { // !! fix API for symbol_types eventually
                // let target_type = symbol_types.parse_stmt(program_id, stmt, symbols, resolutions, diagnostics);
            }

            _ => {
                todo!()
            }
        }
    }
}
