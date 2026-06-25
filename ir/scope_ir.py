from common.span import SourceSpan
from ir.expression_ir import IdentifiedIRNode


class ScopeIR(IdentifiedIRNode):
    def __init__(self, id: int, name: str, kind: str, parent_id: int, span: SourceSpan):
        super().__init__(id=id, span=span)
        self.id = id
        self.name = name
        self.kind = kind
        self.parent_id = parent_id
        self.span = span
