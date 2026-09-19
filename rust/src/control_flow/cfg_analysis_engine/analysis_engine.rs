// use std::{cell::RefCell, rc::Rc};

// use crate::{control_flow::{blockflow::BlockFlow, cfg_table::CfgTable, function_analysis_request::FunctionAnalysisRequest}, linker::program_table::ProgramTable};

// pub struct AnalysisEngine<'ctx> {
//     pub flow: BlockFlow<'ctx>,
//     pub contracts: Rc<RefCell<FunctionContractTable>>,
//     pub requests: Rc<RefCell<Vec<FunctionAnalysisRequest>>>,
// }

// impl<'ctx> AnalysisEngine<'ctx> {
//     pub fn run(&mut self, cfg: &CfgTable, programs: &ProgramTable) {
//         // drive module/function analysis

//         // if flow returns error, we just need to re-analyze the block with the proper request
//         // loop {
//         //     self.flow.run_some_analysis();

//         //     let requests = self.take_pending_requests();

//         //     if requests.is_empty() {
//         //         break;
//         //     }

//         //     for request in requests {
//         //         self.analyze_specialization(request, cfg, programs);
//         //     }
//         // }
//     }
// }
