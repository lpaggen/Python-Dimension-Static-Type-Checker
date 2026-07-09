#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Scalar(ScalarType),

    Tensor {
        dtype: ScalarType,
        shape: Shape,
    },

    Array(Box<Type>),
    Tuple(Vec<Type>),

    None,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarType {
    Int,
    Float,
    Bool,
    Str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dim {
    Known(usize),
    Symbol(String),
    Unknown,
}

pub type Shape = Vec<Dim>;