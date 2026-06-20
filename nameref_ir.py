@dataclass
class NameRefIR:
    name: str
    use_scope_id: int
    span: SourceSpan | None = None