#[derive(Debug)]
pub struct DimParseError {
    pub pos: usize,
    pub message: String,
}
