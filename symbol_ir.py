from span import SourceSpan


@dataclass
class SymbolIR:
    id: int
    name: str
    kind: str
    scope_id: int
    span: SourceSpan | None = None
