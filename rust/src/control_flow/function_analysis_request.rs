use std::{cell::RefCell, rc::Rc};

use crate::{
    control_flow::{block_id::FunctionID, call_binding::CallBinding, flowstate::FlowState}, ir::span_ir::SourceSpan,
};

#[derive(Debug, Clone)]
pub struct FunctionAnalysisRequest {
    pub program_id: usize,
    pub function_id: FunctionID,
    pub bindings: Vec<CallBinding>,
    /// The source call that caused this specialization to be analyzed.
    pub call_site: SourceSpan,
    pub parent_state: Rc<RefCell<FlowState>>,
}

// impl FunctionAnalysisRequest {
//     pub fn new(
//         program_id: i64,
//         function_id: FunctionID,
//         bindings: Vec<CallBinding>
//     ) -> Self {
//         Self {
//             program_id,
//             function_id,
//             bindings
//         }
//     }
// }
