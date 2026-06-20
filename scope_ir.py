from span import SourceSpan


class ScopeIR:
    def __init__(self, id: int, name: str, kind: str, parent_id: int, span: SourceSpan):
        self.id=id,
        self.name=name,
        self.kind=kind,
        self.parent_id=parent_id,
        self.span=span
