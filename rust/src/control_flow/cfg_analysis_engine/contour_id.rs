use crate::control_flow::block_id::{ClassID, FunctionID};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContourID {
    Module(i64),
    Function(FunctionID),
    Class(ClassID),
}
