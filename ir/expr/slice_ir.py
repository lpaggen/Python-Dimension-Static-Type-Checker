from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class SliceIR(ExprIR):
    lower: ExprIR | None
    upper: ExprIR | None
    step: ExprIR | None
    span: SourceSpan

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
        return _pb2.ExprIR(slice=proto)
