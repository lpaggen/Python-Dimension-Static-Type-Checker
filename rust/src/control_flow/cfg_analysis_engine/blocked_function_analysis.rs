use crate::control_flow::{block_id::BlockID, cfg_analysis_engine::contour_id::ContourID, function_analysis_request::FunctionAnalysisRequest};

// used to carry the block ID over to the parent AnalysisEngine
#[derive(Debug)]
pub struct BlockedFunctionAnalysis {
    pub program_id: usize,
    pub contour: ContourID,
    pub block_id: BlockID,
    pub request: FunctionAnalysisRequest,
}
