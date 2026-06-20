from span import SourceSpan


class IRNode:
    def __init__(self):
        pass

class SymbolIR(IRNode):
    def __init__(self, id: int, name: str, kind: str, scope_id: id, span: SourceSpan):
        self.id=id,
        self.name=name,
        self.kind=kind,
        self.scope_id=scope_id,
        self.span=span
