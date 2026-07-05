#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    FloorDiv,
    Mod,
    Pow,

    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,

    And,
    Or,
    Not,

    UAdd,
    USub,

    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,

    Unknown(i32),
}

impl From<i32> for Operator {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Sub,
            3 => Self::Mult,
            4 => Self::MatMult,
            5 => Self::Div,
            6 => Self::FloorDiv,
            7 => Self::Mod,
            8 => Self::Pow,

            9 => Self::LShift,
            10 => Self::RShift,
            11 => Self::BitOr,
            12 => Self::BitXor,
            13 => Self::BitAnd,

            14 => Self::And,
            15 => Self::Or,
            16 => Self::Not,

            17 => Self::UAdd,
            18 => Self::USub,

            19 => Self::Eq,
            20 => Self::NotEq,
            21 => Self::Lt,
            22 => Self::LtE,
            23 => Self::Gt,
            24 => Self::GtE,
            25 => Self::Is,
            26 => Self::IsNot,
            27 => Self::In,
            28 => Self::NotIn,

            other => Self::Unknown(other),
        }
    }
}

impl Operator {
    pub fn as_i32(self) -> i32 {
        match self {
            Self::Add => 1,
            Self::Sub => 2,
            Self::Mult => 3,
            Self::MatMult => 4,
            Self::Div => 5,
            Self::FloorDiv => 6,
            Self::Mod => 7,
            Self::Pow => 8,

            Self::LShift => 9,
            Self::RShift => 10,
            Self::BitOr => 11,
            Self::BitXor => 12,
            Self::BitAnd => 13,

            Self::And => 14,
            Self::Or => 15,
            Self::Not => 16,

            Self::UAdd => 17,
            Self::USub => 18,

            Self::Eq => 19,
            Self::NotEq => 20,
            Self::Lt => 21,
            Self::LtE => 22,
            Self::Gt => 23,
            Self::GtE => 24,
            Self::Is => 25,
            Self::IsNot => 26,
            Self::In => 27,
            Self::NotIn => 28,

            Self::Unknown(value) => value,
        }
    }
}
