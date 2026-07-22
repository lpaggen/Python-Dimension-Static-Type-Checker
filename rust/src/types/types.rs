use crate::linker::symbol_ref::SymbolRef;

#[derive(Debug, Clone)]
pub enum Type {
    Tensor(TensorTypeState),  // annotation knows it's a tensor, but doesn't know dimensions or dtype
    Int,
    Float,
    Bool,
    String,
    None,
    Tuple(Vec<Type>),
    List(Vec<Type>),
    Callable(CallableType), // functions
    Class(ClassType),
    Dim(DimType),
    //Module(ModuleType),
    Unknown, // may be a valid type, we just don't consider it in this tool
}

#[derive(Debug, Clone)]
pub struct CallableType {
    pub params: Vec<Type>,
    pub return_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct ClassType {
    pub symbol: SymbolRef,
}

#[derive(Debug, Clone)]
pub enum TensorTypeState {
    Resolved(TensorType),
    Unresolved,
}

#[derive(Debug, Clone)]
pub struct TensorType {
    pub shape: Vec<DimType>,
    pub dtype: Option<DType>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { // tensor inner type, found in Numpy and Torch
    Bool,

    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,

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
pub enum DimType {
    Known(i64),
    Symbol(String),  // might become SymbolRef instead, easier to resolve
    Unknown,
}
