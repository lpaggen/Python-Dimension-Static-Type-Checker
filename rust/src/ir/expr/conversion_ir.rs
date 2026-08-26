#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conversion {
    None,
    Str,
    Repr,
    Ascii,
}

impl TryFrom<i32> for Conversion {
    type Error = String;

    fn try_from(int: i32) -> Result<Self, Self::Error> {
        match int {
            -1 => Ok(Conversion::None),
            97 => Ok(Conversion::Ascii),
            114 => Ok(Conversion::Repr),
            115 => Ok(Conversion::Str),
            _ => Err(format!("invalid Conversion value: {}", int)),
        }
    }
}
