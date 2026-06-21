from span import SourceSpan
from ir_node import IRNode


class SymbolIR(IRNode):
    def __init__(self, id: int, name: str, kind: str, scope_id: id, span: SourceSpan):
        self.id=id,
        self.name=name,
        self.kind=kind,
        self.scope_id=scope_id,
        self.span=span
