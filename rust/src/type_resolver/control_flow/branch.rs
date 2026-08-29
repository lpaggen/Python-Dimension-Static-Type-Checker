use crate::{ir::expr::ExprIR, type_resolver::control_flow::block_id::BlockID};

#[derive(Debug, Clone)]
pub struct Branch<'a> {
    pub condition: &'a ExprIR,
    pub true_target: BlockID,
    pub false_target: BlockID,
}
