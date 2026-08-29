use crate::control_flow::block_id::BlockID;

#[derive(Clone, Copy)]
pub struct LoopContext {
    // pub preheader: BlockID,
    pub header: BlockID,
    // pub body_entry: BlockID,
    pub exit: BlockID,
    // pub back_edges: Vec<BlockID>,
}

// commenting out stuff we need for Phi-Nodes, that's a much later objective
