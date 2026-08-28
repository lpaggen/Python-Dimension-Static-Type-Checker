use crate::{ir::expr::ExprIR, type_resolver::control_flow::block_id::BlockID};

pub struct Branch {
    // pub condition: Box<ExprIR>,
    pub true_target: BlockID, 
    pub false_target: BlockID,
}
