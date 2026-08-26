from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.ir_node import IRNode


@dataclass
class AnnotationHeadIR(IRNode):
    root: str
    attrs: list[str]
    scope_id: int
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.AnnotationHeadIR(
            root=self.root,
            attrs=self.attrs,
            scope_id=self.scope_id,
        )

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto
