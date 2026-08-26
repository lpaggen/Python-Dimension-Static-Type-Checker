use crate::ir::expr_ir::ExprIR;
use crate::ir::span_ir::SourceSpan;

#[derive(Debug, Clone)]
pub enum PatternIR {
    ValuePattern(ValuePatternIR),
    SingletonPattern(SingletonPatternIR),
    SequencePattern(SequencePatternIR),
    MappingPattern(MappingPatternIR),
    ClassPattern(ClassPatternIR),
    StarPattern(StarPatternIR),
    CapturePattern(CapturePatternIR),
    WildcardPattern(WildcardPatternIR),
    AsPattern(AsPatternIR),
    OrPattern(OrPatternIR),
}

#[derive(Debug, Clone)]
pub struct ValuePatternIR {
    pub value: ExprIR,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct SingletonPatternIR {
    /// None  => Python `None`
    /// Some(true)  => Python `True`
    /// Some(false) => Python `False`
    pub value: Option<bool>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct SequencePatternIR {
    pub patterns: Vec<PatternIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct MappingPatternIR {
    pub keys: Vec<ExprIR>,
    pub patterns: Vec<PatternIR>,
    pub rest: Option<String>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct ClassPatternIR {
    pub cls: ExprIR,
    pub patterns: Vec<PatternIR>,
    pub kwd_attrs: Vec<String>,
    pub kwd_patterns: Vec<PatternIR>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct StarPatternIR {
    /// None corresponds to `*_`
    pub name: Option<String>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct CapturePatternIR {
    pub name: String,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct WildcardPatternIR {
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct AsPatternIR {
    pub pattern: Box<PatternIR>,
    pub name: String,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone)]
pub struct OrPatternIR {
    pub patterns: Vec<PatternIR>,
    pub span: Option<SourceSpan>,
}
