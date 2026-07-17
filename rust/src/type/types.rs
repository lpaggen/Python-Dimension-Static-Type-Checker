#[derive(Debug, Clone)]
pub enum Type {
    Tensor(TensorType),
    Int,
    Float,
    Bool,
    String,
    Tuple(Vec<Type>),
    Callable(CallableType),
    Module(ModuleType),
    Unknown, // may be a valid type, we just don't consider it in this tool
}

pub struct TensorType {
    pub shape: Vec<Dim>,
    pub dtype: Option<DType>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { // tensor inner type, necessary to check whether say, tensor A with dtype f16 is compatible with i32 tensor B
    Bool,

    Int8,
    Int16,
    Int32,
    Int64,

    UInt8,

    Float16,
    BFloat16,
    Float32,
    Float64,

    Complex32,
    Complex64,
    Complex128,

    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dim {
    Known(i64),
    Symbol(String),
    Unknown,
}