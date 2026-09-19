use crate::control_flow::{block_id::BlockID, cfg_analysis_engine::contour_id::ContourID};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlowBlockID {
    pub contour: ContourID,
    pub block: BlockID,
}
