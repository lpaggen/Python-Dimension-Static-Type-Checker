use std::{collections::{HashMap, VecDeque}, ops::Deref};

use crate::{control_flow::{
    basic_block::BasicBlock, bindingstate::BindingState, block_id::BlockID, bound_type::TypedBinding, cfg::Cfg, flowstate::FlowState, graph::Graph, programcfg::ProgramCfg
}, ir::{expr::ExprIR, nodes::SymbolIR, stmt::{StmtIR, annassign_ir}}, linker::{program_table::ProgramTable, symbol_ref::SymbolRef}, type_resolver::type_resolver::TypeResolver, types::types::Type};

pub struct BlockFlow<'ctx> {
    pub incoming: HashMap<BlockID, FlowState>,
    pub outgoing: HashMap<BlockID, FlowState>,

    type_resolver: TypeResolver<'ctx>,
}

impl<'ctx> BlockFlow<'ctx> {
    pub fn new(type_resolver: TypeResolver<'ctx>) -> Self {
        Self {
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
            type_resolver,
        }
    }

    // for every program, go one by one to resolve CFG instructions Bound, Unbound, MaybeUnbound and their type
    // we want to end up with something like: Bound(int | float), etc., so we need bound status + type inference
    // TODO fix huge bug, terminator None is getting unwrapped, causes issues
    pub fn analyze_cfg(&mut self, programcfg: &ProgramCfg, symbols: &Vec<SymbolIR>) {

        let entry = BlockID {id: 0};  // start at entry always

        let graph = &programcfg.module;

        // declare all symbols as Unbound and Unknown first, update their status as we go
        let mut entry_state = FlowState::new();
        for symbol in symbols {
            let symbol_ref = SymbolRef {
                program_id: programcfg.id,
                symbol_id: symbol.id,
            };

            entry_state.by_ref.insert(
                symbol_ref,
                TypedBinding {
                    binding: BindingState::Unbound,
                    ty: Type::Unknown,
                },
            );
        }

        self.incoming.insert(entry, entry_state);

        let mut queue: VecDeque<BlockID> = VecDeque::new();
        queue.push_back(entry);

        // suppose this has B3
        while let Some(id) = queue.pop_front() {
            let block = graph.blocks.get(&id).unwrap();

            // this just gets IN[B3], which we know from merge(OUT[predecessors])
            let mut state = self.incoming[&id].clone();

            for &stmt in &block.statements {
                // do_something(), specifics matter later 
                // probably we want to populate everything with UNBOUND(TYPE) first, then update binding status? 
                // yes, this is the right step moving ahead, makes life easier
                self.analyze_stmt(stmt, &mut state, programcfg.id);
            }

            let successors = graph.get_outgoing_ids(&id);

            self.outgoing.insert(id, state);

            for successor in &successors {
                let successor_block = &graph.blocks[successor];

                // find outgoing states of current block's predecessors
                let states = successor_block
                    .incoming
                    .iter()
                    .filter_map(|pred| self.outgoing.get(pred));

                // now merge the outgoing states of the current block's predecessors
                let merged = FlowState::merge(states);

                let changed = self
                    .incoming
                    .get(&successor)
                    .map(|old| old != &merged)
                    .unwrap_or(true);

                // if successor is B1 depends on B0, just set IN[B1] = merge(OUT(predecessors[B1])), that's it
                // we only want to do this if something has changed, else we run into infinite loops
                if changed {
                    self.incoming.insert(*successor, merged);
                    queue.push_back(*successor);
                }
            }
        }
    }

    pub fn analyze_stmt(&mut self, stmt: &StmtIR, state: &mut FlowState, program_id: i64) {
        match stmt {
            StmtIR::Assign(assign) => {
                let value_type =
                    self.type_resolver
                        .parse_expr(&assign.value, program_id, state);

                for target in &assign.targets {
                    // self.bind_target(target, &value_type, state, program_id);
                }
            }

            StmtIR::AnnAssign(annassign) => { // !! fix API for symbol_types eventually
                println!("{annassign:?}");
                let target_type = self.type_resolver.resolve_type(program_id, stmt, state);
            }

            _ => {}
        }
    }

    pub fn build(&mut self, cfg: &Cfg, programs: &ProgramTable) {
        for (id, programcfg) in &cfg.programs {
            self.analyze_cfg(
                programcfg,
                &programs.by_id.get(id).unwrap().symbols
            );
        }
    }
}
