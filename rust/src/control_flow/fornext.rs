use crate::{control_flow::block_id::BlockID, ir::expr::ExprIR};

#[derive(Debug, Clone)]
pub struct Next<'a> {
    pub iterator: &'a ExprIR,
    pub target: &'a ExprIR,
    pub hasnext_target: BlockID,
    pub empty_target: BlockID,
}
