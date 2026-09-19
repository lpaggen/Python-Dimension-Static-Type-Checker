use crate::{control_flow::block_id::BlockID, ir::expr::ExprIR};

#[derive(Debug, Clone)]
pub struct Branch<'a> {
    pub condition: &'a ExprIR,
    pub true_target: BlockID,
    pub false_target: BlockID,
}
