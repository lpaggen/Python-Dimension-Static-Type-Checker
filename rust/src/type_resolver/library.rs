#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Library {
    External(KnownLibrary),
    Std(StandardLibrary),
    User(UserDefinedLibrary),
}

pub struct ResolvedAttributePath {
    pub root: KnownLibrary,
    pub attrs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownLibrary {
    PyTorch,
    TensorFlow,
    Keras,
    NumPy,
    Jax,
}

impl KnownLibrary {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PyTorch => "torch",
            Self::TensorFlow => "tensorflow",
            Self::Keras => "keras",
            Self::NumPy => "numpy",
            Self::Jax => "jax",
        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "torch" => Some(Self::PyTorch),
            "tensorflow" => Some(Self::TensorFlow),
            "keras" => Some(Self::Keras),
            "numpy" => Some(Self::NumPy),
            "jax" => Some(Self::Jax),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardLibrary {
    Math,
}

impl StandardLibrary {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Math => "math",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserDefinedLibrary {
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownFunction {
    Torch(TorchFunction),
    NumPy(NumPyFunction),
    Jax(JaxFunction),
    Math(MathFunction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TorchFunction {
    Tensor,
    Zeros,
    Ones,
    Empty,
    Arange,
    Reshape,
    Cat,
    Stack,
    Matmul,
    Relu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumPyFunction {
    Array,
    Zeros,
    Ones,
    Empty,
    Arange,
    Reshape,
    Concatenate,
    Stack,
    Matmul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JaxFunction {
    Array,
    Zeros,
    Ones,
    Empty,
    Arange,
    Reshape,
    Concatenate,
    Stack,
    Matmul,
    Relu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MathFunction {
    Sqrt,
    Sin,
    Cos,
    Floor,
    Ceil,
}
