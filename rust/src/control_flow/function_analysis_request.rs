use crate::control_flow::{block_id::FunctionID, call_binding::CallBinding};

#[derive(Debug, Clone)]
pub struct FunctionAnalysisRequest {
    pub program_id: i64,
    pub function_id: FunctionID,
    pub bindings: Vec<CallBinding>,
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
