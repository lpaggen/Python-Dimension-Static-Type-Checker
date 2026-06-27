from common.span import SourceSpan
from ir.identified_ir_node import IdentifiedIRNode
from generated import _pb2


class ScopeIR(IdentifiedIRNode):
    def __init__(self, id: int, name: str, kind: str, parent_id: int, span: SourceSpan):
        super().__init__(id=id, span=span)
        self.id = id
        self.name = name
        self.kind = kind
        self.parent_id = parent_id
        self.span = span

    def to_proto(self):
        proto = _pb2.ScopeIR(
            id=self.id,
            name=self.name,
            kind=self.kind.value if hasattr(self.kind, "value") else self.kind,
        )

        if self.parent_id is not None:
            proto.parent_id = self.parent_id

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto