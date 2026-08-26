from common.span import SourceSpan
from ir.ir_node import IRNode
from generated import _pb2
from dataclasses import dataclass


@dataclass
class ScopeIR(IRNode):
    id: int
    name: str
    kind: object
    parent_id: int | None
    span: SourceSpan | None
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
