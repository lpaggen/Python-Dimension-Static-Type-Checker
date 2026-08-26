from common.span import SourceSpan
from ir.ir_node import IRNode
from generated import _pb2
from dataclasses import dataclass


@dataclass
class SymbolIR(IRNode):
    id: int
    name: str
    kind: object
    scope_id: int
    span: SourceSpan | None
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
