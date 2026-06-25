from .ir_node import IRNode
from common.span import SourceSpan
from generated import _pb2


class SliceIR(IRNode):
    def __init__(self, lower: IRNode, upper: IRNode, step: IRNode, span: SourceSpan):
        super().__init__(span=span)
        self.lower = lower
        self.upper = upper
        self.step = span
        self.span = span

    def to_proto(self):
        proto = _pb2.SliceIR()

        if self.lower is not None:
            proto.lower.CopyFrom(self.lower.to_proto())

        if self.upper is not None:
            proto.upper.CopyFrom(self.upper.to_proto())

        if self.step is not None:
            proto.step.CopyFrom(self.step.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.slice.CopyFrom(proto)
        return expr
