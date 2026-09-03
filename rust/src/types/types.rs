use std::collections::HashSet;

use crate::linker::symbol_ref::SymbolRef;

// to do rename all to *Type for clarity
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Type {
    Tensor(TensorTypeState), // annotation knows it's a tensor, but doesn't know dimensions or dtype
    Int,
    Float,
    Bool,
    Bytes,
    Complex,
    String,
    None,
    Tuple(Vec<Type>),
    List(Vec<Type>),
    Callable(CallableType), // functions
    Class(ClassType),
    Dim(DimType),
    Union(Vec<Type>), // represent if-else-then branches, where variable types depend on conditions
    //Module(ModuleType),
    Ellipsis,
    Unknown, // may be a valid type, we just don't consider it in this tool
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Type::Bool
                | Type::Int
                | Type::Float
                | Type::Complex
        )
    }

    pub fn merge(self, other: Type) -> Type {
        if self == other {
            return self;
        }

        match (self, other) {
            (Type::Union(mut left), Type::Union(right)) => {
                left.extend(right);
                Type::Union(left)
            }

            (Type::Union(mut items), other) => {
                items.push(other);
                Type::Union(items)
            }

            (other, Type::Union(mut items)) => {
                items.push(other);
                Type::Union(items)
            }

            (left, right) => {
                Type::Union(vec![left, right])
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct CallableType {
    pub params: Vec<Type>,
    pub return_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct ClassType {
    pub symbol: SymbolRef,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum TensorTypeState {
    Resolved(TensorType),
    Unresolved,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct TensorType {
    pub shape: Vec<DimType>,
    pub dtype: Option<DType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType {
    // tensor inner type, found in Numpy and Torch
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
    Symbol(String), // might become SymbolRef instead, easier to resolve
    Unknown,
}
