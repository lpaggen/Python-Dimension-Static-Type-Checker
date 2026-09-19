use std::{cell::RefCell, rc::Rc};

use crate::{control_flow::{block_id::{ClassID, FunctionID}, blockflow::BlockFlow, call_binding::CallBinding, cfg_analysis_engine::{blocked_function_analysis::BlockedFunctionAnalysis, contour_id::ContourID, flow_block_id::FlowBlockID, function_contract_key::FunctionSpecializationKey, functioncontract_table::FunctionContractTable}, cfg_table::CfgTable, class_cfg::ClassCfg, flowstate::FlowState, function_analysis_request::FunctionAnalysisRequest, function_cfg::FunctionCfg, function_contract::{ContractParam, FunctionContract, GuardedReturn}, module_cfg::ModuleCfg, terminator::Terminator}, ir::{nodes::SymbolIR, span_ir::SourceSpan}, linker::{program_table::ProgramTable, symbol_ref::SymbolRef}, solver::BoolExpr, types::types::Type};

pub struct AnalysisEngine<'ctx> {
    pub flow: BlockFlow<'ctx>,
    pub contracts: Rc<RefCell<FunctionContractTable>>,
}

impl<'ctx> AnalysisEngine<'ctx> {
    pub fn run(
        &mut self, 
        cfg: &CfgTable, 
        programs: &ProgramTable
    ) -> Result<(), BlockedFunctionAnalysis> {
        for (id, program_cfg) in &cfg.programs {
            let program = programs
                .by_id
                .get(id)
                .expect("CFG exists without corresponding ProgramIR");

            for (class_id, class_cfg) in &program_cfg.classes {
                self.analyze_class(
                    *id, 
                    *class_id,
                    class_cfg, 
                    &program.symbols
                )?;
            }

            for (function_id, function_cfg) in &program_cfg.functions {
                let contract = self.analyze_function(
                    *id, 
                    *function_id, 
                    function_cfg, 
                    &program.symbols,
                )?;

                self.contracts
                    .borrow_mut()
                    .by_id
                    .insert(*function_id, contract);
            }

            let mut result = self.analyze_module(
                *id,
                ContourID::Module(0),
                &program_cfg.module,
                &program.symbols,
            );

            loop {
                match result {
                    Ok(()) => break,

                    Err(blocked) => {
                        self.resolve_specialization(
                            cfg,
                            programs,
                            &blocked.request,
                        )?;

                        result = self.resume_blocked(
                            cfg,
                            blocked,
                        );
                    }
                }
            }
        }

        Ok(())

    }

    fn analyze_function_specialized(
        &mut self,
        program_id: i64,
        function_id: FunctionID,
        function: &FunctionCfg,
        symbols: &[SymbolIR],
        bindings: &[CallBinding],
        call_site: &SourceSpan,
    ) -> Result<FunctionContract, BlockedFunctionAnalysis> {
        let contour_id = ContourID::Function(function_id);

        let mut state =
            FlowState::new(BoolExpr::from_bool(true));

        for symbol in symbols {
            if symbol.scope_id != function.scope_id {
                continue;
            }

            state.register_unbound(&SymbolRef {
                program_id,
                symbol_id: symbol.id,
            });
        }

        let mut contract = FunctionContract::new();

        for param in &function.params {
            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: param.symbol_id,
            };

            let declared_ty = match &param.annotation {
                Some(expr) => {
                    self.flow
                        .type_resolver
                        .parse_annotation(expr, program_id)
                }

                None => Type::Unknown,
            };

            let entry_ty = bindings
                .iter()
                .find(|binding| binding.symbol_id == param.symbol_id)
                .map(|binding| binding.ty.clone())
                .unwrap_or_else(|| declared_ty.clone());

            contract.params.push(ContractParam {
                symbol_id: param.symbol_id,
                ty: declared_ty,
                default: param
                    .default
                    .as_ref()
                    .map(|default| *default.clone()),
                kind: param.kind.clone(),
            });

            state.bind(
                &symbol_ref,
                entry_ty,
            );
        }

        contract.declared_return_type =
            match &function.returns {
                Some(expr) => self
                    .flow
                    .type_resolver
                    .parse_annotation(expr, program_id),

                None => Type::Unknown,
            };

        let previous_diagnostic_span = self
            .flow
            .type_resolver
            .replace_diagnostic_span_override(Some(call_site.clone()));

        let analysis_result = self
            .flow
            .analyze_body(program_id, contour_id, &function.graph, state)
            .and_then(|_| self.collect_function_returns(program_id, contour_id, function));

        self.flow
            .type_resolver
            .replace_diagnostic_span_override(previous_diagnostic_span);

        contract.returns = analysis_result?;

        Ok(contract)
    }

    fn resolve_specialization(
        &mut self,
        cfg: &CfgTable,
        programs: &ProgramTable,
        request: &FunctionAnalysisRequest,
    ) -> Result<(), BlockedFunctionAnalysis> {
        let program_cfg = cfg
            .programs
            .get(&request.program_id)
            .unwrap();

        let program = programs
            .by_id
            .get(&request.program_id)
            .unwrap();

        let function = program_cfg
            .functions
            .get(&request.function_id)
            .unwrap();

        let contract = self.analyze_function_specialized(
            request.program_id,
            request.function_id,
            function,
            &program.symbols,
            &request.bindings,
            &request.call_site,
        )?;

        let key = FunctionSpecializationKey {
            function_id: request.function_id,
            params: request
                .bindings
                .iter()
                .map(|binding| binding.ty.clone())
                .collect(),
        };

        self.contracts
            .borrow_mut()
            .specialized
            .insert(key, contract);

        Ok(())
    }

    fn resume_blocked(
        &mut self,
        cfg: &CfgTable,
        blocked: BlockedFunctionAnalysis
    ) -> Result<(), BlockedFunctionAnalysis> {
        let program_cfg = cfg
            .programs
            .get(&blocked.program_id)
            .unwrap();

        let graph = match blocked.contour {
            ContourID::Module(_) => {
                &program_cfg.module.graph
            },

            ContourID::Function(function_id) => {
                &program_cfg
                    .functions
                    .get(&function_id)
                    .unwrap()
                    .graph
            },

            ContourID::Class(class_id) => {
                &program_cfg
                    .classes
                    .get(&class_id)
                    .unwrap()
                    .graph
            },
        };

        self.flow.resume_from_block(
            blocked.program_id,
            blocked.contour,
            graph,
            blocked.block_id,
        )
    }

    fn analyze_module(
        &mut self,
        program_id: i64,
        contour_id: ContourID,
        module: &ModuleCfg,
        symbols: &[SymbolIR],
    ) -> Result<(), BlockedFunctionAnalysis> {
        let mut state = FlowState::new(BoolExpr::from_bool(true));

        for symbol in symbols {
            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: symbol.id,
            };

            state.register_unbound(&symbol_ref);
        }

        self.flow.analyze_body(
            program_id, 
            contour_id, 
            &module.graph, 
            state
        )?;

        Ok(())
    }

    fn analyze_function(
        &mut self,
        program_id: i64,
        function_id: FunctionID,
        function: &FunctionCfg,
        symbols: &[SymbolIR],
    ) -> Result<FunctionContract, BlockedFunctionAnalysis> {
        let mut state = FlowState::new(BoolExpr::from_bool(true));

        let contour_id = ContourID::Function(function_id);

        for symbol in symbols {
            if symbol.scope_id != function.scope_id {
                continue;
            }

            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: symbol.id,
            };

            state.register_unbound(&symbol_ref);
        }

        let mut contract = FunctionContract::new();

        for param in &function.params {
            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: param.symbol_id,
            };

            let param_type = match &param.annotation {
                Some(expr) => self.flow.type_resolver.parse_annotation(expr, program_id),

                None => Type::Unknown,
            };

            let param_default = param.default.as_ref().map(|default| *default.clone());

            let param_contract = ContractParam {
                symbol_id: param.symbol_id,
                ty: param_type.clone(),
                default: param_default,
                kind: param.kind.clone(),
            };

            // ugly? but it works, contract is just a cheap copy made to pass to callers either way]
            // TODO double check logic here
            contract.params.push(param_contract);

            state.bind(&symbol_ref, param_type);
        }

        // !! be mindful of the fact that there is whatever the user DECLARED, and what actually gets returned
        // this is the former.
        contract.declared_return_type = match &function.returns {
            Some(expr) => self.flow.type_resolver.parse_annotation(expr, program_id),
            None => Type::Unknown,
        };

        self
            .flow
            .analyze_body(
            program_id, 
            contour_id,
            &function.graph, 
            state
        )?;

        let returns = self.collect_function_returns(
            program_id,
            contour_id,
            function,
        )?;

        contract.returns = returns;

        Ok(contract)
    }

    fn collect_function_returns(
        &mut self, 
        program_id: i64,
        contour_id: ContourID,
        function: &FunctionCfg,
    ) -> Result<Vec<GuardedReturn>, BlockedFunctionAnalysis>{
        let mut returns = Vec::new();

        for (block_id, block) in &function.graph.blocks {
            let Some(Terminator::Return(ret)) = &block.terminator else {
                continue;
            };

            let key = FlowBlockID {
                contour: contour_id,
                block: *block_id,
            };

            let Some(return_state) = self.flow.block_out.get(&key) else {
                continue;
            };

            let mut return_state = return_state.clone();

            let return_type = match ret {
                Some(expr) => self
                    .flow
                    .type_resolver
                    .parse_expr(
                        expr,
                        program_id,
                        &mut return_state,
                    )
                    .map_err(|request| BlockedFunctionAnalysis {
                        program_id,
                        block_id: *block_id,
                        request,
                        contour: contour_id,
                    })?,

                None => Type::None,
            };

            returns.push(GuardedReturn {
                guard: return_state.guard.simplify(),
                ty: return_type,
                constraints: return_state
                    .constraints
                    .iter()
                    .map(|c| c.simplify())
                    .collect(),
            });
        }

        Ok(returns)
    }

    // TODO further improve, this is a skeleton
    fn analyze_class(
        &mut self,
        program_id: i64,
        class_id: ClassID,
        class: &ClassCfg,
        symbols: &[SymbolIR],
    ) -> Result<(), BlockedFunctionAnalysis> {
        let mut state = FlowState::new(BoolExpr::from_bool(true));

        let contour_id = ContourID::Class(class_id);

        for symbol in symbols {
            let symbol_ref = SymbolRef {
                program_id,
                symbol_id: symbol.id,
            };

            state.register_unbound(&symbol_ref);
        }

        self
            .flow
            .analyze_body(
            program_id, 
            contour_id,
            &class.graph, 
            state
        )?;

        Ok(())
    }
}
