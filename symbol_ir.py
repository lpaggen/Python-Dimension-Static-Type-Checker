from span import SourceSpan
from ir_node import IRNode
from expression_ir import IdentifiedIRNode


class SymbolIR(IdentifiedIRNode):
    def __init__(self, id: int, name: str, kind: str, scope_id: int, span: SourceSpan):
        super().__init__(id=id, span=span)
        self.id = id
        self.name = name
        self.kind = kind
        self.scope_id = scope_id
        self.span = span
