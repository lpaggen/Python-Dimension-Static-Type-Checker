#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID {
    pub id: usize, // yes, we could just use usize directly, this is more clear and costs nothing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionID {
    pub id: usize, // yes, we could just use usize directly, this is more clear and costs nothing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassID {
    pub id: usize, // yes, we could just use usize directly, this is more clear and costs nothing
}
