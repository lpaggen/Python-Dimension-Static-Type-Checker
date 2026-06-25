from common.span import SourceSpan
from .ir_node import IRNode
from .identified_ir_node import IdentifiedIRNode
from generated import _pb2


class SymbolIR(IdentifiedIRNode):
    def __init__(self, id: int, name: str, kind: str, scope_id: int, span: SourceSpan):
        super().__init__(id=id, span=span)
        self.id = id
        self.name = name
        self.kind = kind
        self.scope_id = scope_id
        self.span = span

    def to_proto(self):
        proto = _pb2.SymbolIR(
            id=self.id,
            name=self.name,
            kind=self.kind.value if hasattr(self.kind, "value") else self.kind,
            scope_id=self.scope_id,
        )

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto
