use std::{collections::{HashMap, VecDeque}, ops::Deref};

use crate::{control_flow::{
    basic_block::BasicBlock, bindingstate::BindingState, block_id::BlockID, bound_type::TypedBinding, cfg::Cfg, flowstate::FlowState, graph::Graph, programcfg::ProgramCfg, terminator::Terminator
}, ir::{expr::{ConstantIR, ExprIR}, nodes::SymbolIR, operator::Operator, stmt::{StmtIR, annassign_ir}}, linker::{program_table::ProgramTable, resolution_table::{self, ResolutionTable}, scope_table::GlobalSymbolTable, symbol_ref::SymbolRef}, type_resolver::type_resolver::TypeResolver, types::types::Type};

pub struct BlockFlow<'ctx> {
    pub incoming: HashMap<BlockID, FlowState>,
    pub edge_states: HashMap<(BlockID, BlockID), FlowState>,  // necessary to preserve conditional guards on branches

    symbols: &'ctx GlobalSymbolTable,

    type_resolver: TypeResolver<'ctx>,
}

impl<'ctx> BlockFlow<'ctx> {
    pub fn new(type_resolver: TypeResolver<'ctx>, symbol_table: &'ctx GlobalSymbolTable) -> Self {
        Self {
            incoming: HashMap::new(),
            edge_states: HashMap::new(),
            type_resolver,
            symbols: symbol_table,
        }
    }

    fn update_successor(&mut self, graph: &Graph, successor: BlockID, queue: &mut VecDeque<BlockID>) {
        let successor_block = &graph.blocks[&successor];

        // find outgoing states of current block's predecessors
        let states = successor_block
            .incoming
            .iter()
            .filter_map(|pred| self.edge_states.get(&(*pred, successor)));

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
            self.incoming.insert(successor, merged);
            queue.push_back(successor);
        }

    }

    fn to_z3_bool(&self, expr: &ExprIR) -> z3::ast::Bool {
        match expr {
            ExprIR::Constant(ConstantIR::IntegerLit(intlit)) => {
                z3::ast::Bool::from_bool(intlit.value != 0)
            }

            ExprIR::Constant(ConstantIR::FloatLit(floatlit)) => {
                z3::ast::Bool::from_bool(floatlit.value != 0.0)
            }

            ExprIR::Constant(ConstantIR::BooleanLit(booleanlit)) => {
                z3::ast::Bool::from_bool(booleanlit.value)
            }

            ExprIR::Constant(ConstantIR::StringLit(stringlit)) => {
                z3::ast::Bool::from_bool(!stringlit.value.is_empty())
            }

            ExprIR::Constant(ConstantIR::NoneLit(_)) => {
                z3::ast::Bool::from_bool(false)
            }

            ExprIR::Constant(ConstantIR::EllipsisLit(_)) => {
                z3::ast::Bool::from_bool(true)
            }

            ExprIR::Constant(ConstantIR::BytesLit(byteslit)) => {
                z3::ast::Bool::from_bool(!byteslit.value.is_empty())
            }

            ExprIR::Constant(ConstantIR::ComplexLit(complexlit)) => {
                z3::ast::Bool::from_bool(
                    complexlit.real != 0.0 || complexlit.imag != 0.0
                )
            },

            ExprIR::BoolOpExpr(boolop) => {
                let guards: Vec<z3::ast::Bool> = boolop
                    .values
                    .iter()
                    .map(|expr| self.to_z3_bool(expr))
                    .collect();

                match boolop.op {
                    Operator::And => {
                        let refs: Vec<&z3::ast::Bool> = guards
                            .iter()
                            .collect();
                        z3::ast::Bool::and(&refs)
                    }

                    Operator::Or => {
                        let refs: Vec<&z3::ast::Bool> = guards
                            .iter()
                            .collect();
                        z3::ast::Bool::or(&refs)
                    }

                    _ => {
                        todo!()
                    }
                }
            },

            // add Comparison, Calls, etc etc etc everything we can, Name, whatever works

            _ => {  // default to assume accessible branch TODO check
                z3::ast::Bool::from_bool(true)
            }
        }
    }

    // for every program, go one by one to resolve CFG instructions Bound, Unbound, MaybeUnbound and their type
    // we want to end up with something like: Bound(int | float), etc., so we need bound status + type inference
    // TODO fix huge bug, terminator None is getting unwrapped, causes issues
    pub fn analyze_cfg(&mut self, programcfg: &ProgramCfg, symbols: &Vec<SymbolIR>) {

        let entry = BlockID {id: 0};  // start at entry always

        let graph = &programcfg.module;

        // declare all symbols as Unbound and Unknown first, update their status as we go
        // true guard means the block is reachable
        let mut entry_state = FlowState::new(z3::ast::Bool::from_bool(true));

        for symbol in symbols {
            let symbol_ref = SymbolRef {
                program_id: programcfg.id,
                symbol_id: symbol.id,
            };

            entry_state.register_unbound(&symbol_ref);
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
                self.analyze_stmt(stmt, &mut state, programcfg.id);
            }

            let successors = graph.get_outgoing_ids(&id);

            // update true and false targets with the guard, true gets "guard" false gets "NOT guard"
            // this helps later for z3 and for error reporting, we can tell the user why something may fail
            match block.terminator.as_ref() {
                Some(Terminator::Branch(branch)) => {

                    let z3_guard = self.to_z3_bool(&branch.condition);

                    let mut true_state = state.clone();

                    let mut false_state = state.clone();

                    // parent condition AND current guard
                    true_state.guard = z3::ast::Bool::and(&[
                        &state.guard,
                        &z3_guard,
                    ]);

                    let not_condition = z3_guard.not();

                    false_state.guard = z3::ast::Bool::and(&[
                        &state.guard,
                        &not_condition,
                    ]);

                    self.edge_states.insert(
                        (id, branch.true_target), 
                        true_state
                    );

                    self.edge_states.insert(
                        (id, branch.false_target), 
                        false_state
                    );

                    self.update_successor(
                        graph, 
                        branch.true_target, 
                        &mut queue
                    );

                    self.update_successor(
                        graph, 
                        branch.false_target, 
                        &mut queue
                    );
                },

                _ => {
                    for successor in successors {
                        self.edge_states.insert(
                            (id, successor),
                            state.clone(),
                        );

                        self.update_successor(
                            graph, 
                            successor, 
                            &mut queue);
                    }
                }
            }
        }
    }

    pub fn analyze_stmt(
        &mut self,
        stmt: &StmtIR,
        state: &mut FlowState,
        program_id: i64,
    ) {
        match stmt {
            StmtIR::Assign(assign) => {
                let value_type =
                    self.type_resolver
                        .parse_expr(&assign.value, program_id, state);

                for target in &assign.targets {
                    if let ExprIR::Name(name) = target {
                        let symbol_ref = match self.symbols.lookup_by_name(
                            program_id,
                            name.use_scope_id,
                            &name.id,
                        ) {
                            Some(symbol_ref) => {
                                symbol_ref
                            }

                            None => {
                                // TODO diag, DO NOT FAIL SILENTLY
                                continue;
                            }
                        };

                        state.bind(
                            &symbol_ref,
                            value_type.clone(),
                        );
                    }
                }
            }

            StmtIR::AnnAssign(annassign) => {
                // println!("{annassign:?}");
                let target_type = self.type_resolver.resolve_type(program_id, stmt, state);
                    if let ExprIR::Name(name) = &annassign.target {
                        let symbol_ref = self.symbols
                            .lookup_by_name(
                                program_id,
                                name.use_scope_id,
                                &name.id,
                            )
                            .unwrap();

                        state.bind(
                            &symbol_ref,
                            target_type.clone(),
                        );
                    }
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
