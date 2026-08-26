use crate::{ir::expr::ExprIR, type_resolver::control_flow::{block_id::BlockID, branch::Branch}};

pub enum Terminator {
    Branch(Branch),  // if, while, for, (match)

    Goto(BlockID),   // break (exit loop), continue (back to top of loop), (match)

    Return(Option<ExprIR>),

    Raise(Option<ExprIR>)
}

impl Terminator {
    pub fn outgoing(&self) -> Vec<BlockID> {
        match self {
            Terminator::Branch(branch) => {
                vec![branch.true_target, branch.false_target]
            },

            Terminator::Goto(goto) => {
                vec![*goto]
            }

            Terminator::Raise(_) => {
                vec![]
            }

            Terminator::Return(_) => {
                vec![]
            }
        }
    }

    // pub fn next(&self) ->  todo!()
}
