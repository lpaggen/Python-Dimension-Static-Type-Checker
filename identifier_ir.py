from span import SourceSpan


class IdentifierIR:
    def __init__(self, name: str, use_scope_id: int, span: SourceSpan):
        self.name=name,
        self.use_scope_id=use_scope_id,
        self.span=span
